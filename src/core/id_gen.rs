use imnodes::{IdentifierGenerator, InputPinId, LinkId, NodeId, OutputPinId};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;

struct IdGenWrapper {
    internal_id_gen: IdentifierGenerator,
}

static ID_GEN: OnceCell<RwLock<IdGenWrapper>> = OnceCell::new();

pub fn initialize_id_generator(internal_id_gen: IdentifierGenerator) {
    ID_GEN
        .set(RwLock::new(IdGenWrapper { internal_id_gen }))
        .map_err(|_| panic!("Tried to initialize ID_GEN twice. Shame on you."))
        .unwrap();
}

pub trait GeneratesId {
    fn generate() -> Self;
}

impl GeneratesId for InputPinId {
    fn generate() -> Self {
        ID_GEN
            .get()
            .unwrap()
            .write()
            .internal_id_gen
            .next_input_pin()
    }
}

impl GeneratesId for OutputPinId {
    fn generate() -> Self {
        ID_GEN
            .get()
            .unwrap()
            .write()
            .internal_id_gen
            .next_output_pin()
    }
}

impl GeneratesId for NodeId {
    fn generate() -> Self {
        ID_GEN.get().unwrap().write().internal_id_gen.next_node()
    }
}

impl GeneratesId for LinkId {
    fn generate() -> Self {
        ID_GEN.get().unwrap().write().internal_id_gen.next_link()
    }
}
