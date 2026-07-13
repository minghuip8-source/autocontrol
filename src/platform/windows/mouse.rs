use windows::{
    Win32::{
        Foundation::POINT,
        UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
    },
    core::Result,
};

pub struct WindowsMouse;

impl WindowsMouse {
    #[inline]
    pub fn position() -> Result<(i32, i32)> {
        unsafe {
            let mut pt = POINT::default();

            GetCursorPos(&mut pt).unwrap();

            Ok((pt.x, pt.y))
        }
    }

    #[inline]
    pub fn move_absolute(x: i32, y: i32) -> Result<()> {
        unsafe {
            SetCursorPos(x, y).unwrap();
        }

        Ok(())
    }

    #[inline]
    pub fn move_relative(dx: i32, dy: i32) -> Result<()> {
        let (x, y) = Self::position().unwrap();

        Self::move_absolute(x + dx, y + dy)
    }

    #[inline]
    pub fn send(flags: MOUSE_EVENT_FLAGS) -> Result<()> {
        unsafe {
            let input = INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: flags,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);

            Ok(())
        }
    }

    #[inline]
    pub fn left_down() -> Result<()> {
        Self::send(MOUSEEVENTF_LEFTDOWN)
    }

    #[inline]
    pub fn left_up() -> Result<()> {
        Self::send(MOUSEEVENTF_LEFTUP)
    }

    #[inline]
    pub fn left_click() -> Result<()> {
        Self::left_down().unwrap();
        Self::left_up()
    }

    #[inline]
    pub fn right_down() -> Result<()> {
        Self::send(MOUSEEVENTF_RIGHTDOWN)
    }

    #[inline]
    pub fn right_up() -> Result<()> {
        Self::send(MOUSEEVENTF_RIGHTUP)
    }

    #[inline]
    pub fn right_click() -> Result<()> {
        Self::right_down().unwrap();
        Self::right_up()
    }

    #[inline]
    pub fn middle_down() -> Result<()> {
        Self::send(MOUSEEVENTF_MIDDLEDOWN)
    }

    #[inline]
    pub fn middle_up() -> Result<()> {
        Self::send(MOUSEEVENTF_MIDDLEUP)
    }

    #[inline]
    pub fn middle_click() -> Result<()> {
        Self::middle_down().unwrap();
        Self::middle_up()
    }

    #[inline]
    pub fn wheel_up(delta: i32) -> Result<()> {
        unsafe {
            let input = INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        mouseData: delta as u32,
                        dwFlags: MOUSEEVENTF_WHEEL,
                        ..Default::default()
                    },
                },
            };

            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);

            Ok(())
        }
    }

    #[inline]
    pub fn wheel_down(delta: i32) -> Result<()> {
        Self::wheel_up(-delta)
    }

    #[inline]
    pub fn wheel_left(delta: i32) -> Result<()> {
        unsafe {
            let input = INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        mouseData: delta as u32,
                        dwFlags: MOUSEEVENTF_HWHEEL,
                        ..Default::default()
                    },
                },
            };

            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);

            Ok(())
        }
    }

    #[inline]
    pub fn wheel_right(delta: i32) -> Result<()> {
        Self::wheel_left(-delta)
    }
}
