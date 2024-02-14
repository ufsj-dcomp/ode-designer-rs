use std::{io::Read, process::{Command, Stdio}};

use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use minijinja::{Environment, context, Error as JinjaError};

const INSPECTOR_TEMPLATE: &str = include_str!("templates/inspect_node_functions.py.jinja");

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct NodeFunction {
    pub name: String,
    pub required_arguments: Vec<String>,
    pub features_variadic: bool,
    pub format: Option<String>,
}

#[derive(Debug)]
enum InspectionError {
    TemplateError(JinjaError),
    PythonInvocationError(std::io::Error),
    PythonExecutionError(String),
    DeserializationError(SerdeJsonError),
}

fn inspect_user_code(user_code: &str) -> Result<Vec<NodeFunction>, InspectionError> {
    use InspectionError::*;

    let env = Environment::new();
    let mut ctx = context! {
        user_code,
    };

    let py_code = env.render_str(INSPECTOR_TEMPLATE, &mut ctx)
        .map_err(TemplateError)?;

    let python_out = Command::new("python3")
        .arg("-c")
        .arg(py_code)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(PythonInvocationError)?;

    let mut inspection_res = String::new();

    python_out.stderr.unwrap().read_to_string(&mut inspection_res).unwrap();

    if !inspection_res.is_empty() {
        return Err(PythonExecutionError(inspection_res));
    }
        
    inspection_res.clear();
    python_out.stdout.unwrap().read_to_string(&mut inspection_res).unwrap();

    serde_json::from_str(&inspection_res)
        .map_err(DeserializationError)

}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

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
    fn test_node_functions(#[case] user_code_input: &str, #[case] expected_node_funcs: &[NodeFunction]) {
        let node_funcs = inspect_user_code(user_code_input).unwrap();
        assert_eq!(&node_funcs, expected_node_funcs)
    }
}
