use std::cell::{Ref, RefCell};

use imgui::{DragDropFlags, Ui};

use crate::nodes::{Node, NodeImpl, Term};

use crate::ode::ga_json::{Bound, ConfigData, GA_Argument, GA_Metadata};

pub struct Parameter {
    pub term: Term,
    pub bounds: (f32, f32),
}
#[derive(Default)]
pub struct Model {
    pub parameters: Vec<Parameter>,
    pub adjusted_parameters: RefCell<Vec<usize>>,
}

impl Model {
    pub fn new(variables: Vec<Term>) -> Self {
        let parameters = variables
            .into_iter()
            .map(|term| Parameter {
                term: term.clone(),
                bounds: Default::default(),
            })
            .collect();

        Self {
            parameters,
            adjusted_parameters: RefCell::new(Vec::new()),
        }
    }

    pub fn draw_tables(&mut self, ui: &Ui, selected: &RefCell<Vec<usize>>){
        ui.columns(4, "Parameters", true);

        if let Some(_t) = ui.begin_table("Parameters", 3) {
            ui.table_setup_column("Variable Name");
            ui.table_setup_column("Initial Value");
            ui.table_setup_column("Estimate");
            ui.table_headers_row();

            for (index, parameter) in self
                .parameters
                .iter()
                .enumerate()
                .filter(|(id, _)| !RefCell::borrow(selected).contains(id))
            {
                ui.table_next_row();
                ui.table_next_column();

                ui.button_with_size(&imgui::ImString::new(parameter.term.name()), [60.0, 20.0]);

                let drag_drop_name = "parameter_drag";
                if let Some(tooltip) = ui
                    .drag_drop_source_config(drag_drop_name)
                    .flags(DragDropFlags::empty())
                    .begin_payload(index)
                {
                    ui.text(parameter.term.name());
                    tooltip.end();
                }

                ui.table_next_column();

                let value = parameter.term.initial_value;
                ui.text(imgui::ImString::new(value.to_string()));

                ui.table_next_column();
            }
        }

        ui.next_column();

        ui.button_with_size("Drag and Drop", [100.0, 75.0]);

        let drag_drop_name = "parameter_drag";

        if let Some(target) = ui.drag_drop_target() {
            if let Some(Ok(payload_data)) =
                target.accept_payload::<usize, _>(drag_drop_name, DragDropFlags::empty())
            {
                selected.borrow_mut().push(payload_data.data);

                println!("index: {}", payload_data.data);
            }
            target.pop();
        }

        ui.next_column();

        if let Some(_t) = ui.begin_table("Parameters to be adjusted", 3) {
            ui.table_setup_column("Name");
            ui.table_setup_column("Min");
            ui.table_setup_column("Max");
            ui.table_headers_row();

            for id in selected.borrow().iter() {
                let parameter = &mut self.parameters[*id];

                ui.table_next_row();
                let stack = ui.push_id(&imgui::ImString::new(parameter.term.name()));
                ui.table_next_column();
                {
                ui.text(imgui::ImString::new(parameter.term.name()));
                ui.table_next_column();
                ui.input_float("##min", &mut parameter.bounds.0).build();
                ui.table_next_column();
                ui.input_float("##max", &mut parameter.bounds.1).build();
                }
                stack.pop();
            }

            //ui.push_style_color(style_color, color)

            //ui.color_button("color_button", [1.0, 0.0, 0.0, 1.0]);
        }

        ui.next_column();
        let run_button = ui.button("Run");

        if run_button {
            self.populate_config_data(selected);
        }
    }

    pub fn populate_config_data(&self, selected: &RefCell<Vec<usize>>) {
        let metadata = GA_Metadata {
            name: String::from("TODO!"),
            start_time: 0.0,
            delta_time: 0.1,
            end_time: 10.0,
            population_size: 100,
            crossover_rate: 0.7,
            mutation_rate: 0.01,
            max_iterations: 1000,
        };

        let mut arguments: Vec<GA_Argument> = vec![];
        let mut bounds: Vec<Bound> = vec![];

        for id in selected.borrow().iter() {
            let parameter = &self.parameters[*id];

            arguments.push(GA_Argument::new(
                parameter.term.name().to_string(),
                parameter.term.initial_value,
            ));

            bounds.push(Bound::new(
                parameter.term.name().to_string(),
                parameter.bounds.0 as f64,
                parameter.bounds.1 as f64,
            ));
        }

        arguments.sort_by(|a, b| a.name.cmp(&b.name));

        let config_data = ConfigData {
            metadata,
            arguments,
            bounds,
        };

        println!("{:?}", config_data);
    }
}
