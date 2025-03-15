use gui::window::AppWindow;

mod gui;
mod config;

const WIDTH: usize = 640;
const HEIGHT: usize = 460;

fn main() {
    config::main();
    let window: AppWindow = AppWindow::new(WIDTH, HEIGHT);
    window.display();
}
