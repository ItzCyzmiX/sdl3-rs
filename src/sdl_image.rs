use std::ffi::{c_char, c_void};

use crate::sdl::{SDL_FRect, SDL_Renderer};

#[repr(C)]
pub struct SDL_Surface {
    flags: u32,
    format: i32,
    w: i32,
    h: i32,
    pitch: i32,
    pixels: *mut c_void,
    refcount: i32,
    pub reserved: *mut c_void,
}

#[repr(C)]
pub struct SDL_Texture {
    format: i32,
    w: i32,
    h: i32,
    refcount: i32,
}

#[link(name = "SDL3_image")]
unsafe extern "C" {
    pub fn IMG_Load(file: *const c_char) -> *mut SDL_Surface;

    pub fn SDL_CreateTextureFromSurface(
        renderer: *mut SDL_Renderer,
        surface: *mut SDL_Surface,
    ) -> *mut SDL_Texture;

    pub fn SDL_RenderTexture(
        renderer: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        srcrect: *const SDL_FRect,
        dstrect: *const SDL_FRect,
    ) -> bool;

    pub fn SDL_DestroySurface(surface: *mut SDL_Surface);

    pub fn SDL_DestroyTexture(texture: *mut SDL_Texture);
}
