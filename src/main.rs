use sdl_rust::engine;

fn main() {
    let mut engine = engine::Engine::new();

    engine.create_window("arigato", 640, 360).unwrap();

    engine.on_draw(|ctx| {
        let r = ctx.renderer.as_ref().unwrap();
        r.set_draw_color(100, 100, 100, 255);
        r.draw_rect(10.0, 0.0, 100.0, 100.0);
    });
    engine.run();
}
