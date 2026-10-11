//! Rectangle geometry primitives.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn window(win_w: i32, win_h: i32) -> Self {
        Self::new(0, 0, win_w, win_h)
    }

    /// Content area: horizontal padding `pad`, top offset `top`,
    /// bottom reserved `bottom_h`.
    pub fn content(win_w: i32, win_h: i32, top: i32, bottom_h: i32, pad: i32) -> Self {
        Self {
            x: pad,
            y: top,
            w: (win_w - pad * 2).max(0),
            h: (win_h - top - bottom_h).max(0),
        }
    }

    pub fn x2(&self) -> i32 { self.x + self.w }
    pub fn y2(&self) -> i32 { self.y + self.h }
    pub fn center_x(&self) -> i32 { self.x + self.w / 2 }
    pub fn center_y(&self) -> i32 { self.y + self.h / 2 }

    /// Shrink by `pad` on all sides.
    pub fn inset(self, pad: i32) -> Self {
        Self {
            x: self.x + pad,
            y: self.y + pad,
            w: (self.w - pad * 2).max(0),
            h: (self.h - pad * 2).max(0),
        }
    }

    /// Translate by `(dx, dy)`.
    pub fn shift(self, dx: i32, dy: i32) -> Self {
        Self { x: self.x + dx, y: self.y + dy, ..self }
    }

    /// Change width and height.
    pub fn resize(self, w: i32, h: i32) -> Self {
        Self { w, h, ..self }
    }

    // ---- cuts: return (remaining, cut) ----

    /// Cut `h` from the top.
    pub fn cut_top(self, h: i32) -> (Self, Self) {
        let h = h.clamp(0, self.h);
        let cut = Self::new(self.x, self.y, self.w, h);
        let rest = Self::new(self.x, self.y + h, self.w, self.h - h);
        (rest, cut)
    }

    /// Cut `h` from the bottom.
    pub fn cut_bottom(self, h: i32) -> (Self, Self) {
        let h = h.clamp(0, self.h);
        let rest = Self::new(self.x, self.y, self.w, self.h - h);
        let cut = Self::new(self.x, self.y + self.h - h, self.w, h);
        (rest, cut)
    }

    /// Cut `w` from the left, leaving `gap` before the remainder.
    pub fn cut_left(self, w: i32, gap: i32) -> (Self, Self) {
        let w = w.clamp(0, self.w);
        let cut = Self::new(self.x, self.y, w, self.h);
        let rest_w = (self.w - w - gap).max(0);
        let rest = Self::new(self.x + w + gap, self.y, rest_w, self.h);
        (rest, cut)
    }

    /// Cut `w` from the right, leaving `gap` before the cut.
    pub fn cut_right(self, w: i32, gap: i32) -> (Self, Self) {
        let w = w.clamp(0, self.w);
        let rest_w = (self.w - w - gap).max(0);
        let rest = Self::new(self.x, self.y, rest_w, self.h);
        let cut = Self::new(self.x + rest_w + gap, self.y, w, self.h);
        (rest, cut)
    }

    /// Split horizontally by ratio (0.0..=1.0) with `gap` between halves.
    pub fn split_h(self, ratio: f32, gap: i32) -> (Self, Self) {
        let usable = (self.w - gap).max(0);
        let left_w = (usable as f32 * ratio.clamp(0.0, 1.0)) as i32;
        let right_w = usable - left_w;
        let left = Self::new(self.x, self.y, left_w, self.h);
        let right = Self::new(self.x + left_w + gap, self.y, right_w, self.h);
        (left, right)
    }

    // ---- common templates ----

    /// Left fixed, right flexible. Returns `(right, left)`.
    pub fn left_fixed(self, left_w: i32, gap: i32) -> (Self, Self) {
        self.cut_left(left_w, gap)
    }

    /// Top fixed, rest flexible. Returns `(bottom, top)`.
    pub fn top_fixed(self, top_h: i32, gap: i32) -> (Self, Self) {
        let (rest, top) = self.cut_top(top_h);
        (rest.shift(0, gap), top)
    }

    /// A row of `n` centered buttons inside this rect.
    /// Buttons are vertically and horizontally centered.
    pub fn buttons_row(self, n: usize, btn_w: i32, btn_h: i32, gap: i32) -> Vec<Self> {
        if n == 0 {
            return Vec::new();
        }
        let n_i32 = n as i32;
        let total_w = btn_w * n_i32 + gap * (n_i32 - 1);
        let start_x = self.x + (self.w - total_w) / 2;
        let btn_y = self.y + (self.h - btn_h) / 2;

        (0..n)
            .map(|i| Self {
                x: start_x + i as i32 * (btn_w + gap),
                y: btn_y,
                w: btn_w,
                h: btn_h,
            })
            .collect()
    }

    /// A form row: label on the left, field on the right.
    /// Returns `(field, label)`.
    pub fn form_row(self, label_w: i32, gap: i32) -> (Self, Self) {
        self.cut_left(label_w, gap)
    }

    /// A grid of `rows * cols` cells with uniform `gap`.
    pub fn grid(self, rows: i32, cols: i32, gap: i32) -> Vec<Self> {
        if rows <= 0 || cols <= 0 {
            return Vec::new();
        }
        let cell_w = (self.w - gap * (cols - 1)) / cols;
        let cell_h = (self.h - gap * (rows - 1)) / rows;
        let mut out = Vec::with_capacity((rows * cols) as usize);
        for r in 0..rows {
            for c in 0..cols {
                out.push(Self::new(
                    self.x + c * (cell_w + gap),
                    self.y + r * (cell_h + gap),
                    cell_w,
                    cell_h,
                ));
            }
        }
        out
    }
}