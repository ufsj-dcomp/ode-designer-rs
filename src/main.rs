#![feature(map_many_mut)]
#![feature(try_blocks)]
use app::App;

mod message;

#[cfg(feature = "imgui")]
mod imgui;

pub mod app;
mod id_gen;
pub mod nodes;
pub mod pins;

fn main() {
    color_eyre::install().unwrap();

    let app = App::new();
    /* if let Ok(file) = std::fs::read("model.json") {
        let model = odeir::ffi::model_from_json(&file);
        for (id, n) in model.nodes {
            match n {
                odeir::Node::Population { id, name, related_constant_name, links } => {
                    app.add_node(Node::new_population(n.name(), Population::new()));
                },
                odeir::Node::Combinator => {
                    app.add_node(Node::new_combinator(n.name(), Combinator::default()));
                },

            }
        }
        for c in model.constants {
            app.add_node(Node::new_constant(&c.name, Constant::new(c.value)));
        }
    } else */
    // {
    //     app.add_node(Node::new_constant("K", Constant::new(40.0)));
    //     app.add_node(Node::new_combinator("comb", Combinator::default()));
    //     app.add_node(Node::new_combinator("comb2", Combinator::default()));
    // }

    #[cfg(feature = "imgui")]
    crate::imgui::main(app).unwrap();
}
