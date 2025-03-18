use gui::window::AppWindow;

mod gui;
mod model;
mod utils;

fn main() {
    let mut window: AppWindow = AppWindow::new();
    window.display();
}
