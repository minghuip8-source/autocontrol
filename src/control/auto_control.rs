use crate::error::{AutoError, Result};
use crate::platform::screen::WindowsScreen;
use crate::{
    keyboard::Key,
    platform::{WindowsClipboard, WindowsKeyboard, WindowsMouse},
};
use image::RgbaImage;
use std::thread;
use std::time::Duration;

/// 自动化控制器
///
/// `AutoControl` 是自动化操作的统一入口，
/// 提供鼠标、键盘、屏幕等自动化能力。
///
/// 当前实现基于 Windows API，底层由 [`WindowsMouse`] 等平台模块完成。
///
/// # Examples
///
/// ```
/// let auto = AutoControl::new();
///
/// let (x, y) = auto.position().unwrap();
///
/// auto.move_absolute(500, 300).unwrap();
/// auto.left_click().unwrap();
/// ```
pub struct AutoControl;
impl AutoControl {
    /// 创建一个新的自动化控制器实例
    ///
    /// # Returns
    ///
    /// 返回初始化完成的 `AutoControl` 实例。
    pub fn new() -> Self {
        Self
    }
    /// 获取当前鼠标屏幕坐标
    ///
    /// # Returns
    ///
    /// 返回鼠标当前位置：
    ///
    /// `(x, y)`
    ///
    /// 其中：
    ///
    /// - `x` 表示屏幕横向坐标
    /// - `y` 表示屏幕纵向坐标
    ///
    pub fn position(&self) -> Result<(i32, i32)> {
        Ok(WindowsMouse::position().unwrap())
    }

    /// 左键单击
    ///
    /// 模拟一次完整的：
    ///
    /// ```text
    /// MouseDown
    /// MouseUp
    /// ```
    pub fn left_click(&self) -> Result<()> {
        Ok(WindowsMouse::left_click().unwrap())
    }
    /// 右键单击
    pub fn right_click(&self) -> Result<()> {
        Ok(WindowsMouse::right_click().unwrap())
    }
    /// 中键单击
    pub fn middle_click(&self) -> Result<()> {
        Ok(WindowsMouse::middle_click().unwrap())
    }
    /// 按下鼠标左键
    ///
    /// 按下后不会自动释放，
    /// 需要调用 [`Self::left_up`] 释放。
    pub fn left_down(&self) -> Result<()> {
        Ok(WindowsMouse::left_down().unwrap())
    }
    /// 释放鼠标左键
    pub fn left_up(&self) -> Result<()> {
        Ok(WindowsMouse::left_up().unwrap())
    }

    /// 鼠标双击
    ///
    /// 两次点击之间默认间隔80毫秒。
    pub fn double_click(&self) -> Result<()> {
        self.left_click().unwrap();

        thread::sleep(Duration::from_millis(80));

        self.left_click().unwrap();

        Ok(())
    }
    /// 相对移动鼠标
    ///
    /// 根据当前位置进行偏移移动。
    ///
    /// # Arguments
    ///
    /// * `dx` - X轴方向偏移量，正数向右，负数向左
    /// * `dy` - Y轴方向偏移量，正数向下，负数向上
    ///
    /// # Example
    ///
    /// ```
    /// // 鼠标向右移动100像素
    /// auto.move_relative(100,0).unwrap();
    /// ```

    pub fn move_relative(&self, dx: i32, dy: i32) -> Result<()> {
        Ok(WindowsMouse::move_relative(dx, dy).unwrap())
    }
    /// 移动鼠标到指定屏幕坐标
    ///
    /// # Arguments
    ///
    /// * `x` - 目标X坐标
    /// * `y` - 目标Y坐标
    ///
    pub fn move_absolute(&self, x: i32, y: i32) -> Result<()> {
        Ok(WindowsMouse::move_absolute(x, y).unwrap())
    }
    /// 垂直滚动鼠标滚轮
    ///
    /// # Arguments
    ///
    /// * `delta`
    ///
    /// 滚动距离：
    ///
    /// - 正数：向上滚动
    /// - 负数：向下滚动
    ///
    /// 一个标准滚轮单位通常为 `120`。
    ///
    pub fn scroll_vertical(&self, delta: i32) -> Result<()> {
        if delta > 0 {
            WindowsMouse::wheel_up(delta).unwrap();
        } else {
            WindowsMouse::wheel_down(delta.abs()).unwrap();
        }
        Ok(())
    }
    /// 水平滚动鼠标滚轮
    ///
    /// # Arguments
    ///
    /// * `delta`
    ///
    /// - 正数：向右滚动
    /// - 负数：向左滚动
    ///
    pub fn scroll_horizontal(&self, delta: i32) -> Result<()> {
        if delta > 0 {
            WindowsMouse::wheel_right(delta).unwrap();
        } else {
            WindowsMouse::wheel_left(delta.abs()).unwrap();
        }
        Ok(())
    }

