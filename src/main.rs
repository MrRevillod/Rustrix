mod lib {
    pub mod matrix;
    pub mod menu;
    pub mod utils;
}

use lib::matrix::Matrix;
use lib::menu::Menu;
use lib::utils;

fn main() {
    let mut menu = Menu::new();
    menu.run();
}
