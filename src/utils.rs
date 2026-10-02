use crate::sdl::SDL_GetError;
use std::ffi::CStr;

pub(crate) fn sdl_error() -> String {
    unsafe {
        CStr::from_ptr(SDL_GetError())
            .to_string_lossy()
            .into_owned()
    }
}
