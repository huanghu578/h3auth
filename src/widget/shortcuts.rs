//! Shortcut constants and a global shortcut manager.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use fltk::{
    app,
    enums::{Event, Key, Shortcut},
    prelude::*,
    window::Window,
};

pub struct Shortcuts;

impl Shortcuts {
    // Function keys
    pub fn f1() -> Shortcut { Shortcut::from_key(Key::F1) }
    pub fn f2() -> Shortcut { Shortcut::from_key(Key::F2) }
    pub fn f3() -> Shortcut { Shortcut::from_key(Key::F3) }
    pub fn f4() -> Shortcut { Shortcut::from_key(Key::F4) }
    pub fn f5() -> Shortcut { Shortcut::from_key(Key::F5) }
    pub fn f6() -> Shortcut { Shortcut::from_key(Key::F6) }
    pub fn f7() -> Shortcut { Shortcut::from_key(Key::F7) }
    pub fn f8() -> Shortcut { Shortcut::from_key(Key::F8) }
    pub fn f9() -> Shortcut { Shortcut::from_key(Key::F9) }
    pub fn f10() -> Shortcut { Shortcut::from_key(Key::F10) }
    pub fn f11() -> Shortcut { Shortcut::from_key(Key::F11) }
    pub fn f12() -> Shortcut { Shortcut::from_key(Key::F12) }

    // Common combinations
    pub fn ctrl_a() -> Shortcut { Self::ctrl('a') }
    pub fn ctrl_c() -> Shortcut { Self::ctrl('c') }
    pub fn ctrl_f() -> Shortcut { Self::ctrl('f') }
    pub fn ctrl_n() -> Shortcut { Self::ctrl('n') }
    pub fn ctrl_o() -> Shortcut { Self::ctrl('o') }
    pub fn ctrl_p() -> Shortcut { Self::ctrl('p') }
    pub fn ctrl_s() -> Shortcut { Self::ctrl('s') }
    pub fn ctrl_v() -> Shortcut { Self::ctrl('v') }
    pub fn ctrl_x() -> Shortcut { Self::ctrl('x') }
    pub fn ctrl_y() -> Shortcut { Self::ctrl('y') }
    pub fn ctrl_z() -> Shortcut { Self::ctrl('z') }

    pub fn ctrl_shift_s() -> Shortcut { Self::ctrl_shift('s') }
    pub fn ctrl_shift_z() -> Shortcut { Self::ctrl_shift('z') }

    // Special keys
    pub fn esc() -> Shortcut { Shortcut::from_key(Key::Escape) }
    pub fn enter() -> Shortcut { Shortcut::from_key(Key::Enter) }
    pub fn delete() -> Shortcut { Shortcut::from_key(Key::Delete) }
    pub fn backspace() -> Shortcut { Shortcut::from_key(Key::BackSpace) }

    pub fn func(key: Key) -> Shortcut {
        Shortcut::from_key(key)
    }

    pub fn ctrl(ch: char) -> Shortcut {
        Shortcut::Ctrl | Shortcut::from_key(Key::from_char(ch))
    }

    pub fn ctrl_shift(ch: char) -> Shortcut {
        Shortcut::Ctrl | Shortcut::Shift | Shortcut::from_key(Key::from_char(ch))
    }

    pub fn alt(key: Key) -> Shortcut {
        Shortcut::Alt | Shortcut::from_key(key)
    }

    /// Bind a shortcut to a button-like widget (acts as a click when pressed).
    pub fn bind<T: WidgetExt + ButtonExt>(widget: &mut T, sc: Shortcut) {
        widget.set_shortcut(sc);
    }

    pub fn manager() -> ShortcutManager {
        ShortcutManager::new()
    }
}

pub struct ShortcutManager {
    handlers: Rc<RefCell<HashMap<u64, Box<dyn FnMut()>>>>,
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self {
            handlers: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn register<F>(&mut self, sc: Shortcut, f: F)
    where
        F: FnMut() + 'static,
    {
        self.handlers
            .borrow_mut()
            .insert(sc.bits() as u64, Box::new(f));
    }

    pub fn unregister(&mut self, sc: Shortcut) {
        self.handlers.borrow_mut().remove(&(sc.bits() as u64));
    }

    pub fn clear(&mut self) {
        self.handlers.borrow_mut().clear();
    }

    pub fn attach(&self, wind: &mut Window) {
        let handlers = self.handlers.clone();

        wind.handle(move |_w, ev| {
            if ev == Event::KeyUp {
                let key = app::event_key();
                let state = app::event_state();
                let current = state | Shortcut::from_key(key);

                if let Some(cb) = handlers.borrow_mut().get_mut(&(current.bits() as u64)) {
                    cb();
                    return true;
                }
            }
            false
        });
    }
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}