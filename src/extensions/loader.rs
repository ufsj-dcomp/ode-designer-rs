use std::{
    io::Read,
    process::{Command, Stdio},
};

use minijinja::{context, Environment, Error as JinjaError};
use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

use crate::core::python::{execute_python_code, PythonError};

const INSPECTOR_TEMPLATE: &str = include_str!("templates/inspect_node_functions.py.jinja");

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct NodeFunction {
    pub name: String,
    pub required_arguments: Vec<String>,
    pub features_variadic: bool,
    pub format: Option<String>,
}

#[derive(Debug, Error)]
#[error("{0}")]
pub enum InspectionError {
    Template(JinjaError),
    Python(PythonError),
    Deserialization(SerdeJsonError),
}

pub fn inspect_user_code(user_code: &str) -> Result<Vec<NodeFunction>, InspectionError> {
    use InspectionError::*;

    let env = Environment::new();
    let mut ctx = context! {
        user_code,
    };

    let py_code = env
        .render_str(INSPECTOR_TEMPLATE, &mut ctx)
        .map_err(Template)?;

    let inspection_res =
        execute_python_code(Command::new("python3").arg("-c").arg(py_code)).map_err(Python)?;

    serde_json::from_str(&inspection_res).map_err(Deserialization)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(r"
import math

@node
def sen(x):
    return math.sin(x)
", &[
        NodeFunction {
            name: "sen".to_string(),
            required_arguments: vec!["x".to_string()],
            features_variadic: false,
            format: None,
        }
    ])]
    #[case(r"
@node(format='$x ^ $y')
def pow(x, y):
    return x ** y
", &[
        NodeFunction {
            name: "pow".to_string(),
            required_arguments: ["x", "y"].into_iter().map(String::from).collect(),
            features_variadic: false,
            format: Some("$x ^ $y".to_string())
        }
    ])]
    fn test_node_functions(
        #[case] user_code_input: &str,
        #[case] expected_node_funcs: &[NodeFunction],
    ) {
        let node_funcs = inspect_user_code(user_code_input).unwrap();
        assert_eq!(&node_funcs, expected_node_funcs)
    }
}
