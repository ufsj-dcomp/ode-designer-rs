use std::{fs::File, io::Read, str::FromStr};

use rfd::FileDialog;

use crate::core::App;

use self::{format::Format, loader::NodeFunction};

mod format;
mod loader;

pub struct Extension {
    pub filename: String,
    pub nodes: Vec<CustomNodeSpecification>,
}

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

impl<'n> App<'n> {
    pub fn load_extension_file(&mut self) -> color_eyre::Result<()> {
        let file_path = FileDialog::new()
            .add_filter("Python", &["py"])
            .pick_file()
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "Could not open file")
            })?;

        let mut file = File::open(&file_path)?;
        let mut user_code = String::new();

        file.read_to_string(&mut user_code)?;

        let node_specs = loader::inspect_user_code(&user_code)?
            .into_iter()
            .map(CustomNodeSpecification::from)
            .collect();
        
        self.extensions.push(Extension {
            filename: file_path
                .file_name()
                .map(std::ffi::OsStr::to_string_lossy)
                .map(Into::into)
                .unwrap_or_else(|| String::from("Unknown")),
            nodes: node_specs,
        });

        Ok(())
    }
}