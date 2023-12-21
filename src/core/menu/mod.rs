use imgui::Ui;

use crate::App;
use rfd::FileDialog;

use super::app::SimulationState;

impl App {
    fn draw_menu_load_csv(&mut self, ui: &Ui) {
        if ui.menu_item("Plot CSV file") {
            let file = FileDialog::new()
                .add_filter("csv", &["csv"])
                .set_directory(".")
                .pick_file();

            if let Some(file_path) = file {
                self.simulation_state = Some(SimulationState::from_csv(file_path));
            }
        }
    }

    pub fn draw_menu(&mut self, ui: &Ui) {
        ui.menu_bar(|| {
            ui.menu("File", || {
                if ui.menu_item("New") {
                    todo!("New not implemented");
                }

                if ui.menu_item("Load") {
                    self.load_state();
                }

                if ui.menu_item("Save") {
                    self.save_state();
                }

                self.draw_menu_load_csv(ui);
            });

            ui.menu("Edit", || {
                if ui.menu_item("Generate Code") {
                    todo!("Generate code not implemented");
                }

                if ui.menu_item("Simulate") {
                    todo!("Simulate not implemented");
                }
            })
        });
    }
}
