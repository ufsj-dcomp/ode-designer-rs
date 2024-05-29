use imgui::{DragDropFlags, Ui};
use imnodes::NodeId;
use std::cell::{Ref, RefCell};
use std::collections::BTreeMap;
use std::ops::Range;

use crate::nodes::{Node, NodeImpl, Term};

use crate::ode::ga_json::{Bound, ConfigData, GA_Argument, GA_Metadata};

use super::App;

#[derive(Debug)]
pub struct Parameter {
    pub term: Term,
    pub range: Range<f32>,
    pub selected: bool,
}

#[derive(Default, Debug)]
pub struct ParameterEstimationState {
    pub parameters: BTreeMap<NodeId, Parameter>,
    pub update_needed: bool,
}

impl ParameterEstimationState {
    pub fn new(variables: Vec<Term>) -> Self {
        let mut parameters = BTreeMap::new();

        for term in variables.iter() {
            parameters.insert(
                term.id,
                Parameter {
                    term: term.clone(),
                    range: 0.01..1.0,
                    selected: false,
                },
            );
        }

        println!("parameters: {:#?}", parameters);
        Self {
            parameters: parameters,
            update_needed: false,
        }
    }

    pub fn set_update_needed(&mut self, value: bool) {
        self.update_needed = value;
    }

    pub fn clear_selected(&mut self) {
        self.parameters.clear();
    }

    pub fn draw_tables(&mut self, ui: &Ui) {
        ui.columns(4, "Parameters", true);

        if let Some(_t) = ui.begin_table("Parameters", 2) {
            ui.table_setup_column("Variable Name");
            ui.table_setup_column("Initial Value");
            ui.table_headers_row();

            for (index, (id, parameter)) in self
                .parameters
                .iter()
                .enumerate()
                .filter(|(_id, value)| !value.1.selected)
            {
                ui.table_next_row();
                ui.table_next_column();

                ui.button_with_size(&imgui::ImString::new(parameter.term.name()), [60.0, 20.0]);

                let drag_drop_name = "parameter_drag";
                if let Some(tooltip) = ui
                    .drag_drop_source_config(drag_drop_name)
                    .flags(DragDropFlags::empty())
                    .begin_payload(id.clone())
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
                target.accept_payload::<NodeId, _>(drag_drop_name, DragDropFlags::empty())
            {
                let selected_id: NodeId = payload_data.data;
                if let Some(parameter) = self.parameters.get_mut(&selected_id) {
                    parameter.selected = true;
                }
            }
            target.pop();
        }

        ui.next_column();

        if let Some(_t) = ui.begin_table("Parameters to be adjusted", 3) {
            ui.table_setup_column("Name");
            ui.table_setup_column("Min");
            ui.table_setup_column("Max");
            ui.table_headers_row();

            for (id, parameter) in self
                .parameters
                .iter_mut()
                .filter(|(_, value)| value.selected)
            {
                ui.table_next_row();
                let stack = ui.push_id(&imgui::ImString::new(parameter.term.name()));
                ui.table_next_column();
                {
                    ui.text(imgui::ImString::new(parameter.term.name()));
                    ui.table_next_column();
                    ui.input_float(
                        format!("##min-{0}", parameter.term.name()),
                        &mut parameter.range.start,
                    )
                    .build();
                    ui.table_next_column();
                    ui.input_float(
                        format!("##max={0}", parameter.term.name()),
                        &mut parameter.range.end,
                    )
                    .build();
                }
                stack.pop();
            }
        }

        ui.next_column();
        let run_button = ui.button("Run");

        if run_button {
            self.populate_config_data();
        }
    }

    pub fn populate_config_data(&self) {
        let metadata = GA_Metadata {
            name: String::from("TODO!"),
            start_time: 0.0,
            delta_time: 0.1,
            end_time: 10.0,
            population_size: 100,
            crossover_rate: 0.5,
            mutation_rate: 0.75,
            max_iterations: 50,
        };

        let mut arguments: Vec<GA_Argument> = vec![];
        let mut bounds: Vec<Bound> = vec![];

        for (_id, parameter) in self.parameters.iter() {
            arguments.push(GA_Argument::new(
                parameter.term.name().to_string(),
                parameter.term.initial_value,
            ));
        }

        for (_id, parameter) in self.parameters.iter().filter(|(_id, param)| param.selected) {
            bounds.push(Bound::new(
                parameter.term.name().to_string(),
                parameter.range.start as f64,
                parameter.range.end as f64,
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