    /// 平滑移动鼠标到目标位置
    ///
    /// 鼠标会按照固定步数进行插值移动，
    /// 模拟人工移动效果。
    ///
    /// # Arguments
    ///
    /// * `x` - 目标X坐标
    /// * `y` - 目标Y坐标
    /// * `duration_ms` - 移动持续时间（毫秒）
    ///
    /// # Example
    ///
    /// ```
    /// // 1秒移动到(800,600)
    /// auto.smooth_move(800,600,1000).unwrap();
    /// ```
    pub fn smooth_move(&self, x: i32, y: i32, duration_ms: u64) -> Result<()> {
        let (sx, sy) = WindowsMouse::position().unwrap();

        let steps = 120;

        let dx = (x - sx) as f32 / steps as f32;
        let dy = (y - sy) as f32 / steps as f32;

        for i in 0..steps {
            WindowsMouse::move_absolute(
                (sx as f32 + dx * i as f32) as i32,
                (sy as f32 + dy * i as f32) as i32,
            )
            .unwrap();

            thread::sleep(Duration::from_millis(duration_ms / steps as u64));
        }

        WindowsMouse::move_absolute(x, y).unwrap();

        Ok(())
    }
    /// 拖拽鼠标
    ///
    /// 从指定起点按住鼠标左键，
    /// 平滑移动到目标位置后释放。
    ///
    /// # Arguments
    ///
    /// * `from` - 起始坐标
    /// * `to` - 目标坐标
    /// * `duration_ms` - 拖拽时间
    ///
    pub fn drag(&self, from: (i32, i32), to: (i32, i32), duration_ms: u64) -> Result<()> {
        self.move_absolute(from.0, from.1).unwrap();

        self.left_down().unwrap();

        self.smooth_move(to.0, to.1, duration_ms).unwrap();

        self.left_up().unwrap();

        Ok(())
    }

    /// 按下键盘按键
    ///
    /// 按下后不会自动释放。
    ///
    /// 需要配合 [`Self::key_up`] 使用。
    ///
    /// # Example
    ///
    /// ```
    /// auto.key_down(Key::Ctrl).unwrap();
    /// auto.key_up(Key::Ctrl).unwrap();
    /// ```
    pub fn key_down(&self, key: Key) -> Result<()> {
        Ok(WindowsKeyboard::key_down(key.vk()).unwrap())
    }
    /// 释放键盘按键
    ///
    /// # Example
    ///
    /// ```
    /// auto.key_up(Key::Ctrl).unwrap();
    /// ```
    pub fn key_up(&self, key: Key) -> Result<()> {
        Ok(WindowsKeyboard::key_up(key.vk()).unwrap())
    }

    /// 根据字符串按下按键
    ///
    /// 支持:
    ///
    /// - ctrl
    /// - shift
    /// - alt
    /// - enter
    /// - esc
    /// - a-z
    ///
    /// # Example
    ///
    /// ```
    /// auto.key_down_str("ctrl").unwrap();
    /// ```
    pub fn key_down_str(&self, key: &str) -> Result<()> {
        let key = key
            .parse::<Key>()
            .map_err(|_| AutoError::InvalidKey(key.to_string()))
            .unwrap();

        self.key_down(key)
    }

