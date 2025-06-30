use glium::glutin::display::GetGlDisplay;
use glium::glutin::prelude::{GlDisplay, NotCurrentGlContext};
use glium::{glutin, Surface};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use raw_window_handle::HasWindowHandle;
use std::num::NonZeroU32;
use std::time::Instant;
use winit::event::{Event, WindowEvent};

pub use app::App;
pub use id_gen::{initialize_id_generator, GeneratesId};

pub mod adjust_params;
pub mod app;
mod id_gen;
pub mod menu;
pub mod notification;
pub mod plot;
pub mod python;
pub mod side_bar;
pub mod style;
pub mod widgets;

pub struct System {
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub display: glium::Display<glutin::surface::WindowSurface>,
    pub window: winit::window::Window,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
}

impl System {
    pub fn make_window<T: winit::dpi::Pixel>(title: &str, (width, height): (T, T)) -> Self {
        let event_loop = winit::event_loop::EventLoop::new().unwrap(); // assumed to remain

        let window_attrs = winit::window::WindowAttributes::default()
            .with_title(title)
            .with_transparent(false)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height));

        let config_template_builder = glutin::config::ConfigTemplateBuilder::new();
        let display_builder =
            glutin_winit::DisplayBuilder::new().with_window_attributes(Some(window_attrs));

        let (window, gl_config) = display_builder
            .build(&event_loop, config_template_builder, |mut configs| {
                configs.next().unwrap()
            })
            .unwrap();

        let window = window.unwrap();
        let raw_window_handle = window.window_handle().unwrap().as_raw();

        let context_attributes =
            glutin::context::ContextAttributesBuilder::new().build(Some(raw_window_handle));

        let fallback_context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let gl_display = gl_config.display();
        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .unwrap()
                })
        };

        let attrs =
            glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
                .build(
                    raw_window_handle,
                    NonZeroU32::new(width.cast()).unwrap(),
                    NonZeroU32::new(height.cast()).unwrap(),
                );

        let surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };
        let current_context = not_current_gl_context.make_current(&surface).unwrap();
        let display = glium::Display::from_context_surface(current_context, surface).unwrap();

        let mut imgui = Context::create();
        imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
        imgui.set_ini_filename(None);

        let mut platform = WinitPlatform::new(&mut imgui);
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);

        // Fixed font size. Note imgui_winit_support uses "logical
        // pixels", which are physical pixels scaled by the devices
        // scaling factor. Meaning, 13.0 pixels should look the same size
        // on two different screens, and thus we do not need to scale this
        // value (as the scaling is handled by winit)
        let font_size: f32 = 14.0f32;
        let nerdfont_size = font_size - 1.0;

        let nerdfont = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/fonts/nerdfont-mono.ttf"
        ));

        // Magic runes gotten from these ancient scrolls:
        // https://github.com/ryanoasis/nerd-fonts/wiki/Glyph-Sets-and-Code-Points
        static NERDFONT_RANGE: &[u32] = &[
            0x23fb, 0x23fe, 0xe000, 0xe00a, 0xe0a0, 0xe0a2, 0xe0b0, 0xe0b3, 0xe0b4, 0xe0c8, 0xe0cc,
            0xe0d4, 0xe200, 0xe2a9, 0xe300, 0xe3e3, 0xe5fa, 0xe6b1, 0xe700, 0xe7c5, 0xea60, 0xebeb,
            0xf000, 0xf2e0, 0xf300, 0xf372, 0xf400, 0xfd46, 0xf0001, 0xf1af0,
            // Don't remove this otherwise it will panic
            0,
        ];

        imgui.fonts().add_font(&[
            FontSource::DefaultFontData { config: Some(FontConfig {
                    size_pixels: nerdfont_size,  ..Default::default() }) 
                },
            FontSource::TtfData {
                data: nerdfont,            
                size_pixels: nerdfont_size,
                config: Some(FontConfig {
                    size_pixels: nerdfont_size,
                    glyph_ranges: FontGlyphRanges::from_slice(NERDFONT_RANGE),
                    ..Default::default()
                }),
            },
        ]);

        let renderer = Renderer::new(&mut imgui, &display).expect("Failed to initialize renderer");

        Self {
            event_loop,
            display,
            window,
            imgui,
            platform,
            renderer,
        }
    }

    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) {
        let Self {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            window,
            ..
        } = self;
        let mut last_frame = Instant::now();

        event_loop
            .run(move |event, elwt| match event {
                Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                Event::AboutToWait => {
                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    let ui = imgui.frame();

                    let mut run = true;
                    run_ui(&mut run, ui);
                    if !run {
                        elwt.exit();
                    }

                    let mut target = display.draw();
                    target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);
                    platform.prepare_render(ui, &window);

                    let draw_data = imgui.render();

                    renderer
                        .render(&mut target, draw_data)
                        .expect("Rendering failed");
                    target.finish().expect("Failed to swap buffers");
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    ..
                } => {
                    display.resize((new_size.width, new_size.height));
                    platform.handle_event(imgui.io_mut(), &window, &event);
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => elwt.exit(),
                event => {
                    platform.handle_event(imgui.io_mut(), &window, &event);
                }
            })
            .unwrap();
    }
}
