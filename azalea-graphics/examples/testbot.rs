use azalea_graphics::renderer::Renderer;

fn main() {
    env_logger::init();
    let (_handle, renderer) = Renderer::new();
    renderer.run();
}
