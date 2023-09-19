use std::ops::{Deref, DerefMut};

use imnodes::{IdentifierGenerator, InputPinId, LinkId, NodeId, OutputPinId};

struct IdGenWrapper {
    internal_id_gen: Option<IdentifierGenerator>,
}

static mut ID_GEN: IdGenWrapper = IdGenWrapper {
    internal_id_gen: None,
};

impl IdGenWrapper {
    pub fn emplace(&mut self, internal_id_gen: IdentifierGenerator) {
        self.internal_id_gen = Some(internal_id_gen);
    }
}

pub unsafe fn initialize_id_generator(internal_id_gen: IdentifierGenerator) {
    ID_GEN.emplace(internal_id_gen);
}

impl Deref for IdGenWrapper {
    type Target = IdentifierGenerator;
    fn deref(&self) -> &Self::Target {
        match &self.internal_id_gen {
            Some(ref internal_id_gen) => internal_id_gen,
            None => panic!("Tried to take an IdentifierGenerator without initializing it"),
        }
    }
}

impl DerefMut for IdGenWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.internal_id_gen {
            Some(ref mut internal_id_gen) => internal_id_gen,
            None => panic!("Tried to take an IdentifierGenerator without initializing it"),
        }
    }
}

pub trait GeneratesId {
    fn generate() -> Self;
}

impl GeneratesId for InputPinId {
    fn generate() -> Self {
        unsafe { ID_GEN.next_input_pin() }
    }
}

impl GeneratesId for OutputPinId {
    fn generate() -> Self {
        unsafe { ID_GEN.next_output_pin() }
    }
}

impl GeneratesId for NodeId {
    fn generate() -> Self {
        unsafe { ID_GEN.next_node() }
    }
}

impl GeneratesId for LinkId {
    fn generate() -> Self {
        unsafe { ID_GEN.next_link() }
    }
}
