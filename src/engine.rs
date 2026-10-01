use crate::component::Component;
use crate::ctx::Ctx;
use crate::enums::Keys;
use crate::sdl::{
    SDL_CreateRenderer, SDL_CreateWindow, SDL_DestroyWindow, SDL_Event, SDL_GetError, SDL_GetTicks,
    SDL_Init, SDL_PollEvent, SDL_Quit, SDL_RenderClear, SDL_RenderPresent, SDL_SetRenderDrawColor,
};

use crate::{enums::WindowFlags, renderer::Renderer, window::Window};
use std::ffi::CString;

pub type Sdl3Result = Result<(), String>;

pub struct Engine<T> {
    running: bool,
    window: Option<Window>,
    renderer: Option<Renderer>,
    components: Vec<Box<dyn Component<T>>>,
    pub state: T,
    pub ctx: Ctx,
}

impl<T> Engine<T> {
    pub fn add(&mut self, component: impl Component<T> + 'static) {
        self.components.push(Box::new(component));
    }
}

impl<T> Drop for Engine<T> {
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}

impl<T> Engine<T> {
    pub fn new(state: T) -> Engine<T> {
        unsafe {
            SDL_Init(0x20);
        };

        Engine {
            running: false,
            components: Vec::new(),
            state: state,
            window: None,
            renderer: None,
            ctx: Ctx::new(),
        }
    }

    pub fn create_window(
        &mut self,
        title: &'static str,
        width: u32,
        height: u32,
        flags: WindowFlags,
    ) -> Sdl3Result {
        let title_c = CString::new(title).unwrap();

        unsafe {
            let sdl_window =
                SDL_CreateWindow(title_c.as_ptr(), width as i32, height as i32, flags as u64);
            if sdl_window.is_null() {
                return Err(SDL_GetError().cast::<String>().read());
            }

            let sdl_renderer = SDL_CreateRenderer(sdl_window, std::ptr::null());
            if sdl_renderer.is_null() {
                SDL_DestroyWindow(sdl_window);
                return Err(SDL_GetError().cast::<String>().read());
            }

            self.window = Some(Window {
                sdl_window,
                title,
                width: width as i32,
                height: height as i32,
            });
            self.renderer = Some(Renderer { sdl_renderer });
        }
        Ok(())
    }

    pub fn run(&mut self) -> Sdl3Result {
        unsafe {
            let Some(sdl_renderer) = self.renderer.as_ref().map(|r| r.sdl_renderer) else {
                return Err(SDL_GetError().cast::<String>().read());
            };
            if sdl_renderer.is_null() {
                return Err(SDL_GetError().cast::<String>().read());
            }

            let Some(sdl_window) = self.window.as_ref().map(|r| r.sdl_window) else {
                return Err(SDL_GetError().cast::<String>().read());
            };
            if sdl_window.is_null() {
                return Err(SDL_GetError().cast::<String>().read());
            }

            self.running = true;

            let mut last_time = SDL_GetTicks();
            while self.running {
                let now = SDL_GetTicks();
                self.ctx.dt = (now - last_time) as f32 / 1000.0;
                last_time = now;

                let mut event: SDL_Event = core::mem::zeroed();
                while SDL_PollEvent(&mut event) {
                    match event.r#type {
                        0x100 => {
                            self.running = false;
                            break;
                        }

                        0x300 => println!("{}", event.key.key == Keys::A as u32),

                        _ => {}
                    }
                }

                for component in self.components.iter_mut() {
                    component.update(&mut self.state, &mut self.ctx)?;
                }

                if !SDL_SetRenderDrawColor(sdl_renderer, 0, 0, 0, 255) {
                    return Err(SDL_GetError().cast::<String>().read());
                }
                if !SDL_RenderClear(sdl_renderer) {
                    return Err(SDL_GetError().cast::<String>().read());
                }
                for component in self.components.iter_mut() {
                    component.draw(&mut self.state, &mut self.renderer.as_mut().unwrap())?;
                }
                if !SDL_RenderPresent(sdl_renderer) {
                    return Err(SDL_GetError().cast::<String>().read());
                }
            }
        }
        Ok(())
    }
}
