use std::sync::mpsc;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::renderer::state::RenderState;

mod state;
mod mesh;

pub enum RendererCommand {
    Quit,
}

pub enum RendererEvent {
    Closed,
}

pub struct RendererHandle {
    pub tx: mpsc::Sender<RendererCommand>,
    pub rx: mpsc::Receiver<RendererEvent>,
}

pub struct Renderer {
    window: Option<Window>,
    cmd_rx: mpsc::Receiver<RendererCommand>,
    evt_tx: mpsc::Sender<RendererEvent>,

    state: Option<RenderState>,
}

impl Renderer {
    pub fn new() -> (RendererHandle, Renderer) {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (evt_tx, evt_rx) = mpsc::channel();

        let handle = RendererHandle { tx: cmd_tx, rx: evt_rx };
        let renderer = Renderer {
            window: None,
            cmd_rx,
            evt_tx,
            state: None
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
        let window = event_loop.create_window(Window::default_attributes()).unwrap();
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
                    state.mark_recreate(size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.state {
                    state.maybe_recreate();
                    state.draw_frame();
                    state.maybe_recreate();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        while let Ok(cmd) = self.cmd_rx.try_recv() {
            match cmd {
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

    fn exiting(&mut self, _: &ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            state.destroy();
        }
    }
}
