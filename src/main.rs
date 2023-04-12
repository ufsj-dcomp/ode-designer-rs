use app::App;
use nodes::{Node, Constant, Combinator};

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
    let node_const = *app.get_node(0).unwrap().outputs()[0].id();
    let node_pop = *app.get_node(2).unwrap().inputs()[0].id();
    app.add_link(&node_const, &node_pop);

    #[cfg(feature="egui")]
    crate::egui::main(app).unwrap();

    #[cfg(feature = "imgui")]
    crate::imgui::main(app).unwrap();
}
