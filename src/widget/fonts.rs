//! Font selection with a three-tier priority:
//! 1. user-provided candidates
//! 2. operating-system language
//! 3. generic fallback list

use fltk::{app, enums::Font};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSource {
    User,
    System,
    Fallback,
    Default,
}

#[derive(Debug, Clone)]
pub struct FontSetup {
    pub success: bool,
    pub source: FontSource,
    pub font_name: Option<String>,
    pub size: i32,
}

impl FontSetup {
    fn ok(source: FontSource, name: &str, size: i32) -> Self {
        Self { success: true, source, font_name: Some(name.to_string()), size }
    }
    fn fail(source: FontSource, size: i32) -> Self {
        Self { success: false, source, font_name: None, size }
    }
}

pub struct Fonts;

impl Fonts {
    pub const DEFAULT_SIZE: i32 = 14;

    pub const FALLBACK: &'static [&'static str] = &[
        "Arial", "Helvetica", "DejaVu Sans", "Liberation Sans", "Segoe UI",
    ];

    pub const CANDIDATES_ZH_CN: &'static [&'static str] = &[
        "Microsoft YaHei", "PingFang SC", "Noto Sans CJK SC",
        "WenQuanYi Micro Hei", "SimHei", "SimSun",
    ];

    pub const CANDIDATES_ZH_TW: &'static [&'static str] = &[
        "Microsoft JhengHei", "PingFang TC", "Noto Sans CJK TC",
        "WenQuanYi Zen Hei", "PMingLiU",
    ];

    pub const CANDIDATES_JA: &'static [&'static str] = &[
        "Yu Gothic UI", "Meiryo", "Hiragino Sans", "Noto Sans CJK JP", "MS Gothic",
    ];

    pub const CANDIDATES_KO: &'static [&'static str] = &[
        "Malgun Gothic", "Apple SD Gothic Neo", "Noto Sans CJK KR", "Gulim",
    ];

    pub const CANDIDATES_AR: &'static [&'static str] = &[
        "Segoe UI", "Geeza Pro", "Noto Sans Arabic", "Amiri",
    ];

    pub const CANDIDATES_LATIN: &'static [&'static str] = &[
        "Segoe UI", "Helvetica Neue", "Noto Sans", "DejaVu Sans",
    ];

    /// Apply a font using the three-tier priority.
    pub fn setup_auto(
        user: Option<&[&str]>,
        lang: Option<&str>,
        size: i32,
    ) -> FontSetup {
        if let Some(list) = user {
            if !list.is_empty() {
                if let Some((name, font)) = Self::find_first(list) {
                    app::set_font(font);
                    app::set_font_size(size);
                    return FontSetup::ok(FontSource::User, &name, size);
                }
            }
        }
        if let Some(lang) = lang {
            let sys = Self::candidates_for_language(lang);
            if let Some((name, font)) = Self::find_first(sys) {
                app::set_font(font);
                app::set_font_size(size);
                return FontSetup::ok(FontSource::System, &name, size);
            }
        }
        if let Some((name, font)) = Self::find_first(Self::FALLBACK) {
            app::set_font(font);
            app::set_font_size(size);
            return FontSetup::ok(FontSource::Fallback, &name, size);
        }
        FontSetup::fail(FontSource::Default, size)
    }

    pub fn setup_by_language(lang: &str, size: i32) -> FontSetup {
        Self::setup_auto(None, Some(lang), size)
    }

    pub fn setup_custom(candidates: &[&str], size: i32) -> FontSetup {
        Self::setup_auto(Some(candidates), None, size)
    }

    pub fn setup_cjk() -> FontSetup {
        Self::setup_by_language("zh-CN", Self::DEFAULT_SIZE)
    }

    pub fn set_default(size: i32) {
        app::set_font(Font::Helvetica);
        app::set_font_size(size);
    }

    /// Find the first available font from `candidates`.
    pub fn find_first(candidates: &[&str]) -> Option<(String, Font)> {
        for name in candidates {
            if let Some(font) = Self::resolve(name) {
                return Some((name.to_string(), font));
            }
        }
        None
    }

    /// Candidate list for a language code such as `"zh-CN"`.
    pub fn candidates_for_language(lang: &str) -> &'static [&'static str] {
        let primary = lang.split(['-', '_']).next().unwrap_or(lang).to_lowercase();
        match primary.as_str() {
            "zh" => {
                let lower = lang.to_lowercase();
                if lower.contains("tw") || lower.contains("hk") || lower.contains("hant") {
                    Self::CANDIDATES_ZH_TW
                } else {
                    Self::CANDIDATES_ZH_CN
                }
            }
            "ja" => Self::CANDIDATES_JA,
            "ko" => Self::CANDIDATES_KO,
            "ar" | "fa" | "ur" => Self::CANDIDATES_AR,
            _ => Self::CANDIDATES_LATIN,
        }
    }

    pub fn is_available(name: &str) -> bool {
        Self::resolve(name).is_some()
    }

    /// Resolve a font name. Returns `None` if the font is unavailable.
    pub fn resolve(name: &str) -> Option<Font> {
        if name.eq_ignore_ascii_case("Helvetica") {
            return Some(Font::Helvetica);
        }
        let font = Font::by_name(name);
        if font != Font::Helvetica { Some(font) } else { None }
    }
}