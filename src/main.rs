use sdl_rust::{
    component::Component,
    ctx::Ctx,
    engine,
    enums::{DrawMode, Event, Keys, WindowFlags},
    renderer::Renderer,
    shapes::Rect,
    sprite::Sprite,
};

struct GameState {}

struct Player {
    rect: Rect,
    sprite: Sprite,
}

impl Component<GameState> for Player {
    fn create(_state: &mut GameState, _ctx: &mut Ctx, gfx: &mut Renderer) -> Self {
        Player {
            rect: Rect::new(0.0, 0.0, 100.0, 100.0),
            sprite: Sprite::new(
                "ultron.jpg",
                Rect::new(0.0, 0.0, 100.0, 100.0),
                Some(Rect::new(0.0, 0.0, 100.0, 100.0)),
                gfx,
            )
            .unwrap(),
        }
    }

    fn on(&mut self, _state: &mut GameState, _ctx: &mut Ctx, event: &Event) -> engine::Sdl3Result {
        match event {
            Event::KeyReleased(key) => {
                if *key == Keys::SPACE {
                    println!("Hi");
                }
            }

            Event::MouseReleased(btn, x, y) => {
                println!("released {:?}  in ({}, {})", btn, x, y);
            }
            _ => {}
        };
        Ok(())
    }

    fn draw(&mut self, _state: &GameState, gfx: &mut Renderer) -> engine::Sdl3Result {
        gfx.set_draw_color(100, 100, 100, 255)?;
        gfx.draw_rect(self.rect, DrawMode::Filled)?;
        gfx.draw_sprite(&mut self.sprite)?;

        Ok(())
    }

    fn update(&mut self, _state: &mut GameState, ctx: &mut Ctx) -> engine::Sdl3Result {
        if ctx.is_key_held(Keys::D) {
            self.rect.x += 100.0 * ctx.dt;
            self.sprite.dest_rect.x += 10.0 * ctx.dt;
        }
        if ctx.is_key_held(Keys::ESCAPE) {
            ctx.exit();
        }
        Ok(())
    }
}

fn main() -> engine::Sdl3Result {
    let mut engine = engine::Engine::new(GameState {}).unwrap();

    engine.create_window("arigato", 640, 360, WindowFlags::DEFAULT.into())?;
    engine.add::<Player>();

    engine.run()?;

    Ok(())
}
