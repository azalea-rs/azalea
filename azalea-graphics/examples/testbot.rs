use std::thread;

use azalea::prelude::*;
use azalea_graphics::{
    plugin::RendererPlugin,
    renderer::{Renderer, RendererHandle},
};
use tokio::runtime::Runtime;

async fn run_azalea(render_handle: RendererHandle) {
    let account = Account::offline("bot");

    ClientBuilder::new()
        .add_plugins(RendererPlugin {
            handle: render_handle,
        })
        .set_handler(handle)
        .start(account, "localhost:33345")
        .await
        .unwrap();
}

fn main() {
    env_logger::init();

    let (handle, renderer) = Renderer::new();
    let azalea_thread = thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(run_azalea(handle));
        println!("exited");
    });

    renderer.run();

    let _ = azalea_thread.join();
}

#[derive(Default, Clone, Component)]
pub struct State;

async fn handle(_client: Client, event: Event, state: State) {}