    pub fn key_up_str(&self, key: &str) -> Result<()> {
        let key = key
            .parse::<Key>()
            .map_err(|_| AutoError::InvalidKey(key.to_string()))
            .unwrap();

        self.key_up(key)
    }
    /// 按下并释放按键
    ///
    /// 等价于：
    ///
    /// ```text
    /// KeyDown
    /// KeyUp
    /// ```
    ///
    /// 默认按下后等待约30毫秒再释放。
    ///
    /// # Arguments
    ///
    /// * `key` - 按键枚举
    ///
    /// # Example
    ///
    /// ```
    /// auto.press(Key::Enter).unwrap();
    /// auto.press(Key::F5).unwrap();
    /// ```
    pub fn press(&self, key: Key) -> Result<()> {
        Ok(WindowsKeyboard::press(key.vk()).unwrap())
    }
    /// 根据字符串按下并释放按键
    ///
    /// 支持常见按键名称：
    ///
    /// - ctrl
    /// - shift
    /// - alt
    /// - enter
    /// - esc
    /// - tab
    /// - f1-f12
    /// - a-z
    /// - 0-9
    ///
    /// # Arguments
    ///
    /// * `key` - 按键名称
    ///
    /// # Example
    ///
    /// ```
    /// auto.press_str("enter").unwrap();
    /// auto.press_str("f5").unwrap();
    /// ```
    pub fn press_str(&self, key: &str) -> Result<()> {
        let key = key
            .parse::<Key>()
            .map_err(|_| AutoError::InvalidKey(key.to_string()))
            .unwrap();

        self.press(key)
    }

    fn combo_vk(&self, keys: &[u16]) -> Result<()> {
        WindowsKeyboard::combo(keys)
    }
    /// 执行组合键
    ///
    /// 按照传入顺序依次按下，
    /// 然后按照相反顺序释放。
    ///
    /// # Example
    ///
    /// ```
    /// auto.combo([
    ///     Key::Ctrl,
    ///     Key::Shift,
    ///     Key::S,
    /// ]).unwrap();
    /// ```
    ///
    /// 实际执行：
    ///
    /// ```text
    /// Ctrl Down
    /// Shift Down
    /// S Down
    ///
    /// S Up
    /// Shift Up
    /// Ctrl Up
    /// ```
    pub fn combo<const N: usize>(&self, keys: [Key; N]) -> Result<()> {
        let vks: Vec<u16> = keys.into_iter().map(Key::vk).collect();

        self.combo_vk(&vks)
    }
    /// 使用字符串执行组合键
    ///
    /// # Example
    ///
    /// ```
    /// auto.combo_str([
    ///     "ctrl",
    ///     "shift",
    ///     "s",
    /// ]).unwrap();
    /// ```
    pub fn combo_str<const N: usize>(&self, keys: [&str; N]) -> Result<()> {
        let vks: Vec<u16> = keys
            .into_iter()
            .map(|k| {
                k.parse::<Key>()
                    .map(|k| k.vk())
                    .map_err(|_| AutoError::InvalidKey(k.to_string()))
            })
            .collect::<Result<_>>()
            .unwrap();

        self.combo_vk(&vks)
    }

