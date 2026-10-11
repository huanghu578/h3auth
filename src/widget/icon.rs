//! Window icons and image loading.

use base64::{engine::general_purpose, Engine as _};
use fltk::{
    frame::Frame,
    image::{IcoImage, PngImage, RgbImage},
    prelude::*,
    window::Window,
};

pub struct Icon;

impl Icon {
    pub fn set_embd_png(wind: &mut Window, data: &[u8]) -> bool {
        if let Some(icon) = Self::png_from_bytes(data) {
            wind.set_icon(Some(icon));
            true
        } else {
            false
        }
    }

    pub fn set_embd_ico(wind: &mut Window, data: &[u8]) -> bool {
        if let Some(icon) = Self::ico_from_bytes(data) {
            wind.set_icon(Some(icon));
            true
        } else {
            false
        }
    }

    pub fn set_embd_auto(wind: &mut Window, data: &[u8]) -> bool {
        Self::set_embd_png(wind, data) || Self::set_embd_ico(wind, data)
    }

    pub fn set_base64_png(wind: &mut Window, b64: &str) -> bool {
        if let Some(icon) = Self::png_from_base64(b64) {
            wind.set_icon(Some(icon));
            true
        } else {
            false
        }
    }

    pub fn set_base64_ico(wind: &mut Window, b64: &str) -> bool {
        if let Some(icon) = Self::ico_from_base64(b64) {
            wind.set_icon(Some(icon));
            true
        } else {
            false
        }
    }

    pub fn png_from_bytes(data: &[u8]) -> Option<PngImage> {
        PngImage::from_data(data).ok()
    }

    pub fn ico_from_bytes(data: &[u8]) -> Option<IcoImage> {
        IcoImage::from_data(data).ok()
    }

    pub fn png_from_base64(b64: &str) -> Option<PngImage> {
        let bytes = Self::decode_base64(b64)?;
        Self::png_from_bytes(&bytes)
    }

    pub fn ico_from_base64(b64: &str) -> Option<IcoImage> {
        let bytes = Self::decode_base64(b64)?;
        Self::ico_from_bytes(&bytes)
    }

    pub fn rgb_from_bytes(data: &[u8]) -> Option<RgbImage> {
        let img = image::load_from_memory(data).ok()?;
        let rgb = img.to_rgb8();
        let (w, h) = rgb.dimensions();
        let raw = rgb.into_raw();
        RgbImage::new(&raw, w as i32, h as i32, fltk::enums::ColorDepth::Rgb8).ok()
    }

    pub fn rgb_from_base64(b64: &str) -> Option<RgbImage> {
        let bytes = Self::decode_base64(b64)?;
        Self::rgb_from_bytes(&bytes)
    }

    pub fn scale_rgb(mut img: RgbImage, w: i32, h: i32, keep_aspect: bool) -> RgbImage {
        img.scale(w, h, keep_aspect, true);
        img
    }

    pub fn rgb_from_base64_scaled(b64: &str, w: i32, h: i32) -> Option<RgbImage> {
        let img = Self::rgb_from_base64(b64)?;
        Some(Self::scale_rgb(img, w, h, true))
    }

    /// Decode Base64 and display the image in `frame`, scaled to fit.
    pub fn show_base64_in_frame(frame: &mut Frame, b64: &str) {
        match Self::rgb_from_base64(b64) {
            Some(img) => {
                let (w, h) = (frame.width(), frame.height());
                let scaled = Self::scale_rgb(img, w, h, true);
                frame.set_image(Some(scaled));
                frame.set_label("");
            }
            _ => {
                frame.set_label("(image decode failed)");
                frame.set_image(None::<RgbImage>);
            }
        }
        frame.redraw();
    }

    fn decode_base64(b64: &str) -> Option<Vec<u8>> {
        let cleaned: String = b64.chars().filter(|c| !c.is_whitespace()).collect();
        general_purpose::STANDARD.decode(cleaned).ok()
    }
}