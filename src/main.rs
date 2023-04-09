use app::App;
use nodes::{Node, Constant};


#[cfg(feature="egui")]
mod egui;

#[cfg(feature="imgui")]
mod imgui;

pub mod app;
pub mod nodes;

fn main() {
    color_eyre::install().unwrap();
    let mut app = App::new();
    app.add_node(Node::constant(Constant{}));

    #[cfg(feature="egui")]
    crate::egui::main(app).unwrap();

    #[cfg(feature = "imgui")]
    crate::imgui::main(app).unwrap();
}
