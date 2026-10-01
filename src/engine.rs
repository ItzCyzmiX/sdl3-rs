use std::ffi::CString;

use crate::sdl::{
    SDL_CreateRenderer, SDL_CreateWindow, SDL_DestroyRenderer, SDL_DestroyWindow, SDL_Event,
    SDL_FRect, SDL_GetTicks, SDL_Init, SDL_PollEvent, SDL_RenderClear, SDL_RenderPresent,
    SDL_RenderRect, SDL_SetRenderDrawColor,
};

#[derive(Debug, Clone)]
pub struct Ctx {
    pub renderer: Option<Renderer>,
    pub window: Option<Window>,
}

pub struct Engine {
    running: bool,
    _on_draws: Vec<fn(&mut Ctx) -> ()>,
    _on_updates: Vec<fn(&mut Ctx, f32) -> ()>,
    pub ctx: Ctx,
}

impl Engine {
    pub fn new() -> Engine {
        unsafe {
            SDL_Init(0x20);
        };

        Engine {
            running: false,
            _on_draws: Vec::new(),
            _on_updates: Vec::new(),
            ctx: Ctx {
                window: None,
                renderer: None,
            },
        }
    }

    pub fn create_window(
        &mut self,
        title: &'static str,
        width: i32,
        height: i32,
    ) -> Result<(), ()> {
        let title_c = CString::new(title).unwrap();

        unsafe {
            let sdl_window = SDL_CreateWindow(title_c.as_ptr(), width, height, 0);
            if sdl_window.is_null() {
                eprintln!("couldnt create window");
                return Err(());
            }

            let sdl_renderer = SDL_CreateRenderer(sdl_window, std::ptr::null());
            if sdl_renderer.is_null() {
                eprintln!("couldnt create renderer");
                SDL_DestroyWindow(sdl_window);
                return Err(());
            }

            self.ctx.window = Some(Window {
                sdl_window,
                title,
                width,
                height,
            });
            self.ctx.renderer = Some(Renderer { sdl_renderer });
        }
        Ok(())
    }

    pub fn on_update(&mut self, f: fn(&mut Ctx, f32) -> ()) {
        self._on_updates.push(f);
    }

    pub fn on_draw(&mut self, f: fn(&mut Ctx) -> ()) {
        self._on_draws.push(f);
    }

    pub fn run(&mut self) {
        let Some(sdl_renderer) = self.ctx.renderer.as_ref().map(|r| r.sdl_renderer) else {
            return;
        };
        if sdl_renderer.is_null() {
            return;
        }

        let Some(sdl_window) = self.ctx.window.as_ref().map(|r| r.sdl_window) else {
            return;
        };
        if sdl_window.is_null() {
            return;
        }

        self.running = true;

        let mut last_time = unsafe { SDL_GetTicks() };
        while self.running {
            unsafe {
                let now = SDL_GetTicks();
                let dt = (now - last_time) as f32 / 1000.0;
                last_time = now;

                let mut event: SDL_Event = core::mem::zeroed();
                while SDL_PollEvent(&mut event) {
                    match event.r#type {
                        0x100 => {
                            self.running = false;
                            break;
                        }
                        _ => {}
                    }
                }

                for f in &self._on_updates {
                    f(&mut self.ctx, dt);
                }

                SDL_SetRenderDrawColor(sdl_renderer, 0, 0, 0, 255);
                SDL_RenderClear(sdl_renderer);
                for f in &self._on_draws {
                    f(&mut self.ctx);
                }
                SDL_RenderPresent(sdl_renderer);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Renderer {
    sdl_renderer: *mut crate::sdl::SDL_Renderer,
}

#[derive(Debug, Clone)]
pub struct Window {
    sdl_window: *mut crate::sdl::SDL_Window,
    pub title: &'static str,
    pub width: i32,
    pub height: i32,
}

impl Window {
    fn kill(&self) {
        unsafe {
            SDL_DestroyWindow(self.sdl_window);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.kill();
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.kill();
    }
}

impl Renderer {
    fn kill(&self) {
        unsafe {
            SDL_DestroyRenderer(self.sdl_renderer);
        }
    }

    pub fn draw_rect(&self, x: f32, y: f32, w: f32, h: f32) {
        unsafe {
            SDL_RenderRect(
                self.sdl_renderer,
                std::ptr::from_ref(&SDL_FRect { x, y, w, h }),
            );
        }
    }

    pub fn set_draw_color(&self, r: u8, g: u8, b: u8, a: u8) {
        unsafe {
            SDL_SetRenderDrawColor(self.sdl_renderer, r, g, b, a);
        }
    }
}
