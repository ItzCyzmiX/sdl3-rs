use sdl_rust::{
    component::Component,
    ctx::Ctx,
    engine,
    enums::{DrawMode, WindowFlags},
    renderer::Renderer,
    shapes::Rect,
};

struct GameState {}

struct Player {
    rect: Rect,
}

impl Player {
    fn new() -> Self {
        Player {
            rect: Rect {
                x: 0.0,
                y: 0.0,
                w: 100.0,
                h: 100.0,
            },
        }
    }
}

impl Component<GameState> for Player {
    fn draw(&mut self, _state: &GameState, gfx: &mut Renderer) -> engine::Sdl3Result {
        gfx.set_draw_color(100, 100, 100, 255)?;
        gfx.draw_rect(self.rect, DrawMode::Filled)?;

        Ok(())
    }

    fn update(&mut self, _state: &mut GameState, ctx: &Ctx) -> engine::Sdl3Result {
        self.rect.x += 10.0 * ctx.dt;
        Ok(())
    }
}

fn main() -> engine::Sdl3Result {
    let mut engine = engine::Engine::new(GameState {});

    engine.create_window("arigato", 640, 360, WindowFlags::DEFAULT)?;

    engine.add(Player::new());
    engine.run()?;

    Ok(())
}
