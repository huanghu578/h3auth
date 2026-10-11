//! Background tasks: spawn a worker thread and deliver the result to the UI thread.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use super::channel::Channel;

pub struct Task;

impl Task {
    /// Run `work` on a worker thread, then call `on_done` on the UI thread.
    pub fn run<W, T, F>(interval_sec: f64, work: W, on_done: F)
    where
        W: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
        F: FnMut(T) + 'static,
    {
        let tx = Channel::spawn_poll(interval_sec, on_done);
        thread::spawn(move || {
            let result = work();
            tx.send_or_warn(result);
        });
    }

    /// Like `run` but returns a `CancelToken` that can stop the work.
    /// The work closure receives the token and should check it periodically.
    pub fn run_cancellable<W, T, F>(
        interval_sec: f64,
        work: W,
        on_done: F,
    ) -> CancelToken
    where
        W: FnOnce(&CancelToken) -> T + Send + 'static,
        T: Send + 'static,
        F: FnMut(T) + 'static,
    {
        let token = CancelToken::new();
        let token_for_work = token.clone();

        let tx = Channel::spawn_poll(interval_sec, on_done);
        thread::spawn(move || {
            let result = work(&token_for_work);
            tx.send_or_warn(result);
        });

        token
    }
}

/// A cancellation flag shared with a worker thread.
#[derive(Clone)]
pub struct CancelToken {
    cancelled: Arc<AtomicBool>,
}

impl CancelToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Request cancellation.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// Whether cancellation was requested.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

impl Default for CancelToken {
    fn default() -> Self {
        Self::new()
    }
}