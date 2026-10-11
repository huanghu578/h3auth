//! Window creation and positioning helpers.

use fltk::{prelude::*, window::Window};

pub struct Windows;

impl Windows {
    /// Create a resizable window, show it, and maximize it.
    ///
    /// The call order is important: `make_resizable` → `end` → `show` → `maximize`.
    pub fn create_maximized(w: i32, h: i32, title: &str) -> Window {
        let mut wind = Window::default().with_size(w, h).with_label(title);
        wind.make_resizable(true);
        wind.end();
        wind.show();
        wind.maximize();
        wind
    }

    /// Show and maximize an existing window.
    pub fn show_maximized(wind: &mut Window) {
        wind.make_resizable(true);
        wind.end();
        wind.show();
        wind.maximize();
    }

    /// Set a minimum window size.
    pub fn set_min_size(wind: &mut Window, w: i32, h: i32) {
        // size_range(min_w, min_h, max_w, max_h)
        wind.size_range(w, h, 0, 0);
    }
}