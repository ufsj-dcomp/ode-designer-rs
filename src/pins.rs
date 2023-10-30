use imnodes::{InputPinId, NodeId, OutputPinId};

use crate::{core::GeneratesId, nodes::LinkPayload};

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

impl From<Sign> for char {
    fn from(value: Sign) -> Self {
        match value {
            Sign::Positive => '+',
            Sign::Negative => '-',
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputPin {
    pub id: InputPinId,
    node_id: NodeId,
    pub sign: Sign,
    pub linked_to: Option<OutputPinId>,
}

#[derive(Debug, Clone)]
pub struct OutputPin {
    pub id: OutputPinId,
    node_id: NodeId,
    pub sign: Sign,
    pub linked_to: Vec<InputPinId>,
}

impl InputPin {
    pub fn map_data(&self, data: LinkPayload) -> LinkPayload {
        match (data, self.sign) {
            (LinkPayload::Number(n), sign) => LinkPayload::Number(n * sign.to_multiplier()),
            (LinkPayload::Text(t), Sign::Negative) => LinkPayload::Text(format!("(-{t})")),
            (data, _) => data,
        }
    }
}

pub trait Pin: Sized {
    type SelfIdType: GeneratesId;
    type LinkedToIdType: PartialEq + Copy;

    fn id(&self) -> &Self::SelfIdType;

    fn new_signed(node_id: NodeId, sign: Sign) -> Self;

    fn new(node_id: NodeId) -> Self {
        Self::new_signed(node_id, Sign::Positive)
    }

    fn is_linked_to(&self, pin_id: &Self::LinkedToIdType) -> bool;

    fn has_links(&self) -> bool;

    fn link_to(&mut self, pin_id: &Self::LinkedToIdType);

    fn unlink(&mut self, pin_id: &Self::LinkedToIdType) -> bool;

    fn get_shape(&self) -> imnodes::PinShape {
        if self.has_links() {
            imnodes::PinShape::CircleFilled
        } else {
            imnodes::PinShape::Circle
        }
    }
}

impl Pin for InputPin {
    type SelfIdType = InputPinId;
    type LinkedToIdType = OutputPinId;

    fn id(&self) -> &Self::SelfIdType {
        &self.id
    }

    fn new_signed(node_id: NodeId, sign: Sign) -> Self {
        Self {
            id: Self::SelfIdType::generate(),
            node_id,
            sign: Sign::Positive,
            linked_to: None,
        }
    }

    fn is_linked_to(&self, pin_id: &Self::LinkedToIdType) -> bool {
        self.linked_to.iter().any(|id| id == pin_id)
    }
    fn has_links(&self) -> bool {
        self.linked_to.is_some()
    }

    fn link_to(&mut self, pin_id: &Self::LinkedToIdType) {
        self.linked_to = Some(*pin_id);
    }

    fn unlink(&mut self, pin_id: &Self::LinkedToIdType) -> bool {
        match self.linked_to {
            Some(ref linked_to_pin_id) => {
                if pin_id == linked_to_pin_id {
                    self.linked_to = None;
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}

impl Pin for OutputPin {
    type SelfIdType = OutputPinId;
    type LinkedToIdType = InputPinId;

    fn id(&self) -> &Self::SelfIdType {
        &self.id
    }

    fn new_signed(node_id: NodeId, sign: Sign) -> Self {
        Self {
            id: Self::SelfIdType::generate(),
            node_id,
            sign: Sign::Positive,
            linked_to: Vec::new(),
        }
    }

    fn is_linked_to(&self, pin_id: &Self::LinkedToIdType) -> bool {
        self.linked_to.iter().any(|id| id == pin_id)
    }
    fn has_links(&self) -> bool {
        !self.linked_to.is_empty()
    }

    fn link_to(&mut self, pin_id: &Self::LinkedToIdType) {
        self.linked_to.push(*pin_id);
    }

    fn unlink(&mut self, pin_id: &Self::LinkedToIdType) -> bool {
        let o: Option<_> = {
            try {
                let i = self.linked_to.iter().position(|id| id == pin_id)?;
                self.linked_to.swap_remove(i);
            }
        };
        o.is_some()
    }
}
