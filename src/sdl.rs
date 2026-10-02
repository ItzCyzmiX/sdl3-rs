#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_FRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_QuitEvent {
    pub r#type: u32,
    pub reserved: u32,
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct SDL_KeyboardEvent {
    pub r#type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
    pub scancode: u32,
    pub key: u32,
    pub r#mod: u16,
    pub raw: u16,
    pub down: bool,
    pub repeat: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct SDL_MouseMotionEvent {
    pub r#type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
    pub state: u32,
    pub x: f32,
    pub y: f32,
    pub xrel: f32,
    pub yrel: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct SDL_MouseButtonEvent {
    pub r#type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
    pub button: u8,
    pub down: bool,
    pub clicks: u8,
    pub padding: u8,
    pub x: f32,
    pub y: f32,
}

pub const SDL_EVENT_QUIT: u32 = 0x100;
pub const SDL_EVENT_KEY_DOWN: u32 = 0x300;
pub const SDL_EVENT_KEY_UP: u32 = 0x301;
pub const SDL_EVENT_MOUSE_MOTION: u32 = 0x400;
pub const SDL_EVENT_MOUSE_BUTTON_DOWN: u32 = 0x401;
pub const SDL_EVENT_MOUSE_BUTTON_UP: u32 = 0x402;

#[repr(C)]
#[derive(Clone, Copy)]
pub union SDL_Event {
    pub r#type: u32,
    pub quit: SDL_QuitEvent,
    pub key: SDL_KeyboardEvent,
    pub motion: SDL_MouseMotionEvent,
    pub button: SDL_MouseButtonEvent,
    pub padding: [u8; 128],
}

#[repr(C)]
pub struct SDL_Renderer {
    pub _private: [u8; 0],
}

#[repr(C)]
pub struct SDL_Window {
    pub _private: [u8; 0],
}

#[link(name = "SDL3")]
unsafe extern "C" {
    pub fn SDL_Init(flags: u32) -> bool;
    pub fn SDL_CreateWindow(
        title: *const std::ffi::c_char,
        w: i32,
        h: i32,
        flags: u64,
    ) -> *mut SDL_Window;
    pub fn SDL_DestroyWindow(window: *mut SDL_Window);

    pub fn SDL_CreateRenderer(
        window: *mut SDL_Window,
        name: *const std::ffi::c_char,
    ) -> *mut SDL_Renderer;
    pub fn SDL_DestroyRenderer(renderer: *mut SDL_Renderer);

    pub fn SDL_SetRenderDrawColor(renderer: *mut SDL_Renderer, r: u8, g: u8, b: u8, a: u8) -> bool;

    pub fn SDL_RenderClear(renderer: *mut SDL_Renderer) -> bool;
    pub fn SDL_RenderPresent(renderer: *mut SDL_Renderer) -> bool;

    pub fn SDL_RenderRect(renderer: *mut SDL_Renderer, rect: *const SDL_FRect) -> bool;
    pub fn SDL_RenderFillRect(renderer: *mut SDL_Renderer, rect: *const SDL_FRect) -> bool;

    pub fn SDL_PollEvent(event: *mut SDL_Event) -> bool;

    pub fn SDL_GetTicks() -> u64;

    pub fn SDL_GetError() -> *const std::ffi::c_char;

    pub fn SDL_Quit();

}
