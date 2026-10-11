//! Extensions for `Rect` that depend on FLTK.
//!
//! The core `Rect` type is FLTK-independent. These helpers bridge it to
//! FLTK widgets.

use fltk::prelude::*;

use crate::core::rect::Rect;

/// Extension trait that adds FLTK-aware methods to `Rect`.
pub trait RectExt {
    /// Move and resize a widget to match this rectangle.
    fn apply_to<W: WidgetExt>(&self, widget: &mut W);

    /// Apply a list of rectangles to a list of widgets, in order.
    fn apply_all<W: WidgetExt>(rects: &[Rect], widgets: &mut [&mut W]);
}

impl RectExt for Rect {
    fn apply_to<W: WidgetExt>(&self, widget: &mut W) {
        widget.set_pos(self.x, self.y);
        widget.set_size(self.w, self.h);
    }

    fn apply_all<W: WidgetExt>(rects: &[Rect], widgets: &mut [&mut W]) {
        for (r, w) in rects.iter().zip(widgets.iter_mut()) {
            r.apply_to(*w);
        }
    }
}