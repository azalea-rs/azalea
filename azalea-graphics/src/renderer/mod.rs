use std::{sync::Arc, time::Instant};

use azalea::{core::position::ChunkPos, world::Chunk};
use crossbeam::channel::{self, Receiver, Sender, unbounded};
use parking_lot::RwLock;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window, WindowId},
};

use crate::renderer::{mesher::LocalChunk, state::RenderState};

mod camera;
mod mesh;
pub(crate) mod mesher;
mod render_world;
mod state;

pub enum RendererCommand {
    ChunkUpdate(ChunkPos, LocalChunk),
    Quit,
}

pub enum RendererEvent {
    Closed,
}

#[derive(Clone)]
pub struct RendererHandle {
    pub tx: Sender<RendererCommand>,
    pub rx: Receiver<RendererEvent>,
}

pub struct Renderer {
    window: Option<Window>,
    cmd_rx: Receiver<RendererCommand>,
    evt_tx: Sender<RendererEvent>,

    state: Option<RenderState>,

    last_frame_time: Instant,
}

impl Renderer {
    pub fn new() -> (RendererHandle, Renderer) {
        let (cmd_tx, cmd_rx) = unbounded();
        let (evt_tx, evt_rx) = unbounded();

        let handle = RendererHandle {
            tx: cmd_tx,
            rx: evt_rx,
        };
        let renderer = Renderer {
            window: None,
            cmd_rx,
            evt_tx,
            state: None,
            last_frame_time: Instant::now(),
        };

        (handle, renderer)
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler for Renderer {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        let size = window.inner_size();

        let window_handle = window.window_handle().unwrap();
        let display_handle = window.display_handle().unwrap();

        let state = RenderState::new(&window_handle, &display_handle, size);
        self.state = Some(state);
        self.window = Some(window);
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            state.destroy();
        }
        self.state = None;
        self.window = None;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                let _ = self.evt_tx.send(RendererEvent::Closed);
            }
            WindowEvent::Resized(size) => {
                if let Some(state) = &mut self.state {
                    state.resize(size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.state {
                    let now = Instant::now();
                    let dt = now - self.last_frame_time;
                    self.last_frame_time = now;

                    state.update(dt);

                    state.maybe_recreate();
                    state.draw_frame();
                    state.maybe_recreate();
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if let (Some(state), PhysicalKey::Code(code)) =
                    (&mut self.state, event.physical_key)
                {
                    state.process_keyboard(code, event.state);

                    if let Some(window) = &self.window
                        && code == KeyCode::Escape
                        && event.state == ElementState::Pressed
                    {
                        let _ = window.set_cursor_grab(CursorGrabMode::None);
                        window.set_cursor_visible(true);
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let Some(state) = &mut self.state {
                    state.handle_mouse_scroll(&delta);
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

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        if let (Some(state), DeviceEvent::MouseMotion { delta }) = (&mut self.state, event) {
            state.handle_mouse(delta.0, delta.1);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        while let Ok(cmd) = self.cmd_rx.try_recv() {
            match cmd {
                RendererCommand::ChunkUpdate(pos, chunk) => {
                    if let Some(state) = &self.state {
                        state.update_chunk(pos, &chunk);
                    }
                }
                RendererCommand::Quit => {
                    event_loop.exit();
                    let _ = self.evt_tx.send(RendererEvent::Closed);
                }
            }
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn exiting(&mut self, el: &ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            state.destroy();
        }
        std::process::exit(0);
    }
}
