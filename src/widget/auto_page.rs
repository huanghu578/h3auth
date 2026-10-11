//! A page container with a stored layout closure that re-runs on resize.

use std::cell::RefCell;
use std::rc::Rc;

use fltk::{group::Group, prelude::*};

use crate::core::rect::Rect;

pub struct AutoPage {
    group: Group,
    layout_fn: Rc<RefCell<Option<Box<dyn FnMut(Rect)>>>>,
    content_top: i32,
    pad: i32,
}

impl AutoPage {
    pub const DEFAULT_CONTENT_TOP: i32 = 30;
    pub const DEFAULT_PAD: i32 = 10;

    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let group = Group::new(x, y, w, h, label);
        Self {
            group,
            layout_fn: Rc::new(RefCell::new(None)),
            content_top: Self::DEFAULT_CONTENT_TOP,
            pad: Self::DEFAULT_PAD,
        }
    }

    pub fn finish(&mut self) {
        self.group.end();
    }

    pub fn group(&self) -> &Group {
        &self.group
    }

    pub fn group_handle(&self) -> Group {
        self.group.clone()
    }

    pub fn set_content_top(&mut self, top: i32) {
        self.content_top = top;
    }

    pub fn set_pad(&mut self, pad: i32) {
        self.pad = pad;
    }

    /// Register a layout closure. It runs on every `relayout`.
    ///
    /// Accepts `FnMut` so the closure may mutate captured widget handles.
    pub fn on_layout<F: FnMut(Rect) + 'static>(&mut self, f: F) {
        *self.layout_fn.borrow_mut() = Some(Box::new(f));
    }

    /// Resize the page to `(win_w, page_h)` and run the layout closure.
    pub fn relayout(&self, win_w: i32, page_h: i32) {
        let mut g = self.group.clone();
        g.set_size(win_w, page_h);

        let content = Rect::new(
            g.x() + self.pad,
            g.y() + self.content_top,
            (g.w() - self.pad * 2).max(0),
            (g.h() - self.content_top - self.pad).max(0),
        );

        if let Some(f) = self.layout_fn.borrow_mut().as_mut() {
            f(content);
        }
    }
}