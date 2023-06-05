
mod matrix;
use matrix::Matrix;

mod menu;
use menu::Menu;

fn main() {
    let mut menu = Menu::new(); menu.run();
}
