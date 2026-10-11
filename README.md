# fltk-kit

A collection of reusable utilities for [fltk-rs](https://github.com/fltk-rs/fltk-rs) applications.

`fltk-kit` provides the boilerplate that almost every FLTK project ends up writing by hand:
geometry helpers, font selection, dialog wrappers, icon loading, background tasks,
and an auto-layout page type.

## Features

- **`Rect`** — geometry primitives with `cut_*`, `split_*`, `buttons_row`, `form_row`.
- **`Theme`** — shared spacing, size, and font constants.
- **`Text` / `Paths`** — small string and path helpers.
- **`Fonts`** — three-tier font selection (user → system language → fallback).
- **`Icon`** — window icons and images from embedded bytes, Base64, or files.
- **`Dialogs`** — centered message, confirm, input, password, and file dialogs.
- **`AutoPage`** — a page that stores a layout closure and re-lays out on resize.
- **`TreeHelper` / `TableHelper`** — common Tree and SmartTable operations.
- **`Windows` / `Shortcuts`** — window creation, global and per-widget shortcuts.
- **`Channel` / `Task` / `CancelToken`** — background work with UI thread updates.

## Installation

```toml
[dependencies]
fltk-kit = "0.1"
```

## Quick start

```rust
use fltk::{app, prelude::*, window::Window};
use fltk_kit::{Fonts, Icon, Windows};

fn main() {
    // Three-tier font selection
    Fonts::setup_by_language("zh-CN", 14);

    let app = app::App::default();

    // Create window, show maximized, set icon
    let mut wind = Windows::create_maximized(800, 600, "My App");
    Icon::set_embd_png(&mut wind, include_bytes!("assets/app.png"));

    // ...build UI...

    app.run().unwrap();
}
```

## Layout with `Rect`

`Rect` is the primary layout primitive. All operations are pure functions that
return new rectangles, so layouts compose naturally.

```rust
use fltk_kit::{Rect, Theme};

// Page content area
let content = Rect::content(win_w, win_h, Theme::PAGE_TOP, 60, Theme::PAD_MD);

// Split off the bottom button strip
let (content, btn_area) = content.cut_bottom(60);

// Left tree (fixed) + right table (fills the rest)
let (table_area, tree_area) = content.cut_left(250, Theme::PAD_MD);

tree_area.apply_to(&mut tree);
table_area.apply_to(&mut table);

// Bottom buttons, vertically and horizontally centered
let buttons = btn_area.buttons_row(2, 280, 30, 20);
buttons[0].apply_to(&mut btn_pick);
buttons[1].apply_to(&mut btn_export);
```

## Auto-layout pages

`AutoPage` wraps a `Group` and a layout closure. Call `relayout` on window resize
and every page re-arranges itself.

```rust
use fltk_kit::{AutoPage, Rect};

let mut page = AutoPage::new(0, 30, 800, 570, "Table");
page.group().begin();

let tree = Tree::new(0, 0, 0, 0, "");
let table = SmartTable::new(0, 0, 0, 0, "");
page.group().end();

page.on_layout({
    let (mut tree_c, mut table_c) = (tree.clone(), table.clone());
    move |content| {
        let (table_area, tree_area) = content.cut_left(250, 10);
        tree_area.apply_to(&mut tree_c);
        table_area.apply_to(&mut table_c);
    }
});

// On window resize:
page.relayout(win_w, win_h - 30);
```

## Background work

FLTK is single-threaded: all UI updates must happen on the main thread. `fltk-kit`
provides two levels of abstraction for moving values from a worker thread back
to the UI:

- **`Channel`** — low-level: create a channel, spawn a worker, poll on the UI thread.
- **`Task`** — high-level: run a closure on a worker thread and deliver the
  result to a UI callback. Supports cancellation.

### `Channel` — low-level

`Channel::spawn_poll` creates an `mpsc` channel, starts a polling loop on the
UI thread, and returns a `Sender`. Send values from any worker thread; the
callback runs on the UI thread.

```rust
use fltk_kit::Channel;

let tx = Channel::spawn_poll(0.1, move |records: Vec<Record>| {
    ui.borrow_mut().show_records(&records);
});

std::thread::spawn(move || {
    let records = load_records();
    tx.send_or_warn(records);
});
```

- Polling stops automatically when all `Sender` clones are dropped.
- `send()` returns `Result<(), T>` — the message is returned on failure.
- `send_or_warn()` prints a warning and drops the message.
- Use `spawn_poll_with_handle` if you need an `AsyncHandle` to stop early.

### `Task` — high-level

`Task::run` combines "spawn a worker thread" and "deliver the result to the UI
thread" into one call.

```rust
use fltk_kit::Task;

Task::run(
    0.1,                                         // polling interval (seconds)
    || load_records(),                           // worker thread
    move |records| {                             // UI thread callback
        ui.borrow_mut().show_records(&records);
    },
);
```

`Task::run` returns immediately. The worker closure runs on a new thread; when
it finishes, the result is passed to the UI callback on the main thread.

### Cancellation

`Task::run_cancellable` returns a `CancelToken` that the worker can check
periodically. Call `token.cancel()` from the UI thread to request cancellation.

```rust
use fltk_kit::{Task, CancelToken};

let token = Task::run_cancellable(
    0.1,
    |cancel: &CancelToken| {
        let mut out = Vec::new();
        for i in 0..1000 {
            if cancel.is_cancelled() {
                break;
            }
            out.push(heavy_step(i));
        }
        out
    },
    move |out| ui.borrow_mut().show_records(&out),
);

// Later, on the UI thread (e.g. from a "Cancel" button):
token.cancel();
```

The worker only stops when it *checks* `is_cancelled()`. For long loops, check
it every iteration or every N iterations. For blocking I/O, the check has to
happen between I/O calls.

### `Channel` vs `Task`

| Use case | API |
|---|---|
| Simple "run and forget" computation | `Task::run` |
| Long computation the user may cancel | `Task::run_cancellable` |
| Streaming multiple values over time | `Channel::spawn_poll` |
| Multiple producers, one consumer | `Channel::spawn_poll` (clone the `Sender`) |
| Fine-grained control (start/stop) | `Channel::spawn_poll_with_handle` |

## Font selection

`Fonts::setup_auto` follows a three-tier priority:

1. User-provided candidates
2. Operating-system language
3. Fallback list

```rust
use fltk_kit::{Fonts, FontSource};

let result = Fonts::setup_auto(
    Some(&["Source Han Sans"]),   // user preference
    Some("zh-CN"),                // system language
    14,
);

match result.source {
    FontSource::User => println!("Using user font: {}", result.font_name.unwrap()),
    FontSource::System => println!("Using system font: {}", result.font_name.unwrap()),
    FontSource::Fallback => println!("Using fallback font"),
    FontSource::Default => eprintln!("Keeping FLTK default font"),
}
```

## Dialogs

All message and input dialogs are centered on the **parent window** (not the
screen). Pass a reference to the parent `Window`.

```rust
use fltk_kit::Dialogs;

Dialogs::info_center(&win, "Operation completed.");
Dialogs::success_center(&win, "Export finished.");
Dialogs::warning_center(&win, "File already exists.");
Dialogs::error_center(&win, "Save failed.");

if Dialogs::confirm_center(&win, "Delete this file?") {
    // user pressed OK
}

if Dialogs::ask_center(&win, "Overwrite existing file?") {
    // user pressed Yes
}

if let Some(name) = Dialogs::input_center(&win, "Booklet name", "") {
    println!("Input: {}", name);
}

if let Some(path) = Dialogs::file_open_center("Open spreadsheet", Some("Excel\t*.xlsx")) {
    println!("Selected: {:?}", path);
}

if let Some(dir) = Dialogs::dir_center("Choose working folder") {
    println!("Folder: {:?}", dir);
}
```

Native file choosers (`file_open_center`, `file_save_center`, `dir_center`) are
shown by the operating system and are already centered by it, so they do not
take a window reference.

## Window icons

`Icon` handles PNG, ICO, Base64, and raw bytes. All setters return `bool` and
never panic.

```rust
use fltk_kit::Icon;

// Embedded PNG
Icon::set_embd_png(&mut wind, include_bytes!("assets/app.png"));

// Embedded ICO
Icon::set_embd_ico(&mut wind, include_bytes!("assets/app.ico"));

// Base64 (PNG content)
Icon::set_base64_png(&mut wind, ICON_B64);

// Show an image in a Frame, scaled to fit
Icon::show_base64_in_frame(&mut frame, cover_b64);
```

## Shortcuts

Predefined constructors and a global shortcut manager.

```rust
use fltk_kit::Shortcuts;

// Per-widget shortcut
Shortcuts::bind(&mut btn_save, Shortcuts::ctrl_s());

// Global shortcuts (F1 for help, F5 for refresh)
let mut mgr = Shortcuts::manager();
mgr.register(Shortcuts::f1(), move || show_help());
mgr.register(Shortcuts::f5(), move || refresh());
mgr.attach(&mut wind);
```

## Module map

| Layer | Module | Contents |
|---|---|---|
| core | `rect` | `Rect` geometry |
| core | `theme` | `Theme` constants |
| core | `text` | `Text` string helpers |
| core | `paths` | `Paths` path helpers |
| widget | `fonts` | `Fonts`, `FontSetup`, `FontSource` |
| widget | `icon` | `Icon` |
| widget | `dialogs` | `Dialogs` |
| widget | `auto_page` | `AutoPage` |
| widget | `tree` | `TreeHelper` |
| widget | `table` | `TableHelper` |
| widget | `windows` | `Windows` |
| widget | `shortcuts` | `Shortcuts`, `ShortcutManager` |
| async | `channel` | `Channel`, `Sender`, `AsyncHandle` |
| async | `task` | `Task`, `CancelToken` |

## Testing

```bash
cargo test                # runs pure-logic tests
cargo test -- --ignored   # runs the FLTK event-loop test (needs a display)
```

Tests cover `Rect` geometry, `Text` helpers, `Paths` utilities, and channel
behavior. GUI-dependent tests are marked `#[ignore]` and require a display.

## License

Dual-licensed under MIT or Apache-2.0, at your option.