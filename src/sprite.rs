use std::{
    collections::HashMap,
    ffi::CString,
    rc::{Rc, Weak},
};

use crate::{
    renderer::Renderer,
    sdl_image::{
        IMG_Load, SDL_CreateTextureFromSurface, SDL_DestroySurface, SDL_DestroyTexture, SDL_Texture,
    },
    shapes::Rect,
    utils::sdl_error,
};

#[derive(Debug)]
pub(crate) struct Texture(*mut SDL_Texture);

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { SDL_DestroyTexture(self.0) };
    }
}

#[derive(Debug)]
pub(crate) struct SpriteManager {
    pub(crate) textures: HashMap<String, Weak<Texture>>,
}

#[derive(Debug)]
pub struct Sprite {
    pub dest_rect: Rect,
    pub source_rect: Option<Rect>,
    pub(crate) texture: Option<Rc<Texture>>,
}

impl Sprite {
    pub fn new(
        path: &str,
        dest_rect: Rect,
        source_rect: Option<Rect>,
        gfx: &mut Renderer,
    ) -> Result<Self, String> {
        if let Some(texture) = gfx
            .sprite_manager
            .textures
            .get(path)
            .and_then(Weak::upgrade)
        {
            return Ok(Sprite {
                dest_rect,
                source_rect,
                texture: Some(texture),
            });
        }

        let c_path = CString::new(path).map_err(|e| e.to_string())?;

        unsafe {
            let surface = IMG_Load(c_path.as_ptr());
            if surface.is_null() {
                return Err(sdl_error());
            }

            let raw = SDL_CreateTextureFromSurface(gfx.sdl_renderer, surface);
            SDL_DestroySurface(surface);

            if raw.is_null() {
                return Err(sdl_error());
            }

            let texture = Rc::new(Texture(raw));
            gfx.sprite_manager
                .textures
                .insert(path.to_string(), Rc::downgrade(&texture));

            Ok(Sprite {
                dest_rect,
                source_rect,
                texture: Some(texture),
            })
        }
    }

    pub fn kill(&mut self) {
        self.texture = None;
    }

    pub fn is_alive(&self) -> bool {
        self.texture.is_some()
    }

    pub fn raw_texture(&self) -> Option<*mut SDL_Texture> {
        self.texture.as_ref().map(|t| t.0)
    }
}
