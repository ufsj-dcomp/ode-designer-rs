#![feature(map_many_mut)]
#![feature(try_blocks)]
use app::App;
use nodes::{Node, Constant, Combinator, Population};

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
    app.add_node(Node::new_constant("K", Constant::new(40.0)));
    app.add_node(Node::new_combinator("comb", Combinator::default()));
    app.add_node(Node::new_population("pop", Population::default()));

    #[cfg(feature="egui")]
    crate::egui::main(app).unwrap();

    #[cfg(feature = "imgui")]
    crate::imgui::main(app).unwrap();
}
