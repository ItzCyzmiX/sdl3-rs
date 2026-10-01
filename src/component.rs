use crate::{ctx::Ctx, engine::Sdl3Result, renderer::Renderer};

pub trait Component<S> {
    fn update(&mut self, _state: &mut S, _ctx: &Ctx) -> Sdl3Result {
        Ok(())
    }
    fn draw(&mut self, _state: &S, _gfx: &mut Renderer) -> Sdl3Result {
        Ok(())
    }
}
