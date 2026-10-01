use crate::engine::Sdl3Result;
use crate::enums::DrawMode;
use crate::sdl::{
    SDL_DestroyRenderer, SDL_FRect, SDL_GetError, SDL_RenderFillRect, SDL_RenderRect,
    SDL_SetRenderDrawColor,
};
use crate::shapes::Rect;
#[derive(Debug, Clone)]
pub struct Renderer {
    pub(crate) sdl_renderer: *mut crate::sdl::SDL_Renderer,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.kill();
    }
}

impl Renderer {
    fn kill(&self) {
        unsafe {
            SDL_DestroyRenderer(self.sdl_renderer);
        }
    }

    pub fn draw_rect(&self, rect: Rect, fill: DrawMode) -> Sdl3Result {
        unsafe {
            let rect_ptr = std::ptr::from_ref(&SDL_FRect {
                x: rect.x,
                y: rect.y,
                w: rect.w,
                h: rect.h,
            });
            match fill {
                DrawMode::Filled => {
                    if !SDL_RenderFillRect(self.sdl_renderer, rect_ptr) {
                        return Err(SDL_GetError().cast::<String>().read());
                    }
                }
                DrawMode::Outlined => {
                    if !SDL_RenderRect(self.sdl_renderer, rect_ptr) {
                        return Err(SDL_GetError().cast::<String>().read());
                    }
                }
            }

            Ok(())
        }
    }

    pub fn set_draw_color(&self, r: u8, g: u8, b: u8, a: u8) -> Sdl3Result {
        unsafe {
            if !SDL_SetRenderDrawColor(self.sdl_renderer, r, g, b, a) {
                return Err(SDL_GetError().cast::<String>().read());
            }
            Ok(())
        }
    }
}
