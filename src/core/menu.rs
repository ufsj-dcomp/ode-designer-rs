use imgui::{Ui, StyleVar};

use crate::{
    locale::{Locale, LANGUAGES},
    utils::localized_error,
    App,
};
use rfd::FileDialog;

use std::{fs::read_to_string, process::Command};

use super::{
    adjust_params::ParameterEstimationState,
    app::{AppState, SimulationState},
    python::execute_python_code,
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
                    localized_error!(locale, "error-csv-read", "file" => file_path.display().to_string())
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
                    let _ = self.load_state();                    
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
                            command.arg("--xlabel").arg(&self.text_fields.x_label);
                        }
                        if !self.text_fields.y_label.is_empty() {
                            command.arg("--ylabel").arg(&self.text_fields.y_label);
                        }

                        match execute_python_code(&mut command) {
                            Ok(_) => {}
                            Err(err) => {
                                localized_error!(locale, "error-pdf-export");
                                eprintln!("{err}")
                            }
                        }
                    }
                }
            });

            ui.menu(locale.get("run"), || {
                self.draw_input_label(ui);

                // Toggle dark theme

                ui.separator();

                let text = locale.get("dark-theme");
                let text_width = ui.calc_text_size(text)[0];
                let checkbox_size = 16.0; 
                let spacing = 8.0;
                let total_width = text_width + spacing + checkbox_size;

                let window_width = ui.content_region_avail()[0];
                let start_x = (window_width - total_width) / 2.0;
                ui.set_cursor_pos([start_x, ui.cursor_pos()[1]]);

                ui.text(text);
                ui.same_line_with_spacing(0.0, spacing);

                let mut dark_theme_toggle = self.dark_theme;
                let _style = ui.push_style_var(StyleVar::FramePadding([2.0, 2.0]));
                if ui.checkbox("##dark_theme", &mut dark_theme_toggle) {
                    self.dark_theme = dark_theme_toggle;
                }

                ui.separator();

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
                                simulation_state.plot.bg_color = self.dark_theme;
                                self.simulation_state = Some(simulation_state);
                            }
                        }
                        Err(err) => {
                            localized_error!(locale, "error-python-exec", "reason" => err.to_string());
                            eprintln!("{err}")
                        }
                    }
                }
            });

            if ui.menu_item(locale.get("parameter-estimation"))
                && self.parameter_estimation_state.is_none()
            {
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
