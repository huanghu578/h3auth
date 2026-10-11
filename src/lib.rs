//! fltk-kit: reusable utilities for fltk-rs applications.
//!
//! Organized in three layers:
//! - [`core`]: pure logic, no FLTK dependency
//! - [`widget`]: FLTK widget helpers
//! - [`async_kit`]: channels and background tasks

pub mod core;
pub mod widget;
pub mod async_kit;

pub use crate::core::rect::Rect;
pub use crate::core::theme::Theme;
pub use crate::core::text::Text;
pub use crate::core::paths::Paths;

pub use crate::widget::fonts::{Fonts, FontSetup, FontSource};
pub use crate::widget::icon::Icon;
pub use crate::widget::dialogs::Dialogs;
pub use crate::widget::auto_page::AutoPage;
pub use crate::widget::tree::TreeHelper;
pub use crate::widget::table::TableHelper;
pub use crate::widget::windows::Windows;
pub use crate::widget::shortcuts::{Shortcuts, ShortcutManager};
pub use crate::widget::layout_ext::RectExt;

pub use crate::async_kit::channel::{Channel, Sender, AsyncHandle};
pub use crate::async_kit::task::{Task, CancelToken};