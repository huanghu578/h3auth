//! String utilities.

pub struct Text;

impl Text {
    /// Truncate to `max_chars` characters, appending an ellipsis if cut.
    pub fn truncate(s: &str, max_chars: usize) -> String {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() <= max_chars {
            s.to_string()
        } else {
            let mut out: String = chars[..max_chars.saturating_sub(1)].iter().collect();
            out.push('…');
            out
        }
    }

    /// Truncate by display width. CJK characters count as 2, others as 1.
    pub fn truncate_display(s: &str, max_width: usize) -> String {
        let mut out = String::new();
        let mut w = 0usize;
        for ch in s.chars() {
            let cw = if (ch as u32) > 0x2E80 { 2 } else { 1 };
            if w + cw > max_width.saturating_sub(1) {
                out.push('…');
                return out;
            }
            out.push(ch);
            w += cw;
        }
        out
    }

    /// Format a byte count as human-readable (e.g. `1.5 MB`).
    pub fn human_bytes(n: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut v = n as f64;
        let mut i = 0;
        while v >= 1024.0 && i < UNITS.len() - 1 {
            v /= 1024.0;
            i += 1;
        }
        if i == 0 {
            format!("{} {}", n, UNITS[i])
        } else {
            format!("{:.1} {}", v, UNITS[i])
        }
    }

    /// Format an integer with thousands separators.
    pub fn human_count(n: u64) -> String {
        let s = n.to_string();
        let bytes = s.as_bytes();
        let mut out = String::new();
        let len = bytes.len();
        for (i, b) in bytes.iter().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                out.push(',');
            }
            out.push(*b as char);
        }
        out
    }
}