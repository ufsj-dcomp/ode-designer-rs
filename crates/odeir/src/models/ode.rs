use serde::{Deserialize, Serialize};

use super::CoreModel;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Metadata {
    pub start_time: f64,
    pub delta_time: f64,
    pub end_time: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OdeModel {
    pub name: String,
    pub metadata: Metadata,
    pub extension_files: Vec<String>,
    #[serde(flatten)]
    pub core: CoreModel,
}

impl OdeModel {
    pub fn new(name: String, metadata: Metadata) -> Self {
        Self {
            name,
            metadata,
            core: CoreModel::new(),
            extension_files: Vec::new(),
        }
    }
}

impl std::ops::Deref for OdeModel {
    type Target = CoreModel;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl std::ops::DerefMut for OdeModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.core
    }
}
