use std::{
    io::Read,
    process::{Command, Stdio},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PythonError {
    #[error("Failed to start Python process: {0}")]
    FailedToStartProcess(std::io::Error),
    #[error("Failed to read Python output: {0}")]
    FailedToReadStdout(std::io::Error),
    #[error("Python process output is not available (is it empty?)")]
    StdoutNotAvailable,
    #[error("Python error: {0}")]
    ProcessFailed(String),
    #[error("Failed to read Python error: {0}")]
    FailedToReadStderr(std::io::Error),
    #[error("Python process error is not available (is it empty?)")]
    StderrNotAvailable,
}

pub fn execute_python_code(command: &mut Command) -> Result<String, PythonError> {
    let mut python_process = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(PythonError::FailedToStartProcess)?;

    let mut output = String::new();
    let mut stdout = python_process
        .stdout
        .take()
        .ok_or(PythonError::StdoutNotAvailable)?;

    stdout
        .read_to_string(&mut output)
        .map_err(PythonError::FailedToReadStdout)?;

    let status = python_process.wait().unwrap();
    if status.success() {
        Ok(output)
    } else {
        let mut error = String::new();
        python_process.stderr
            .take()
            .ok_or(PythonError::StderrNotAvailable)?
            .read_to_string(&mut error)        
            .map_err(PythonError::FailedToReadStderr)?;

        match error.trim_end().rsplit_once('\n') {
            Some((_,err)) => Err(PythonError::ProcessFailed(err.to_string())),
            None => Err(PythonError::StderrNotAvailable),
        }
        
    }
}
