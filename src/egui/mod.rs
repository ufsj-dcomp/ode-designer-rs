use egui::Context;
use nodes::{NodeClass, Node};

mod nodes;

struct OdeEditorApp {
    graph: egui_node_graph::GraphEditorState<Node, nodes::NodeClassDiscriminant, Node, (), ()>,
}

impl OdeEditorApp {
    fn new(cc: &eframe::CreationContext) -> Self { Self { graph: Default::default()  } }
}

impl eframe::App for OdeEditorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("pog");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.graph
                .draw_graph_editor(ui, (), &mut ())
        });
    }
}

pub fn main() -> eframe::Result<()> {
    
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "ode-editor",
        native_options,
        Box::new(|cc| Box::new(OdeEditorApp::new(cc))),
    )
}
