//! Centered dialogs and file choosers.
//!
//! All message and input dialogs are centered on the given parent window
//! (not on the screen). Native file choosers are handled by the operating
//! system and are already centered on the parent.

use fltk::{
    dialog::{self, NativeFileChooser, NativeFileChooserType},
    prelude::*,
    window::Window,
};
use std::path::PathBuf;

pub struct Dialogs;

impl Dialogs {
    pub const MSG_W: i32 = 360;
    pub const MSG_H: i32 = 120;
    pub const CHOICE_W: i32 = 400;
    pub const CHOICE_H: i32 = 150;

    // ---- message boxes ----

    pub fn message_center(win: &Window, msg: &str) {
        let (x, y) = Self::centered(win, Self::MSG_W, Self::MSG_H);
        dialog::message(x, y, msg);
    }

    pub fn info_center(win: &Window, msg: &str) {
        Self::message_center(win, msg);
    }

    pub fn success_center(win: &Window, msg: &str) {
        Self::message_center(win, msg);
    }

    pub fn warning_center(win: &Window, msg: &str) {
        let (x, y) = Self::centered(win, Self::MSG_W, Self::MSG_H);
        dialog::alert(x, y, msg);
    }

    pub fn error_center(win: &Window, msg: &str) {
        Self::warning_center(win, msg);
    }

    // ---- confirmations ----

    pub fn confirm_center(win: &Window, msg: &str) -> bool {
        let (x, y) = Self::centered(win, Self::CHOICE_W, Self::CHOICE_H);
        dialog::choice2(x, y, msg, "Cancel", "OK", "") == Some(1)
    }

    pub fn ask_center(win: &Window, msg: &str) -> bool {
        let (x, y) = Self::centered(win, Self::CHOICE_W, Self::CHOICE_H);
        dialog::choice2(x, y, msg, "No", "Yes", "") == Some(1)
    }

    pub fn choice2_center(
        win: &Window,
        msg: &str,
        b0: &str,
        b1: &str,
        b2: &str,
    ) -> Option<i32> {
        let (x, y) = Self::centered(win, Self::CHOICE_W, Self::CHOICE_H);
        dialog::choice2(x, y, msg, b0, b1, b2)
    }

    // ---- inputs ----

    pub fn input_center(win: &Window, msg: &str, default: &str) -> Option<String> {
        let (x, y) = Self::centered(win, Self::CHOICE_W, Self::CHOICE_H);
        dialog::input(x, y, msg, default)
    }

    pub fn password_center(win: &Window, msg: &str, default: &str) -> Option<String> {
        let (x, y) = Self::centered(win, Self::CHOICE_W, Self::CHOICE_H);
        dialog::password(x, y, msg, default)
    }

    // ---- file choosers ----
    //
    // Native file dialogs are shown by the OS. They are already centered
    // on the parent window, so no explicit position is needed.

    pub fn file_open_center(title: &str, filter: Option<&str>) -> Option<PathBuf> {
        let mut c = NativeFileChooser::new(NativeFileChooserType::BrowseFile);
        c.set_title(title);
        if let Some(f) = filter {
            c.set_filter(f);
        }
        c.show();
        let p = c.filename();
        if p.as_os_str().is_empty() {
            None
        } else {
            Some(p)
        }
    }

    pub fn file_save_center(title: &str, filter: Option<&str>) -> Option<PathBuf> {
        let mut c = NativeFileChooser::new(NativeFileChooserType::BrowseSaveFile);
        c.set_title(title);
        if let Some(f) = filter {
            c.set_filter(f);
        }
        c.show();
        let p = c.filename();
        if p.as_os_str().is_empty() {
            None
        } else {
            Some(p)
        }
    }

    pub fn dir_center(title: &str) -> Option<PathBuf> {
        let mut c = NativeFileChooser::new(NativeFileChooserType::BrowseDir);
        c.set_title(title);
        c.show();
        let p = c.filename();
        if p.as_os_str().is_empty() {
            None
        } else {
            Some(p)
        }
    }

    // ---- internal ----

    /// Compute the top-left corner of a dialog of size `dlg_w * dlg_h`
    /// so that it is centered on `win`'s screen rectangle.
    fn centered(win: &Window, dlg_w: i32, dlg_h: i32) -> (i32, i32) {
        let x = win.x_root() + (win.w() - dlg_w) / 2;
        let y = win.y_root() + (win.h() - dlg_h) / 2;
        (x.max(0), y.max(0))
    }
}