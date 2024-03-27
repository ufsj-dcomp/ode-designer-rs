use std::{
    io::Read,
    process::{Command, Stdio},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PythonError {
    #[error("Failed to start Python process: {0}")]
    FailedToStartProcess(std::io::Error),
    #[error("Failed to read Python process output: {0}")]
    FailedToReadOutput(std::io::Error),
    #[error("Python process output is not available (is it empty?)")]
    OutputNotAvailable,
    #[error("Python process failed with status code {0}")]
    ProcessFailed(i32),
}

pub fn execute_python_code(command: &mut Command) -> Result<String, PythonError> {
    let mut python_process = command
        .stdout(Stdio::piped())
        .spawn()
        .map_err(PythonError::FailedToStartProcess)?;

    let mut output = String::new();
    let mut stdout = python_process
        .stdout
        .take()
        .ok_or(PythonError::OutputNotAvailable)?;

    stdout
        .read_to_string(&mut output)
        .map_err(PythonError::FailedToReadOutput)?;

    let status = python_process.wait().unwrap();
    if status.success() {
        Ok(output)
    } else {
        Err(PythonError::ProcessFailed(
            status.code().unwrap_or_default(),
        ))
    }
}
