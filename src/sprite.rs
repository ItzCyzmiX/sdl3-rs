use crate::{
    renderer::Renderer,
    sdl_image::{
        IMG_Load, SDL_CreateTextureFromSurface, SDL_DestroySurface, SDL_Surface, SDL_Texture,
    },
    shapes::Rect,
    utils::sdl_error,
};

#[derive(Debug)]
pub struct Sprite {
    pub(crate) sdl_texture: *mut SDL_Texture,
    pub dest_rect: Rect,
    pub source_rect: Option<Rect>,
}

impl Sprite {
    pub fn new(
        path: &str,
        dest_rect: Rect,
        source_rect: Option<Rect>,
        gfx: &mut Renderer,
    ) -> Result<Self, String> {
        unsafe {
            let surface = IMG_Load(path.as_ptr().cast());

            if surface.is_null() {
                return Err(sdl_error());
            }

            let texture = SDL_CreateTextureFromSurface(gfx.sdl_renderer, surface);

            if texture.is_null() {
                return Err(sdl_error());
            }

            SDL_DestroySurface(surface);

            Ok(Sprite {
                dest_rect: dest_rect,
                source_rect: source_rect,
                sdl_texture: texture,
            })
        }
    }
}
