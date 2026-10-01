use crate::{engine::Sdl3Result, renderer::Renderer};

pub trait Component<S> {
    fn update(&mut self, _state: &mut S, _dt: f32) -> Sdl3Result {
        Ok(())
    }
    fn draw(&mut self, _state: &S, _gfx: &mut Renderer) -> Sdl3Result {
        Ok(())
    }
}
