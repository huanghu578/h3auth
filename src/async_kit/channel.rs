//! Channel from a worker thread to the FLTK UI thread.
//!
//! Wraps the standard `mpsc` + `add_timeout3` + `try_recv` pattern,
//! including panic protection and automatic shutdown when senders drop.

use std::cell::RefCell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, SendError, Sender as MpscSender, TryRecvError};

use fltk::app;

pub struct Channel;

impl Channel {
    /// Create a channel and start polling. Returns the sender.
    pub fn spawn_poll<T, F>(interval_sec: f64, on_msg: F) -> Sender<T>
    where
        T: 'static,
        F: FnMut(T) + 'static,
    {
        Self::spawn_poll_with_handle(interval_sec, on_msg).0
    }

    /// Like `spawn_poll` but also returns a handle for manual stop.
    pub fn spawn_poll_with_handle<T, F>(
        interval_sec: f64,
        on_msg: F,
    ) -> (Sender<T>, AsyncHandle)
    where
        T: 'static,
        F: FnMut(T) + 'static,
    {
        let (tx, rx) = mpsc::channel::<T>();
        let running = Rc::new(RefCell::new(true));
        let rx = Rc::new(RefCell::new(rx));
        let on_msg = Rc::new(RefCell::new(on_msg));

        start_poll(rx, running.clone(), on_msg, interval_sec);

        (Sender { inner: tx }, AsyncHandle { running })
    }
}

/// The sending half of a channel. Cloneable.
pub struct Sender<T> {
    inner: MpscSender<T>,
}

impl<T> Sender<T> {
    /// Send a message. Returns the message back on failure.
    pub fn send(&self, msg: T) -> Result<(), T> {
        self.inner.send(msg).map_err(|SendError(m)| m)
    }

    /// Send a message, printing a warning on failure.
    pub fn send_or_warn(&self, msg: T) {
        if self.inner.send(msg).is_err() {
            eprintln!("[fltk-kit::channel] receiver closed, message dropped");
        }
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

/// Handle to stop a polling loop manually.
pub struct AsyncHandle {
    running: Rc<RefCell<bool>>,
}

impl AsyncHandle {
    /// Stop the polling loop.
    pub fn stop(&self) {
        *self.running.borrow_mut() = false;
    }

    /// Whether the loop is still running.
    pub fn is_running(&self) -> bool {
        *self.running.borrow()
    }
}

fn start_poll<T, F>(
    rx: Rc<RefCell<Receiver<T>>>,
    running: Rc<RefCell<bool>>,
    on_msg: Rc<RefCell<F>>,
    interval_sec: f64,
) where
    T: 'static,
    F: FnMut(T) + 'static,
{
    fn poll<T, F>(
        rx: Rc<RefCell<Receiver<T>>>,
        running: Rc<RefCell<bool>>,
        on_msg: Rc<RefCell<F>>,
        interval_sec: f64,
    ) where
        T: 'static,
        F: FnMut(T) + 'static,
    {
        if !*running.borrow() {
            return;
        }

        let mut disconnected = false;

        loop {
            let msg = {
                let rx_ref = rx.borrow();
                match rx_ref.try_recv() {
                    Ok(m) => m,
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        disconnected = true;
                        break;
                    }
                }
            };

            let cb = on_msg.clone();
            let _ = catch_unwind(AssertUnwindSafe(|| {
                (cb.borrow_mut())(msg);
            }));
        }

        if disconnected || !*running.borrow() {
            *running.borrow_mut() = false;
            return;
        }

        // Use add_timeout3 to avoid the deprecation warning.
        // Its closure signature is FnMut(*mut ()). We ignore the handle.
        let rx_c = rx.clone();
        let running_c = running.clone();
        let on_msg_c = on_msg.clone();
        app::add_timeout3(interval_sec, move |_handle| {
            poll(
                rx_c.clone(),
                running_c.clone(),
                on_msg_c.clone(),
                interval_sec,
            );
        });
    }

    poll(rx, running, on_msg, interval_sec);
}