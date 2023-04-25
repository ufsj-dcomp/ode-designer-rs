
use derive_more::From;

use crate::{nodes::{Data, PinId}, app::Link};

#[derive(Debug, Clone)]
pub struct SendData {
    pub data: Data,
    pub from_output: PinId,
    pub to_input: PinId,
}

#[derive(Debug, Clone, From)]
pub enum Message {
    SendData(SendData),
    AddLink(Link),
}
