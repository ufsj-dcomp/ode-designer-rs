#![feature(map_many_mut)]
#![feature(try_blocks)]
#![feature(lazy_cell)]

use core::{initialize_id_generator, style, System};

use core::App;
use imnodes::AttributeFlag;
use nodes::{Expression, NodeInitializer, Term};

mod message;

mod core;

pub mod errors;
pub mod exprtree;
pub mod nodes;
pub mod pins;
pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install().unwrap();

    let mut system = System::make_window("ODE Editor", (1024.0, 768.0));
    // SAFETY: The pointer is valid for the lifetime of the function.
    style::set_eel_style(system.imgui.style_mut());
    let nodesctx = imnodes::Context::new();
    let mut nodeseditor = nodesctx.create_editor();

    // SAFETY: The initialization of this value at the startup code is always
    // safe to do. However, using this value will only ever be safe while the
    // application isn't multithreaded or at least not the GUI part, which
    // honestly doesn't need to be.
    unsafe { initialize_id_generator(nodeseditor.new_identifier_generator()) };

    let _link_detach = nodeseditor.push(AttributeFlag::EnableLinkDetachWithDragClick);
    // let _link_creation = nodeseditor.push(AttributeFlag::EnableLinkCreationOnSnap);

    let mut app = App::new();

    // app.add_node(Term::new_boxed("A".into()));
    // app.add_node(Term::new_boxed("K".into()));
    // app.add_node(Expression::new_boxed("comb".into()));
    // app.add_node(Expression::new_boxed("comb2".into()));

    system.main_loop(move |_, ui| {
        app.draw(ui, &mut nodeseditor);
    });
    Ok(())
}
