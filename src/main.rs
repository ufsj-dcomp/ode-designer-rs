#![feature(map_many_mut)]
#![feature(try_blocks)]
#![feature(int_roundings)]
#![feature(let_chains)]
#![feature(iter_collect_into)]
#![feature(once_cell_try_insert)]

use core::{initialize_id_generator, style, System};

use core::App;
use imnodes::AttributeFlag;
use locale::Locale;

mod message;

mod core;

pub mod errors;
pub mod exprtree;
pub mod extensions;
pub mod locale;
pub mod nodes;
pub mod ode;
pub mod pins;
pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install().unwrap();

    let mut system = System::make_window("ODE-Designer", (1278.0, 768.0));

    style::set_eel_style(system.imgui.style_mut());

    let nodesctx = imnodes::Context::new();
    let mut nodeseditor = nodesctx.create_editor();

    style::set_imnodes_style(nodeseditor.get_style());

    initialize_id_generator(nodeseditor.new_identifier_generator());

    let _link_detach = nodeseditor.push(AttributeFlag::EnableLinkDetachWithDragClick);
    // let _link_creation = nodeseditor.push(AttributeFlag::EnableLinkCreationOnSnap);

    let mut locale = Locale::default();
    let mut app = App::new(&locale);
    #[cfg(debug_assertions)]
    let log_level = log::Level::Trace;
    #[cfg(not(debug_assertions))]
    let log_level = log::Level::Info;

    core::notification::NotificationLogger::new()
        .with_max_log_level(log_level)
        .init();

    let plot_ctx = implot::Context::create();
    //plot_ctx.use_classic_colors();
    plot_ctx.use_light_colors();

    system.main_loop(move |_, ui| {
        app.draw(
            ui,
            &mut nodeseditor,
            &mut plot_ctx.get_plot_ui(),
            &mut locale,
        );
    });
    Ok(())
}
