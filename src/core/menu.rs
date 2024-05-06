use imgui::Ui;

use crate::{nodes::expression, App};
use rfd::FileDialog;

use std::{
    fs::read_to_string,
    process::{Command, Stdio},
};

use super::{
    app::{AppState, SimulationState},
    python::{execute_python_code, PythonError},
};

impl<'n> App<'n> {
    fn draw_menu_load_csv(&mut self, ui: &Ui) {
        if ui.menu_item("Plot CSV file") {
            let file = FileDialog::new()
                .add_filter("csv", &["csv"])
                .set_directory(".")
                .pick_file();

            if let Some(file_path) = file {
                if let Ok(file_content) = read_to_string(&file_path) {
                    self.simulation_state = Some(SimulationState::from_csv(file_content));
                } else {
                    eprintln!("Error: Failed to read file content.");
                }
            }
        }
    }

    pub fn draw_input_label(&mut self, ui: &Ui) {
        ui.input_text("X Label", &mut self.text_fields.x_label)
            .hint("time (days)")
            .build();
        ui.input_text("Y Label", &mut self.text_fields.y_label)
            .hint("conc/ml")
            .build();
    }

    pub fn draw_menu(&mut self, ui: &Ui) {
        ui.menu_bar(|| {
            ui.menu("File", || {
                if ui.menu_item_config("New").shortcut("Ctrl + N").build() {
                    self.clear_state();
                }

                if ui.menu_item_config("Load").shortcut("Ctrl + O").build() {
                    self.clear_state();
                    if let Err(err) = self.load_state() {
                        eprintln!("Couldn't load model from file: {err}");
                    }
                }

                if ui.menu_item_config("Save").shortcut("Ctrl + S").build() {
                    self.save_state();
                }

                self.draw_menu_load_csv(ui);
            });

            ui.menu("Export", || {
                self.draw_input_label(ui);

                if ui.menu_item("Generate Code") {
                    let py_code = self.generate_code();
                    self.save_to_file(py_code, "py");
                }

                if ui.menu_item("Plot to PDF") {
                    if let Some(file_path) =
                        FileDialog::new().add_filter("pdf", &["pdf"]).save_file()
                    {
                        let py_code = self.generate_code();

                        let mut command = Command::new("python3");
                        command
                            .arg("-c")
                            .arg(&py_code)
                            .arg("--output")
                            .arg(file_path)
                            .args(self.sidebar_state.time_flags());

                        if !self.text_fields.x_label.is_empty() {
                            command
                                .arg("--xlabel")
                                .arg(self.text_fields.x_label.to_string());
                        }
                        if !self.text_fields.y_label.is_empty() {
                            command
                                .arg("--ylabel")
                                .arg(self.text_fields.y_label.to_string());
                        }

                        match execute_python_code(&mut command) {
                            Ok(_) => {}
                            Err(err) => eprintln!("{err}"),
                        }
                    }
                }
            });

            ui.menu("Run", || {
                self.draw_input_label(ui);

                if ui.menu_item("Run") {
                    let py_code = self.generate_code();

                    let mut command = Command::new("python3");
                    command
                        .arg("-c")
                        .arg(&py_code)
                        .arg("--csv")
                        .args(self.sidebar_state.time_flags());

                    match execute_python_code(&mut command) {
                        Ok(output) => {
                            self.simulation_state = Some(SimulationState::from_csv(output));
                            if let Some(mut simulation_state) = self.simulation_state.clone() {
                                if !self.text_fields.x_label.is_empty() {
                                    simulation_state.plot.xlabel =
                                        self.text_fields.x_label.to_string();
                                }
                                if !self.text_fields.y_label.is_empty() {
                                    simulation_state.plot.ylabel =
                                        self.text_fields.y_label.to_string();
                                }
                                self.simulation_state = Some(simulation_state);
                            }
                        }
                        Err(err) => eprintln!("{err}"),
                    }
                }
            });

            if ui.menu_item("Parameter estimation") {
                self.state = if let Some(AppState::EstimatingParameters) = self.state {
                    None
                } else {
                    Some(AppState::EstimatingParameters)
                }
            }

            /*/*let ode_system = self.generate_equations();
            println!("ODEs: {:#?}", ode_system); */ */

            if ui.menu_item("Manage Extensions") {
                self.state = if let Some(AppState::ManagingExtensions) = self.state {
                    None
                } else {
                    Some(AppState::ManagingExtensions)
                }
            }
        });
    }
}
