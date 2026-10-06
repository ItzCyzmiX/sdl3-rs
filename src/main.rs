use hydra::{
    component::Component, ctx::Ctx, engine, enums::{DrawMode, Event, Keys, MouseButton, WindowFlags}, renderer::Renderer, shapes::{Point, Rect}, sprite::Sprite,
};

struct GameState {}

struct Player {
    rect: Rect,
    sprite: Sprite,
    sprite2: Sprite,
}

impl Component<GameState> for Player {
    fn create(_state: &mut GameState, _ctx: &mut Ctx, gfx: &mut Renderer) -> Self {
        Player {
            rect: Rect::new(0.0, 0.0, 100.0, 100.0),
            sprite: Sprite::new("ultron.jpg", Rect::new(0.0, 0.0, 100.0, 100.0), None, gfx)
                .unwrap(),
            sprite2: Sprite::new("ultron.jpg", Rect::new(100.0, 0.0, 100.0, 100.0), None, gfx)
                .unwrap(),
            
        }
    }

    fn on(&mut self, _state: &mut GameState, ctx: &mut Ctx, event: &Event) -> engine::Sdl3Result {
        match event {
            Event::KeyReleased(key) => match *key {
                Keys::SPACE => {
                    self.sprite.kill();
                    self.sprite2.kill();
                }
                Keys::ESCAPE => ctx.exit(),
                _ => {}
            },

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
        gfx.draw_sprite(&mut self.sprite2)?;

        gfx.draw_line(&Point { x: 0., y: 0. }, &Point { x: 100., y: 100. })?;

        Ok(())
    }

    fn update(&mut self, _state: &mut GameState, ctx: &mut Ctx) -> engine::Sdl3Result {
        if ctx.is_key_held(Keys::D) {
            self.rect.x += 100.0 * ctx.dt;
            self.sprite.dest_rect.x += 10.0 * ctx.dt;
        }

        if ctx.is_mouse_held(MouseButton::Left) {
            self.sprite.dest_rect.x = ctx.get_mouse_position().0;
            self.sprite.dest_rect.y = ctx.get_mouse_position().1;
        }

        Ok(())
    }

    fn kill(&mut self, _state: &mut GameState, _ctx: &mut Ctx) -> engine::Sdl3Result {        
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
