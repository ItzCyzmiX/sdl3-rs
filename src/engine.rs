use crate::component::Component;
use crate::ctx::Ctx;
use crate::enums::{Event, Keys};
use crate::sdl::{
    SDL_CreateRenderer, SDL_CreateWindow, SDL_DestroyWindow, SDL_EVENT_KEY_DOWN, SDL_EVENT_KEY_UP,
    SDL_EVENT_MOUSE_BUTTON_DOWN, SDL_EVENT_MOUSE_BUTTON_UP, SDL_EVENT_MOUSE_MOTION, SDL_EVENT_QUIT,
    SDL_Event, SDL_GetError, SDL_GetTicks, SDL_Init, SDL_PollEvent, SDL_Quit, SDL_RenderClear,
    SDL_RenderPresent, SDL_SetRenderDrawColor,
};

use crate::sprite::SpriteManager;
use crate::utils::sdl_error;
use crate::{renderer::Renderer, window::Window};
use std::collections::HashMap;
use std::ffi::{CStr, CString};

pub type Sdl3Result = Result<(), String>;

pub struct Engine<T> {
    renderer: Option<Renderer>,
    window: Option<Window>,
    components: Vec<Box<dyn Component<T>>>,
    pub state: T,
    pub ctx: Ctx,
}

impl<T> Engine<T> {
    pub fn add<C: Component<T> + 'static>(&mut self) {
        let gfx = self
            .renderer
            .as_mut()
            .expect("Call create_window before adding components");
        let component = C::create(&mut self.state, &mut self.ctx, gfx);
        self.components.push(Box::new(component));
    }
}

impl<T> Drop for Engine<T> {
    fn drop(&mut self) {
        while self.components.len() > 0 {
            self.components
                .pop()
                .unwrap()
                .kill(&mut self.state, &mut self.ctx)
                .expect("Couldnt kill component");
        }

        self.renderer.take();
        self.window.take();
        unsafe {
            SDL_Quit();
        }
    }
}

impl<T> Engine<T> {
    pub fn new(state: T) -> Result<Engine<T>, String> {
        unsafe {
            if !SDL_Init(0x20 | 0x400) {
                return Err(CStr::from_ptr(SDL_GetError())
                    .to_string_lossy()
                    .into_owned());
            }
        };

        Ok(Engine {
            components: Vec::new(),
            state: state,
            window: None,
            renderer: None,
            ctx: Ctx::new(),
        })
    }

    pub fn create_window(
        &mut self,
        title: &str,
        width: u32,
        height: u32,
        flags: u64,
    ) -> Sdl3Result {
        let title_c = CString::new(title).unwrap();

        unsafe {
            let sdl_window =
                SDL_CreateWindow(title_c.as_ptr(), width as i32, height as i32, flags as u64);
            if sdl_window.is_null() {
                return Err(sdl_error());
            }

            let sdl_renderer = SDL_CreateRenderer(sdl_window, std::ptr::null());
            if sdl_renderer.is_null() {
                SDL_DestroyWindow(sdl_window);
                return Err(sdl_error());
            }

            self.window = Some(Window {
                sdl_window,
                title: title.to_string(),
                width: width as i32,
                height: height as i32,
            });
            self.renderer = Some(Renderer {
                sdl_renderer,
                sprite_manager: SpriteManager {
                    textures: HashMap::new(),
                },
            });
        }
        Ok(())
    }

    fn signal_event_to_components(&mut self, event: Event) -> Sdl3Result {
        for comp in self.components.iter_mut() {
            comp.on(&mut self.state, &mut self.ctx, &event)?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Sdl3Result {
        unsafe {
            let Some(sdl_renderer) = self.renderer.as_ref().map(|r| r.sdl_renderer) else {
                return Err(sdl_error());
            };
            if sdl_renderer.is_null() {
                return Err("Window not created!".to_string());
            }

            let Some(sdl_window) = self.window.as_ref().map(|r| r.sdl_window) else {
                return Err(sdl_error());
            };
            if sdl_window.is_null() {
                return Err("Window not created!".to_string());
            }

            let mut last_time = SDL_GetTicks();

            while self.ctx.running {
                let now = SDL_GetTicks();
                self.ctx.dt = (now - last_time) as f32 / 1000.0;
                last_time = now;

                let mut event: SDL_Event = core::mem::zeroed();
                while SDL_PollEvent(&mut event) {
                    match event.r#type {
                        SDL_EVENT_QUIT => {
                            self.signal_event_to_components(Event::Quit)?;
                            self.ctx.running = false;
                            break;
                        }

                        SDL_EVENT_KEY_DOWN => {
                            if !self.ctx.keys_pressed.contains(&event.key.key) {
                                self.signal_event_to_components(Event::KeyPressed(Keys::from(
                                    event.key.key,
                                )))?;
                            }
                            self.ctx.keys_pressed.insert(event.key.key);
                        }

                        SDL_EVENT_KEY_UP => {
                            self.signal_event_to_components(Event::KeyReleased(Keys::from(
                                event.key.key,
                            )))?;
                            self.ctx.keys_pressed.remove(&event.key.key);
                        }

                        SDL_EVENT_MOUSE_MOTION => {
                            self.ctx.mouse_pos = (event.motion.x, event.motion.y);
                            self.signal_event_to_components(Event::MouseMoved(
                                event.motion.x,
                                event.motion.y,
                                event.motion.xrel,
                                event.motion.yrel,
                            ))?;
                        }

                        SDL_EVENT_MOUSE_BUTTON_DOWN => {
                            self.ctx.button_pressed.insert(event.button.button);

                            self.signal_event_to_components(Event::MousePressed(
                                event.button.button.into(),
                                event.button.clicks,
                                event.button.x,
                                event.button.y,
                            ))?;
                        }

                        SDL_EVENT_MOUSE_BUTTON_UP => {
                            self.ctx.button_pressed.remove(&event.button.button);

                            self.signal_event_to_components(Event::MouseReleased(
                                event.button.button.into(),
                                event.button.x,
                                event.button.y,
                            ))?;
                        }

                        _ => {}
                    }
                }

                if !self.ctx.running {
                    break;
                }

                for component in self.components.iter_mut() {
                    component.update(&mut self.state, &mut self.ctx)?;
                }

                if !SDL_SetRenderDrawColor(sdl_renderer, 0, 0, 0, 255) {
                    return Err(sdl_error());
                }
                if !SDL_RenderClear(sdl_renderer) {
                    return Err(sdl_error());
                }
                for component in self.components.iter_mut() {
                    component.draw(&mut self.state, &mut self.renderer.as_mut().unwrap())?;
                }
                if !SDL_RenderPresent(sdl_renderer) {
                    return Err(sdl_error());
                }
            }
        }
        Ok(())
    }
}
