use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::{glutin, Display, Surface};
use imgui::internal::RawCast;
use imgui::{Context, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

use crate::app::App;
use crate::id_gen::initialize_id_generator;
use crate::nodes::specialization::NodeSpecializationInitializer;
use crate::nodes::{
    specialization::{Combinator, Constant},
    Node,
};

// mod clipboard;
pub mod app;
mod style;

pub fn main(mut app: App) -> Result<(), Box<dyn std::error::Error>> {
    let mut system = make_window("ODE Editor", LogicalSize::new(1024.0, 768.0));
    // SAFETY: The pointer is valid for the lifetime of the function.
    style::set_eel_style(unsafe { system.imgui.style_mut().raw_mut() });
    let nodesctx = imnodes::Context::new();
    let mut nodeseditor = nodesctx.create_editor();

    unsafe { initialize_id_generator(nodeseditor.new_identifier_generator()) };

    app.add_node(Node::new_with_specialization("K", Constant::new_boxed));
    app.add_node(Node::new_with_specialization("comb", Combinator::new_boxed));
    app.add_node(Node::new_with_specialization(
        "comb2",
        Combinator::new_boxed,
    ));

    system.main_loop(move |_, ui| {
        app.draw(ui, &mut nodeseditor);
    });
    Ok(())
}

pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
}

pub fn make_window(title: &str, size: LogicalSize<f32>) -> System {
    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = WindowBuilder::new()
        .with_title(title.to_owned())
        .with_transparent(false)
        .with_inner_size(size);
    let display =
        Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
    imgui.set_ini_filename(None);

    /* if let Some(backend) = clipboard::init() {
        imgui.set_clipboard_backend(backend);
    } else {
        eprintln!("Failed to initialize clipboard");
    } */

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();

        let dpi_mode = HiDpiMode::Default;

        platform.attach_window(imgui.io_mut(), window, dpi_mode);
    }

    // Fixed font size. Note imgui_winit_support uses "logical
    // pixels", which are physical pixels scaled by the devices
    // scaling factor. Meaning, 13.0 pixels should look the same size
    // on two different screens, and thus we do not need to scale this
    // value (as the scaling is handled by winit)
    let font_size = 13.0;

    imgui
        .fonts()
        .add_font(&[FontSource::DefaultFontData { config: None }]);

    let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    System {
        event_loop,
        display,
        imgui,
        platform,
        renderer,
        font_size,
    }
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) {
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;
        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();

                let mut run = true;
                run_ui(&mut run, ui);
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);
                platform.prepare_render(ui, gl_window.window());
                let draw_data = imgui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        })
    }
}
