use std::{collections::HashMap, fmt::Write};

use imnodes::{InputPinId, NodeId};
use linkme::distributed_slice;
use strum::StaticVariantsArray;

use crate::{
    nodes::{Data, Node},
    pins::{InputPin, OutputPin, Pin, Sign}, declare_node,
};

use super::{
    NameAndConstructor, NodeSpecialization, NodeSpecializationInitializer, NODE_SPECIALIZATIONS,
};

declare_node!(Combinator);

#[derive(Debug)]
pub struct Combinator {
    node: Node,
    operation: Operation,
    input_exprs: HashMap<InputPinId, Data>,
    inputs: Vec<InputPin>,
    output: OutputPin,
}

impl Combinator {
    pub fn expression_string(&self) -> String {
        let search_pin = |pin_id| self.inputs.iter().find(|pin| pin.id() == pin_id);
        self.input_exprs
            .iter()
            .map(|(key, value)| {
                let input_pin = search_pin(key).unwrap();
                match input_pin.map_data(value.clone()) {
                    Data::Number(num) => num.to_string(),
                    Data::Text(name) => name.to_string(),
                }
            })
            .collect::<Vec<_>>()
            .join(&self.operation.to_string())
    }
}

impl NodeSpecialization for Combinator {
    fn id(&self) -> NodeId {
        self.node.id()
    }

    fn name(&self) -> &str {
        &self.node.name
    }

    fn send_data(&self) -> Data {
        let mut expr = self.expression_string();
        if !expr.is_empty() {
            expr = format!("({})", expr)
        };
        expr.into()
    }

    fn on_data_received(&mut self, from_pin_id: InputPinId, data: Data) -> bool {
        self.input_exprs.insert(from_pin_id, data);
        true
    }

    fn draw(&mut self, ui: &imgui::Ui) -> bool {
        let mut selected = self.operation as usize;
        let mut changed = false;

        // Needs to be assigned to a variable other than `_`. Otherwise, the
        // style isn't applied. That's probably the case because it needs to be
        // dropped *after* the combo below has been executed.
        let _smth = ui.push_item_width(50.);

        if ui.combo(
            "##combinator operation select",
            &mut selected,
            Operation::ALL_VARIANTS,
            |op| format!("{op}").into(),
        ) {
            self.operation = Operation::from_repr(selected as u8)
                .expect("ImGui returned an out-of-range value in combobox");

            changed = true
        }

        let expr = self.expression_string();

        if expr.is_empty() {
            ui.text("Nothing yet!");
        } else {
            ui.text(expr);
        }

        changed
    }

    fn inputs(&self) -> Option<&[InputPin]> {
        Some(&self.inputs)
    }

    fn outputs(&self) -> Option<&[OutputPin]> {
        Some(std::array::from_ref(&self.output))
    }

    fn inputs_mut(&mut self) -> Option<&mut [InputPin]> {
        Some(&mut self.inputs)
    }

    fn outputs_mut(&mut self) -> Option<&mut [OutputPin]> {
        Some(std::array::from_mut(&mut self.output))
    }
}

impl NodeSpecializationInitializer for Combinator {
    fn new(node: Node) -> Self {
        let node_id = node.id();

        Self {
            node,
            operation: Operation::default(),
            input_exprs: HashMap::new(),
            inputs: vec![
                Pin::new_signed(node_id, Sign::Positive),
                Pin::new_signed(node_id, Sign::Positive),
            ],
            output: Pin::new_output(node_id),
        }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::EnumIter,
    strum::EnumVariantNames,
    strum::FromRepr,
    strum::StaticVariantsArray,
)]
#[repr(u8)]
pub enum Operation {
    #[default]
    Add,
    Sub,
    Div,
    Mul,
}

pub struct NotAnOperationChar;

impl TryFrom<char> for Operation {
    type Error = NotAnOperationChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '/' => Ok(Self::Div),
            '*' => Ok(Self::Mul),
            _ => Err(NotAnOperationChar),
        }
    }
}

impl From<Operation> for char {
    fn from(value: Operation) -> Self {
        match value {
            Operation::Add => '+',
            Operation::Sub => '-',
            Operation::Div => '/',
            Operation::Mul => '*',
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = (*self).into();
        f.write_char(c)
    }
}
