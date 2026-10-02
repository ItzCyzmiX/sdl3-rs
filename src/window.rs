use crate::sdl::SDL_DestroyWindow;

#[derive(Debug)]
pub struct Window {
    pub(crate) sdl_window: *mut crate::sdl::SDL_Window,
    pub title: String,
    pub width: i32,
    pub height: i32,
}

impl Window {
    fn kill(&self) {
        unsafe {
            SDL_DestroyWindow(self.sdl_window);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.kill();
    }
}
