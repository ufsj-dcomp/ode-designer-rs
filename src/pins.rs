use imnodes::{InputPinId, NodeId, OutputPinId};

use crate::{id_gen::GeneratesId, nodes::Data};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub fn toggle(&mut self) {
        *self = match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }

    fn to_multiplier(self) -> f64 {
        match self {
            Sign::Positive => 1.0,
            Sign::Negative => -1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum InputClass {
    Normal,
    Signed(Sign),
}

#[derive(Debug, Clone)]
pub enum PinClass {
    Input(InputClass),
    Output,
}

#[derive(Debug, Clone)]
pub struct Pin<SelfIdType, LinkedToIdType> {
    pub id: SelfIdType,
    node_id: NodeId,
    pub class: PinClass,
    pub linked_to: Vec<LinkedToIdType>,
}

pub type InputPin = Pin<InputPinId, OutputPinId>;
pub type OutputPin = Pin<OutputPinId, InputPinId>;

impl<SelfIdType: GeneratesId, LinkedToIdType: PartialEq + Copy> Pin<SelfIdType, LinkedToIdType> {
    pub fn new_of_class(node_id: NodeId, class: PinClass) -> Self {
        Self {
            id: SelfIdType::generate(),
            node_id,
            class,
            linked_to: vec![],
        }
    }
    pub fn new_output(node_id: NodeId) -> Self {
        Self::new_of_class(node_id, PinClass::Output)
    }
    pub fn new_input(node_id: NodeId) -> Self {
        Self::new_of_class(node_id, PinClass::Input(InputClass::Normal))
    }
    pub fn new_signed(node_id: NodeId, sign: Sign) -> Self {
        Self::new_of_class(node_id, PinClass::Input(InputClass::Signed(sign)))
    }
    pub fn link_to(&mut self, pin_id: &LinkedToIdType) {
        self.linked_to.push(*pin_id);
    }
    pub fn unlink(&mut self, pin_id: &LinkedToIdType) -> bool {
        let o: Option<_> = {
            try {
                let i = self.linked_to.iter().position(|id| id == pin_id)?;
                self.linked_to.swap_remove(i);
            }
        };
        o.is_some()
    }
    pub fn id(&self) -> &SelfIdType {
        &self.id
    }
    pub fn is_linked_to(&self, pin_id: &LinkedToIdType) -> bool {
        self.linked_to.iter().any(|id| id == pin_id)
    }
    pub fn has_links(&self) -> bool {
        !self.linked_to.is_empty()
    }
    pub fn get_shape(&self) -> imnodes::PinShape {
        if self.has_links() {
            imnodes::PinShape::CircleFilled
        } else {
            imnodes::PinShape::Circle
        }
    }
    pub fn class(&self) -> &PinClass {
        &self.class
    }
    pub fn map_data(&self, data: Data) -> Data {
        match self.class() {
            PinClass::Input(InputClass::Signed(sign)) => match data {
                Data::Number(n) => Data::Number(n * sign.to_multiplier()),
                d => d,
            },
            PinClass::Input(InputClass::Normal) | PinClass::Output => data,
        }
    }
}
