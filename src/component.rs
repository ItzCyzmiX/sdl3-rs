use crate::{ctx::Ctx, engine::Sdl3Result, enums::Event, renderer::Renderer};

pub trait Component<S> {
    fn on(&mut self, _state: &mut S, _ctx: &mut Ctx, _event: &Event) -> Sdl3Result {
        Ok(())
    }

    fn update(&mut self, _state: &mut S, _ctx: &mut Ctx) -> Sdl3Result {
        Ok(())
    }
    fn draw(&mut self, _state: &S, _gfx: &mut Renderer) -> Sdl3Result {
        Ok(())
    }
}
