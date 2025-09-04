use std::time::Instant;

use azalea::core::position::ChunkPos;
use crossbeam::channel::{Receiver, Sender, unbounded};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window, WindowId},
};

use crate::{
    plugin::BiomeCache,
    renderer::{
        Renderer,
        mesher::{LocalChunk, LocalSection},
    },
};

pub enum RendererEvent {
    Closed,
}

#[derive(Clone)]
pub struct RendererHandle {
    pub tx: Sender<LocalSection>,
    pub rx: Receiver<RendererEvent>,
}

impl RendererHandle {
    pub fn send_chunk(&self, pos: ChunkPos, chunk: LocalChunk, biome_cache: BiomeCache) {
        for mut section in chunk.local_sections(pos) {
            section.biome_cache = biome_cache.clone();
            self.tx.send(section).unwrap();
        }
    }
}

pub struct App {
    window: Option<Window>,
    cmd_rx: Receiver<LocalSection>,
    evt_tx: Sender<RendererEvent>,

    renderer: Option<Renderer>,

    last_frame_time: Instant,
}

impl App {
    pub fn new() -> (RendererHandle, App) {
        let (cmd_tx, cmd_rx) = unbounded();
        let (evt_tx, evt_rx) = unbounded();

        let handle = RendererHandle {
            tx: cmd_tx,
            rx: evt_rx,
        };
        let app = App {
            window: None,
            cmd_rx,
            evt_tx,
            renderer: None,
            last_frame_time: Instant::now(),
        };

        (handle, app)
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        let size = window.inner_size();

        let window_handle = window.window_handle().unwrap();
        let display_handle = window.display_handle().unwrap();

        let renderer = Renderer::new(&window_handle, &display_handle, size, event_loop)
            .expect("Failed to create renderer");
        self.renderer = Some(renderer);
        self.window = Some(window);
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        if let Some(renderer) = &mut self.renderer {
            renderer.destroy();
        }
        self.renderer = None;
        self.window = None;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // Handle egui events first
        let mut egui_consumed = false;
        if let (Some(renderer), Some(window)) = (&mut self.renderer, &self.window) {
            egui_consumed = renderer.handle_egui_event(window, &event);
        }

        // Only handle non-egui events if egui didn't consume them
        if !egui_consumed {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                    let _ = self.evt_tx.send(RendererEvent::Closed);
                }
                WindowEvent::Resized(size) => {
                    if let Some(renderer) = &mut self.renderer {
                        renderer.resize(size);
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Some(renderer) = &mut self.renderer {
                        let now = Instant::now();
                        let dt = now - self.last_frame_time;
                        self.last_frame_time = now;

                        if let Some(window) = &self.window {
                            window.set_title(&format!("{}ms", dt.as_nanos() as f64 / 1_000_000.0));
                        }

                        renderer.update(dt);

                        // Run debug UI
                        if let Some(window) = &self.window {
                            renderer.run_debug_ui(window, dt.as_millis() as f64);
                        }

                        renderer.maybe_recreate();
                        renderer.draw_frame();
                        renderer.maybe_recreate();
                    }
                }

                WindowEvent::KeyboardInput { event, .. } => {
                    if let (Some(renderer), PhysicalKey::Code(code)) =
                        (&mut self.renderer, event.physical_key)
                    {
                        renderer.process_keyboard(code, event.state);

                        if event.state == ElementState::Pressed {
                            match code {
                                KeyCode::Escape => {
                                    if let Some(window) = &self.window {
                                        let _ = window.set_cursor_grab(CursorGrabMode::None);
                                        window.set_cursor_visible(true);
                                    }
                                }
                                KeyCode::F3 => {
                                    renderer.toggle_wireframe();
                                }
                                _ => {}
                            }
                        }
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    if let Some(renderer) = &mut self.renderer {
                        renderer.handle_mouse_scroll(&delta);
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if let Some(window) = &self.window
                        && button == MouseButton::Left
                        && state == ElementState::Pressed
                        && window.set_cursor_grab(CursorGrabMode::Confined).is_ok()
                    {
                        window.set_cursor_visible(false);
                    }
                }
                _ => {}
            }
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        if let (Some(renderer), DeviceEvent::MouseMotion { delta }) = (&mut self.renderer, event) {
            renderer.handle_mouse(delta.0, delta.1);
        }
    }

    fn about_to_wait(&mut self, _el: &ActiveEventLoop) {
        while let Ok(section) = self.cmd_rx.try_recv() {
            if let Some(renderer) = &mut self.renderer {
                renderer.update_section(section);
            }
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn exiting(&mut self, _el: &ActiveEventLoop) {
        if let Some(renderer) = &mut self.renderer {
            renderer.destroy();
        }
    }
}
