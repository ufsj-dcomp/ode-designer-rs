use std::str::FromStr;

use self::{format::Format, loader::NodeFunction};

mod format;
mod loader;

#[derive(Debug)]
pub struct CustomNodeSpecification {
    pub function: NodeFunction,
    pub format: Format,
}

impl CustomNodeSpecification {
    pub fn input_count(&self) -> usize {
        self.function.required_arguments.len()
    }
}

impl From<NodeFunction> for CustomNodeSpecification {
    fn from(mut function: NodeFunction) -> Self {
        let format = function
            .format
            .take()
            .and_then(|format| Format::from_str(&format).ok())
            .unwrap_or_else(|| Format::default_with_name(&function.name));

        Self {
            function,
            format,
        }
    }
}
