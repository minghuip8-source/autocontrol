use crate::error::{AutoError, Result};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use xcap::Monitor;
pub struct WindowsScreen;
use image::{RgbaImage, imageops};
impl WindowsScreen {
    #[inline]
    pub fn screen_width() -> Result<i32> {
        Ok(unsafe { GetSystemMetrics(SM_CXSCREEN) })
    }
    #[inline]
    pub fn screen_height() -> Result<i32> {
        Ok(unsafe { GetSystemMetrics(SM_CYSCREEN) })
    }
    #[inline]
    pub fn screen_size() -> Result<(i32, i32)> {
        Ok(unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) })
    }

    /// 获取主显示器
    fn monitor() -> Result<Monitor> {
        Monitor::all()
            .map_err(|e| AutoError::ScreenshotFailed(e.to_string()))
            .unwrap()
            .into_iter()
            .next()
            .ok_or_else(|| AutoError::ScreenshotFailed("No monitor found".into()))
    }
    /// 全屏截图
    pub fn screenshot() -> Result<RgbaImage> {
        let monitor = Self::monitor().unwrap();

        monitor
            .capture_image()
            .map_err(|e| AutoError::ScreenshotFailed(e.to_string()))
    }

    /// 区域截图
    pub fn screenshot_region(x: u32, y: u32, width: u32, height: u32) -> Result<RgbaImage> {
        let image = Self::screenshot().unwrap();

        Ok(imageops::crop_imm(&image, x, y, width, height).to_image())
    }
}
