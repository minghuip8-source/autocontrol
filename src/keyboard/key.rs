use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Key {
    // =====================
    // Control
    // =====================
    Backspace = 0x08,
    Tab = 0x09,

    Enter = 0x0D,

    Shift = 0x10, //
    Ctrl = 0x11,  //
    Alt = 0x12,   //

    Pause = 0x13,
    CapsLock = 0x14,

    Escape = 0x1B, //

    Space = 0x20, //

    // =====================
    // Navigation
    // =====================
    PageUp = 0x21,
    PageDown = 0x22,

    End = 0x23,
    Home = 0x24,

    Left = 0x25,  //
    Up = 0x26,    //
    Right = 0x27, //
    Down = 0x28,  //

    Insert = 0x2D,
    Delete = 0x2E,
    // =====================
    // Windows
    // =====================
    LeftWin = 0x5B, //
    RightWin = 0x5C,

    Menu = 0x5D,
    // =====================
    // Number
    // =====================
    Num0 = 0x30,
    Num1 = 0x31,
    Num2 = 0x32,
    Num3 = 0x33,
    Num4 = 0x34,
    Num5 = 0x35,
    Num6 = 0x36,
    Num7 = 0x37,
    Num8 = 0x38,
    Num9 = 0x39,

    // =====================
    // Alphabet
    // =====================
    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,

    // =====================
    // Function
    // =====================
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    F13 = 0x7C,
    F14 = 0x7D,
    F15 = 0x7E,
    F16 = 0x7F,
    F17 = 0x80,
    F18 = 0x81,
    F19 = 0x82,
    F20 = 0x83,
    F21 = 0x84,
    F22 = 0x85,
    F23 = 0x86,
    F24 = 0x87,

    // =====================
    // Numpad
    // =====================
    Numpad0 = 0x60,
    Numpad1 = 0x61,
    Numpad2 = 0x62,
    Numpad3 = 0x63,
    Numpad4 = 0x64,
    Numpad5 = 0x65,
    Numpad6 = 0x66,
    Numpad7 = 0x67,
    Numpad8 = 0x68,
    Numpad9 = 0x69,

    Multiply = 0x6A,
    Add = 0x6B,
    Subtract = 0x6D,
    Decimal = 0x6E,
    Divide = 0x6F,
}
impl Key {
    #[inline]
    pub const fn vk(self) -> u16 {
        self as u16
    }
}
impl FromStr for Key {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let key = value.to_lowercase();

        match key.as_str() {
            "backspace" => Ok(Key::Backspace),
            "tab" => Ok(Key::Tab),

            "enter" | "return" => Ok(Key::Enter),

            "shift" => Ok(Key::Shift),
            "ctrl" | "control" => Ok(Key::Ctrl),
            "alt" => Ok(Key::Alt),

            "pause" => Ok(Key::Pause),
            "capslock" => Ok(Key::CapsLock),

            "esc" | "escape" => Ok(Key::Escape),

            "space" => Ok(Key::Space),

            "pageup" => Ok(Key::PageUp),
            "pagedown" => Ok(Key::PageDown),

            "end" => Ok(Key::End),
            "home" => Ok(Key::Home),

            "left" => Ok(Key::Left),

            "right" => Ok(Key::Right),

            "up" => Ok(Key::Up),

            "down" => Ok(Key::Down),

            "win" | "windows" => Ok(Key::LeftWin),
            "menu" => Ok(Key::Menu),

            "insert" => Ok(Key::Insert),
            "delete" => Ok(Key::Delete),

            "0" => Ok(Key::Num0),
            "1" => Ok(Key::Num1),
            "2" => Ok(Key::Num2),
            "3" => Ok(Key::Num3),
            "4" => Ok(Key::Num4),
            "5" => Ok(Key::Num5),
            "6" => Ok(Key::Num6),
            "7" => Ok(Key::Num7),
            "8" => Ok(Key::Num8),
            "9" => Ok(Key::Num9),

            "a" => Ok(Key::A),
            "b" => Ok(Key::B),
            "c" => Ok(Key::C),
            "d" => Ok(Key::D),
            "e" => Ok(Key::E),
            "f" => Ok(Key::F),
            "g" => Ok(Key::G),
            "h" => Ok(Key::H),
            "i" => Ok(Key::I),
            "j" => Ok(Key::J),
            "k" => Ok(Key::K),
            "l" => Ok(Key::L),
            "m" => Ok(Key::M),
            "n" => Ok(Key::N),
            "o" => Ok(Key::O),
            "p" => Ok(Key::P),
            "q" => Ok(Key::Q),
            "r" => Ok(Key::R),
            "s" => Ok(Key::S),
            "t" => Ok(Key::T),
            "u" => Ok(Key::U),
            "v" => Ok(Key::V),
            "w" => Ok(Key::W),
            "x" => Ok(Key::X),
            "y" => Ok(Key::Y),
            "z" => Ok(Key::Z),

            "multiply" | "mul" => Ok(Key::Multiply),
            "add" => Ok(Key::Add),
            "subtract" | "sub" => Ok(Key::Subtract),
            "decimal" | "dec" => Ok(Key::Decimal),
            "divide" | "div" => Ok(Key::Divide),

            value if value.starts_with("f") => {
                let num = value[1..]
                    .parse::<u8>()
                    .map_err(|_| value.to_string())
                    .unwrap();

                match num {
                    1 => Ok(Key::F1),
                    2 => Ok(Key::F2),
                    3 => Ok(Key::F3),
                    4 => Ok(Key::F4),
                    5 => Ok(Key::F5),
                    6 => Ok(Key::F6),
                    7 => Ok(Key::F7),
                    8 => Ok(Key::F8),
                    9 => Ok(Key::F9),
                    10 => Ok(Key::F10),
                    11 => Ok(Key::F11),
                    12 => Ok(Key::F12),
                    13 => Ok(Key::F13),
                    14 => Ok(Key::F14),
                    15 => Ok(Key::F15),
                    16 => Ok(Key::F16),
                    17 => Ok(Key::F17),
                    18 => Ok(Key::F18),
                    19 => Ok(Key::F19),
                    20 => Ok(Key::F20),
                    21 => Ok(Key::F21),
                    22 => Ok(Key::F22),
                    23 => Ok(Key::F23),
                    24 => Ok(Key::F24),

                    _ => Err(value.to_string()),
                }
            }

            _ => Err(format!("unknown key: {}", value)),
        }
    }
}