    /// 输入文本
    ///
    /// 使用 Windows Unicode 输入方式。
    ///
    /// 支持:
    ///
    /// - 英文
    /// - 数字
    /// - 中文
    /// - 特殊符号
    ///
    /// # Example
    ///
    /// ```
    /// auto.text("Hello Rust").unwrap();
    /// ```
    pub fn text(&self, text: &str) -> Result<()> {
        Ok(WindowsKeyboard::text(text).unwrap())
    }
    /// 阻塞当前线程
    ///
    /// 用于自动化流程中的等待操作。
    ///
    /// # Arguments
    ///
    /// * `ms` - 等待时间（毫秒）
    ///
    /// # Example
    ///
    /// ```
    /// auto.left_click().unwrap();
    ///
    /// auto.wait(1000).unwrap();
    ///
    /// auto.text("hello").unwrap();
    /// ```
    pub fn wait(&self, ms: u64) -> Result<()> {
        thread::sleep(Duration::from_millis(ms));
        Ok(())
    }
    /// 获取主显示器宽度
    ///
    /// # Returns
    ///
    /// 返回屏幕宽度（像素）
    ///
    /// # Example
    ///
    /// ```
    /// let width = auto.screen_width().unwrap();
    /// ```
    pub fn screen_width(&self) -> Result<i32> {
        Ok(WindowsScreen::screen_width().unwrap())
    }
    /// 获取主显示器高度
    ///
    /// # Returns
    ///
    /// 返回屏幕高度（像素）
    pub fn screen_height(&self) -> Result<i32> {
        Ok(WindowsScreen::screen_height().unwrap())
    }
    /// 获取主显示器尺寸
    ///
    /// # Returns
    ///
    /// 返回：
    ///
    /// ```text
    /// (width, height)
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// let (w, h) = auto.screen_size().unwrap();
    /// ```
    pub fn screen_size(&self) -> Result<(i32, i32)> {
        Ok(WindowsScreen::screen_size().unwrap())
    }
    /// 获取剪贴板文本
    ///
    /// # Returns
    ///
    /// 返回当前剪贴板中的文本内容。
    ///
    /// # Example
    ///
    /// ```
    /// let text = auto.clipboard_get_text().unwrap();
    /// ```
    pub fn clipboard_get_text(&self) -> Result<String> {
        WindowsClipboard::get_text()
    }
    /// 设置剪贴板文本
    ///
    /// # Arguments
    ///
    /// * `text` - 要写入剪贴板的文本
    ///
    /// # Example
    ///
    /// ```
    /// auto.clipboard_set_text(
    ///     "Hello World"
    /// ).unwrap();
    /// ```
    pub fn clipboard_set_text(&self, text: &str) -> Result<()> {
        WindowsClipboard::set_text(text)
    }
    /// 清空剪贴板内容
    ///
    /// # Example
    ///
    /// ```
    /// auto.clipboard_clear().unwrap();
    /// ```
    pub fn clipboard_clear(&self) -> Result<()> {
        WindowsClipboard::clear()
    }
    /// 获取剪贴板图片
    ///
    /// # Returns
    ///
    /// 返回 RGBA 格式图片。
    ///
    /// # Example
    ///
    /// ```
    /// let image = auto.clipboard_image().unwrap();
    /// ```
    pub fn clipboard_get_image(&self) -> Result<RgbaImage> {
        WindowsClipboard::get_image()
    }
    /// 设置剪贴板图片
    ///
    /// # Arguments
    ///
    /// * `image` - 图片对象
    ///
    /// # Example
    ///
    /// ```
    /// let image = auto.screenshot().unwrap();
    ///
    /// auto.clipboard_set_image(&image).unwrap();
    /// ```
    pub fn clipboard_set_image(&self, image: &RgbaImage) -> Result<()> {
        WindowsClipboard::set_image(image)
    }
    /// 全屏截图
    ///
    /// 捕获主显示器当前画面。
    ///
    /// # Returns
    ///
    /// 返回 RGBA 图片对象。
    ///
    /// 图片可直接保存：
    ///
    /// ```
    /// auto
    ///     .screenshot().unwrap()
    ///     .save("screen.png").unwrap();
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// let image = auto.screenshot().unwrap();
    ///
    /// println!(
    ///     "{}x{}",
    ///     image.width(),
    ///     image.height()
    /// );
    /// ```
    pub fn screenshot(&self) -> Result<RgbaImage> {
        WindowsScreen::screenshot()
    }
    /// 区域截图
    ///
    /// 从屏幕指定区域截取图像。
    ///
    /// # Arguments
    ///
    /// * `x` - 起始X坐标
    /// * `y` - 起始Y坐标
    /// * `width` - 截图宽度
    /// * `height` - 截图高度
    ///
    /// # Example
    ///
    /// ```
    /// auto
    ///     .screenshot_region(
    ///         100,
    ///         100,
    ///         500,
    ///         300,
    ///     ).unwrap()
    ///     .save("region.png").unwrap();
    /// ```
    ///
    /// # Returns
    ///
    /// 返回 RGBA 图片对象。
    pub fn screenshot_region(&self, x: u32, y: u32, width: u32, height: u32) -> Result<RgbaImage> {
        WindowsScreen::screenshot_region(x, y, width, height)
    }
}
