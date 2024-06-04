use imgui::{DragDropFlags, MouseButton, Ui};
use imnodes::NodeId;
use std::collections::BTreeMap;
use std::ops::Range;

use crate::locale::Locale;
use crate::nodes::{NodeImpl, Term};

use crate::ode::ga_json::{Bound, ConfigData, GA_Argument, GA_Metadata};
use crate::ode::odesystem::OdeSystem;
use crate::ode::ParameterEstimation;

use super::App;

#[derive(Debug, Clone)]
pub struct Parameter {
    term: Term,
    range: Range<f32>,
    selected: bool,
    min_label: String,
    max_label: String
}

impl Parameter {
    pub fn new(term: Term) -> Self {
        let node_id: i32 = term.id().into();
        Self {
            term,
            range: 0.01..1.0,
            selected: false,
            min_label: format!("##min-{node_id}"),
            max_label: format!("##max-{node_id}"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct ParameterEstimationState {
    parameters: BTreeMap<NodeId, Parameter>,
    pub ode_system: OdeSystem,
    pub estimator: ParameterEstimation,
}

impl ParameterEstimationState {
    pub fn new(terms: impl IntoIterator<Item = Term>) -> Self {
        Self {
            parameters: terms
                .into_iter()
                .map(|term| (term.id(), Parameter::new(term)))
                .collect(),
            ode_system: OdeSystem::new(),
            estimator: ParameterEstimation::default(),
        }
    }

    pub fn add_variable(&mut self, term: Term) {
        self.parameters.insert(term.id(), Parameter::new(term));
    }

    pub fn remove_variable(&mut self, node_id: &NodeId) {
        self.parameters.remove(node_id);
    }

    pub fn rename_variable(&mut self, node_id: &NodeId, name: impl ToOwned<Owned = String>) {
        if let Some(param) = self.parameters.get_mut(node_id) {
            name.clone_into(param.term.name_mut());
        }
    }

    pub fn clear_selected(&mut self) {
        self.parameters.values_mut()
            .for_each(|param| param.selected = false);
    }

    pub fn draw_tables(&mut self, ui: &Ui, locale: &Locale) {
        // This is required because we want to replace the drag-and-drop target
        // component, which would only receive data *after* nothing is being
        // dragged anymore
        static mut DRAGGING: bool = false;
        const DRAG_DROP_NAME: &str = "parameter_drag";
        
        ui.columns(4, "Parameters", true);
        if let Some(_t) = ui.begin_table("Parameters", 2) {
            ui.table_setup_column(locale.get("parameter-name"));
            ui.table_setup_column(locale.get("initial-value"));
            ui.table_headers_row();

            for (id, parameter) in self
                .parameters
                .iter()
                .filter(|(_id, param)| !param.selected)
            {
                ui.table_next_row();
                ui.table_next_column();

                ui.button_with_size(parameter.term.name(), [60.0, 20.0]);

                if let Some(tooltip) = ui
                    .drag_drop_source_config(DRAG_DROP_NAME)
                    .flags(DragDropFlags::empty())
                    .begin_payload(*id)
                {
                    // Safety: this is fine because the software isn't
                    // multi-threaded and this global is local to this function
                    unsafe { DRAGGING = true; }
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

        if unsafe { DRAGGING } {
            ui.button_with_size(locale.get("dnd-parameter-estimation"), [100.0, 100.0]);
            if let Some(target) = ui.drag_drop_target() {
                if let Some(Ok(payload_data)) =
                    target.accept_payload(DRAG_DROP_NAME, DragDropFlags::empty())
                {
                    let selected_id: NodeId = payload_data.data;
                    if let Some(parameter) = self.parameters.get_mut(&selected_id) {
                        parameter.selected = true;
                        // Safety: this is fine because the software isn't
                        // multi-threaded and this global is local to this
                        // function
                        unsafe { DRAGGING = false; }
                    }
                } else if !ui.is_mouse_down(MouseButton::Left) {
                    // Safety: this is fine because the software isn't
                    // multi-threaded and this global is local to this function
                    unsafe { DRAGGING = false; }
                }
                target.pop();
            }
        } else if let Some(_t) = ui.begin_table("Parameters to be adjusted", 3) {
            ui.table_setup_column(locale.get("parameter-name"));
            ui.table_setup_column(locale.get("min-value"));
            ui.table_setup_column(locale.get("max-value"));
            ui.table_headers_row();

            for parameter in self
                .parameters
                .values_mut()
                .filter(|param| param.selected)
            {
                ui.table_next_row();
                let stack = ui.push_id(parameter.term.name());
                ui.table_next_column();
                {
                    ui.text(imgui::ImString::new(parameter.term.name()));
                    ui.table_next_column();
                    ui.input_float(
                        &parameter.min_label,
                        &mut parameter.range.start,
                    )
                    .build();
                    ui.table_next_column();
                    ui.input_float(
                        &parameter.max_label,
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

    pub fn populate_config_data(&mut self) {
        let metadata = GA_Metadata {
            name: String::from("GA"),
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

        self.estimator.config_data = ConfigData {
            metadata,
            arguments,
            bounds,
        };

        println!("Ode system: {:#?}", self.ode_system);
        println!("Estimator: {:#?}", self.estimator);
    }    
}
