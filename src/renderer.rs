use crate::engine::Sdl3Result;
use crate::enums::{Color, DrawMode};
use crate::sdl::{
    SDL_DestroyRenderer, SDL_FRect, SDL_RenderFillRect, SDL_RenderLine, SDL_RenderPoint,
    SDL_RenderRect, SDL_SetRenderDrawColor,
};
use crate::sdl_image::SDL_RenderTexture;
use crate::shapes::{Point, Rect};
use crate::sprite::{Sprite, SpriteManager};
use crate::utils::sdl_error;
#[derive(Debug)]
pub struct Renderer {
    pub(crate) sdl_renderer: *mut crate::sdl::SDL_Renderer,
    pub(crate) sprite_manager: SpriteManager,
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

    pub fn draw_point(&self, point: Point) -> Sdl3Result {
        unsafe {
            if !SDL_RenderPoint(self.sdl_renderer, point.x, point.y) {
                return Err(sdl_error());
            }
            Ok(())
        }
    }

    pub fn draw_line(&self, point1: Point, point2: Point) -> Sdl3Result {
        unsafe {
            if !SDL_RenderLine(self.sdl_renderer, point1.x, point1.y, point2.x, point2.y) {
                return Err(sdl_error());
            }
            Ok(())
        }
    }

    pub fn draw_sprite(&self, sprite: &mut Sprite) -> Sdl3Result {
        unsafe {
            if !sprite.raw_texture().is_some() {
                return Ok(());
            }

            let dstrect = SDL_FRect {
                x: sprite.dest_rect.x,
                y: sprite.dest_rect.y,
                w: sprite.dest_rect.w,
                h: sprite.dest_rect.h,
            };

            if !SDL_RenderTexture(
                self.sdl_renderer,
                sprite.raw_texture().unwrap(),
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
            ) {
                return Err(sdl_error());
            }

            Ok(())
        }
    }

    pub fn set_draw_color(&self, c: Color) -> Sdl3Result {
        unsafe {
            if !SDL_SetRenderDrawColor(self.sdl_renderer, c.r, c.g, c.b, c.a) {
                return Err(sdl_error());
            }
            Ok(())
        }
    }
}
