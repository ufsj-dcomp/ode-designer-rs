use app::App;
use nodes::{Node, Constant};

mod message;


#[cfg(feature="egui")]
mod egui;

#[cfg(feature="imgui")]
mod imgui;

pub mod app;
pub mod nodes;

fn main() {
    color_eyre::install().unwrap();
    let mut app = App::new();
    app.add_node(Node::new_constant("K", Constant{}));

    #[cfg(feature="egui")]
    crate::egui::main(app).unwrap();

    #[cfg(feature = "imgui")]
    crate::imgui::main(app).unwrap();
}
