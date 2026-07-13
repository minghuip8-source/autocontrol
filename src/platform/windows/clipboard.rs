use crate::error::{AutoError, Result};

use arboard::{Clipboard, ImageData};

use image::{ImageBuffer, RgbaImage};

pub struct WindowsClipboard;

impl WindowsClipboard {
    fn clipboard() -> Result<Clipboard> {
        Clipboard::new().map_err(|e| AutoError::Clipboard(e.to_string()))
    }

    /// 获取剪贴板文本
    pub fn get_text() -> Result<String> {
        let mut clipboard = Self::clipboard().unwrap();

        clipboard
            .get_text()
            .map_err(|e| AutoError::Clipboard(e.to_string()))
    }

    /// 设置剪贴板文本
    pub fn set_text(text: &str) -> Result<()> {
        let mut clipboard = Self::clipboard().unwrap();

        clipboard
            .set_text(text)
            .map_err(|e| AutoError::Clipboard(e.to_string()))
    }

    /// 清空剪贴板
    pub fn clear() -> Result<()> {
        let mut clipboard = Self::clipboard().unwrap();

        clipboard
            .clear()
            .map_err(|e| AutoError::Clipboard(e.to_string()))
    }

    /// 获取剪贴板图片
    pub fn get_image() -> Result<RgbaImage> {
        let mut clipboard = Self::clipboard().unwrap();

        let image = clipboard
            .get_image()
            .map_err(|e| AutoError::Clipboard(e.to_string()))
            .unwrap();

        let buffer = ImageBuffer::from_raw(
            image.width as u32,
            image.height as u32,
            image.bytes.into_owned(),
        )
        .ok_or(AutoError::Clipboard("Invalid image data".into()))
        .unwrap();

        Ok(buffer)
    }

    /// 设置剪贴板图片
    pub fn set_image(image: &RgbaImage) -> Result<()> {
        let data = ImageData {
            width: image.width() as usize,

            height: image.height() as usize,

            bytes: std::borrow::Cow::Borrowed(image.as_raw()),
        };

        let mut clipboard = Self::clipboard().unwrap();

        clipboard
            .set_image(data)
            .map_err(|e| AutoError::Clipboard(e.to_string()))
    }
}
