use std::str::FromStr;

use self::{format::Format, loader::NodeFunction};

mod format;
mod loader;

pub struct CustomNodeSpecification {
    pub function: NodeFunction,
    pub format: Format,
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
