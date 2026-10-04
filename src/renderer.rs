use crate::engine::Sdl3Result;
use crate::enums::DrawMode;
use crate::sdl::{
    SDL_DestroyRenderer, SDL_FRect, SDL_RenderFillRect, SDL_RenderRect, SDL_SetRenderDrawColor,
};
use crate::sdl_image::{SDL_CreateTextureFromSurface, SDL_RenderTexture};
use crate::shapes::Rect;
use crate::sprite::Sprite;
use crate::utils::sdl_error;
#[derive(Debug)]
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
            let sdl_rect = SDL_FRect {
                x: rect.x,
                y: rect.y,
                w: rect.w,
                h: rect.h,
            };
            match fill {
                DrawMode::Filled => {
                    if !SDL_RenderFillRect(self.sdl_renderer, &sdl_rect) {
                        return Err(sdl_error());
                    }
                }
                DrawMode::Outlined => {
                    if !SDL_RenderRect(self.sdl_renderer, &sdl_rect) {
                        return Err(sdl_error());
                    }
                }
            }

            Ok(())
        }
    }

    pub fn draw_sprite(&self, sprite: &mut Sprite) -> Sdl3Result {
        unsafe {
            if sprite.sdl_texture.is_null() {
                return Err(sdl_error());
            }

            let dstrect = SDL_FRect {
                x: sprite.dest_rect.x,
                y: sprite.dest_rect.y,
                w: sprite.dest_rect.w,
                h: sprite.dest_rect.h,
            };

            SDL_RenderTexture(
                self.sdl_renderer,
                sprite.sdl_texture,
                if let Some(source_rect) = sprite.source_rect {
                    &SDL_FRect {
                        x: source_rect.x,
                        y: source_rect.y,
                        w: source_rect.w,
                        h: source_rect.h,
                    }
                } else {
                    core::mem::zeroed()
                },
                &dstrect,
            );

            Ok(())
        }
    }

    pub fn set_draw_color(&self, r: u8, g: u8, b: u8, a: u8) -> Sdl3Result {
        unsafe {
            if !SDL_SetRenderDrawColor(self.sdl_renderer, r, g, b, a) {
                return Err(sdl_error());
            }
            Ok(())
        }
    }
}
