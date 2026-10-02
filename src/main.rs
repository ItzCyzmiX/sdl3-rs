use sdl_rust::{
    component::Component,
    ctx::Ctx,
    engine,
    enums::{DrawMode, Event, Keys, WindowFlags},
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
    fn on(&mut self, _state: &mut GameState, _ctx: &mut Ctx, event: &Event) -> engine::Sdl3Result {
        match event {
            Event::KeyPressed(key) => {
                if *key == Keys::SPACE {
                    println!("Hi");
                }
            }
            Event::MouseMoved(_, _, relx, rely) => {
                println!("{}, {}", relx, rely);
            }
            _ => {}
        };
        Ok(())
    }
    fn draw(&mut self, _state: &GameState, gfx: &mut Renderer) -> engine::Sdl3Result {
        gfx.set_draw_color(100, 100, 100, 255)?;
        gfx.draw_rect(self.rect, DrawMode::Filled)?;

        Ok(())
    }

    fn update(&mut self, _state: &mut GameState, ctx: &mut Ctx) -> engine::Sdl3Result {
        if ctx.is_key_held(Keys::D) {
            self.rect.x += 100.0 * ctx.dt;
        }
        if ctx.is_key_held(Keys::ESCAPE) {
            ctx.exit();
        }
        Ok(())
    }
}

fn main() -> engine::Sdl3Result {
    let mut engine = engine::Engine::new(GameState {}).unwrap();

    engine.create_window(
        "arigato",
        640,
        360,
        WindowFlags::DEFAULT | WindowFlags::FULLSCREEN,
    )?;

    engine.add(Player::new());
    engine.run()?;

    Ok(())
}
