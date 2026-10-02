use std::ops::BitOr;

pub enum DrawMode {
    Filled,
    Outlined,
}

macro_rules! keys {
    ($($name:ident = $val:literal),* $(,)?) => {
        #[repr(u32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Keys {
            $($name = $val),*
        }

        impl From<u32> for Keys {
            fn from(v: u32) -> Keys {
                match v {
                    $($val => Keys::$name,)*
                    _ => Keys::UNKNOWN,
                }
            }
        }
    };
}

keys! {
    RETURN = 0x0000000D,
    ESCAPE = 0x0000001B,
    BACKSPACE = 0x00000008,
    TAB = 0x00000009,
    SPACE = 0x00000020,
    EXCLAIM = 0x00000021,
    DBLAPOSTROPHE = 0x00000022,
    HASH = 0x00000023,
    DOLLAR = 0x00000024,
    PERCENT = 0x00000025,
    AMPERSAND = 0x00000026,
    APOSTROPHE = 0x00000027,
    LEFTPAREN = 0x00000028,
    RIGHTPAREN = 0x00000029,
    ASTERISK = 0x0000002A,
    PLUS = 0x0000002B,
    COMMA = 0x0000002C,
    MINUS = 0x0000002D,
    PERIOD = 0x0000002E,
    SLASH = 0x0000002F,

    Num0 = 0x00000030,
    Num1 = 0x00000031,
    Num2 = 0x00000032,
    Num3 = 0x00000033,
    Num4 = 0x00000034,
    Num5 = 0x00000035,
    Num6 = 0x00000036,
    Num7 = 0x00000037,
    Num8 = 0x00000038,
    Num9 = 0x00000039,

    COLON = 0x0000003A,
    SEMICOLON = 0x0000003B,
    LESS = 0x0000003C,
    EQUALS = 0x0000003D,
    GREATER = 0x0000003E,
    QUESTION = 0x0000003F,
    AT = 0x00000040,

    BACKSLASH = 0x0000005C,
    CARET = 0x0000005E,
    UNDERSCORE = 0x0000005F,
    GRAVE = 0x00000060,

    A = 0x00000061,
    B = 0x00000062,
    C = 0x00000063,
    D = 0x00000064,
    E = 0x00000065,
    F = 0x00000066,
    G = 0x00000067,
    H = 0x00000068,
    I = 0x00000069,
    J = 0x0000006A,
    K = 0x0000006B,
    L = 0x0000006C,
    M = 0x0000006D,
    N = 0x0000006E,
    O = 0x0000006F,
    P = 0x00000070,
    Q = 0x00000071,
    R = 0x00000072,
    S = 0x00000073,
    T = 0x00000074,
    U = 0x00000075,
    V = 0x00000076,
    W = 0x00000077,
    X = 0x00000078,
    Y = 0x00000079,
    Z = 0x0000007A,

    LEFTBRACE = 0x0000007B,
    RIGHTBRACE = 0x0000007D,
    LEFTBRACKET = 0x0000005B,
    RIGHTBRACKET = 0x0000005D,

    PIPE = 0x0000007C,
    TILDE = 0x0000007E,
    DELETE = 0x0000007F,
    PLUSMINUS = 0x000000B1,
    CAPSLOCK = 0x40000039,

    F1 = 0x4000003A,
    F2 = 0x4000003B,
    F3 = 0x4000003C,
    F4 = 0x4000003D,
    F5 = 0x4000003E,
    F6 = 0x4000003F,
    F7 = 0x40000040,
    F8 = 0x40000041,
    F9 = 0x40000042,
    F10 = 0x40000043,
    F11 = 0x40000044,
    F12 = 0x40000045,

    RIGHT = 0x4000004F,
    LEFT = 0x40000050,
    DOWN = 0x40000051,
    UP = 0x40000052,

    LCTRL = 0x400000E0,
    LSHIFT = 0x400000E1,
    LALT = 0x400000E2,

    RCTRL = 0x400000E4,
    RSHIFT = 0x400000E5,
    RALT = 0x400000E6,

    UNKNOWN = 0x00000000,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowFlags {
    DEFAULT = 0,
    FULLSCREEN = 0x0000000000000001,
    HIDDEN = 0x0000000000000008,
    BORDERLESS = 0x0000000000000010,
    RESIZABLE = 0x0000000000000020,
    MINIMIZED = 0x0000000000000040,
    MAXIMIZED = 0x0000000000000080,
    ALWAYS_ON_TOP = 0x0000000000010000,
}

impl BitOr for WindowFlags {
    type Output = u64;
    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u64) | (rhs as u64)
    }
}

impl BitOr<WindowFlags> for u64 {
    type Output = u64;
    fn bitor(self, rhs: WindowFlags) -> Self::Output {
        self | (rhs as u64)
    }
}

impl From<WindowFlags> for u64 {
    fn from(value: WindowFlags) -> Self {
        value as u64
    }
}

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other,
}

impl Into<MouseButton> for u8 {
    fn into(self) -> MouseButton {
        match self {
            1 => MouseButton::Left,
            2 => MouseButton::Middle,
            3 => MouseButton::Right,
            _ => MouseButton::Other,
        }
    }
}

#[non_exhaustive]
pub enum Event {
    Quit,
    KeyPressed(Keys),
    KeyReleased(Keys),
    MouseMoved(f32, f32, f32, f32),
    MousePressed(MouseButton, u8, f32, f32),
    MouseReleased(MouseButton, f32, f32),
}
