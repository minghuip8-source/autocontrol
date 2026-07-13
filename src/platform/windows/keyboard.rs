use crate::error::Result;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
pub struct WindowsKeyboard;
use std::{thread, time::Duration};
impl WindowsKeyboard {
    #[inline]
    pub fn key_down(vk: u16) -> Result<()> {
        unsafe {
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(vk),
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
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
    pub fn key_up(vk: u16) -> Result<()> {
        unsafe {
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(vk),
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
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
    pub fn press(vk: u16) -> Result<()> {
        Self::key_down(vk).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(30));

        Self::key_up(vk)
    }

    pub fn combo(keys: &[u16]) -> Result<()> {
        for vk in keys {
            Self::key_down(*vk).unwrap();
        }

        for vk in keys.iter().rev() {
            Self::key_up(*vk).unwrap();
        }

        Ok(())
    }

    pub fn text(text: &str) -> Result<()> {
        thread::sleep(Duration::from_millis(10));
        for ch in text.encode_utf16() {
            let inputs = [
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VIRTUAL_KEY(0),
                            wScan: ch,
                            dwFlags: KEYEVENTF_UNICODE,
                            ..Default::default()
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VIRTUAL_KEY(0),
                            wScan: ch,
                            dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                            ..Default::default()
                        },
                    },
                },
            ];

            unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
        }

        Ok(())
    }
}
