use imgui::Ui;

use crate::{
    core::adjust_params,
    locale::{Locale, LANGUAGES},
    nodes::expression,
    App,
};
use rfd::FileDialog;

use std::{cell::RefCell, fs::read_to_string, process::Command};

use super::{
    adjust_params::ParameterEstimationState,
    app::{AppState, SimulationState},
    python::{execute_python_code, PythonError},
};

impl App {
    fn draw_menu_load_csv(&mut self, ui: &Ui, locale: &Locale) {
        if ui.menu_item(locale.get("file-plot")) {
            let file = FileDialog::new()
                .add_filter("csv", &["csv"])
                .set_directory(".")
                .pick_file();

            if let Some(file_path) = file {
                if let Ok(file_content) = read_to_string(&file_path) {
                    self.simulation_state = Some(SimulationState::from_csv(file_content, locale));
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

    pub fn draw_menu(&mut self, ui: &Ui, locale: &mut Locale) {
        ui.menu_bar(|| {
            ui.menu(locale.get("file"), || {
                if ui
                    .menu_item_config(locale.get("file-new"))
                    .shortcut("Ctrl + N")
                    .build()
                {
                    self.clear_state();
                }

                if ui
                    .menu_item_config(locale.get("file-load"))
                    .shortcut("Ctrl + O")
                    .build()
                {
                    self.clear_state();
                    if let Err(err) = self.load_state() {
                        eprintln!("Couldn't load model from file: {err}");
                    }
                }

                if ui
                    .menu_item_config(locale.get("file-save"))
                    .shortcut("Ctrl + S")
                    .build()
                {
                    self.save_state();
                }

                self.draw_menu_load_csv(ui, locale);
            });

            ui.menu(locale.get("export"), || {
                self.draw_input_label(ui);
                if ui.menu_item(locale.get("export-code")) {
                    let py_code = self.generate_code();
                    self.save_to_file(py_code, "py");
                }

                if ui.menu_item(locale.get("export-pdf")) {
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

            ui.menu(locale.get("run"), || {
                self.draw_input_label(ui);

                if ui.menu_item(locale.get("run")) {
                    let py_code = self.generate_code();

                    let mut command = Command::new("python3");
                    command
                        .arg("-c")
                        .arg(&py_code)
                        .arg("--csv")
                        .args(self.sidebar_state.time_flags());

                    match execute_python_code(&mut command) {
                        Ok(output) => {
                            self.simulation_state = Some(SimulationState::from_csv(output, locale));
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

            if ui.menu_item("Parameter estimation") && self.parameter_estimation_state.is_none() {
                let all_population_ids = self.get_all_population_ids();
                let all_constants = self.get_all_constants(&all_population_ids);
                let all_populations = self.get_all_populations(&all_population_ids);
                let param_state =
                    ParameterEstimationState::new(all_populations, all_constants.clone());
                self.parameter_estimation_state.replace(param_state);
                self.generate_equations(all_constants);
            }

            if ui.menu_item(locale.get("extensions")) {
                self.state = if let Some(AppState::ManagingExtensions) = self.state {
                    None
                } else {
                    Some(AppState::ManagingExtensions)
                }
            }

            let current_loc = locale.current();
            let mut selected_loc = current_loc;

            ui.menu(locale.get("language"), || {
                for (loc, name) in LANGUAGES {
                    ui.radio_button(name, &mut selected_loc, loc);
                }
            });

            if selected_loc != current_loc {
                self.update_locale(locale, selected_loc.clone());
            }
        });
    }
}
