use serde::{Deserialize, Serialize};

use crate::{
    models::{self, cellular_automata::CaModel, ode::OdeModel, Argument, CoreModel, Equation},
    Map,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Json {
    pub metadata: Metadata,
    pub arguments: Vec<Argument>,
    pub equations: Vec<Equation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ModelMetadata {
    #[serde(rename = "ode")]
    ODE(models::ode::Metadata),
    #[serde(rename = "cellular-automata")]
    CellularAutomata {},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    #[serde(default)]
    pub name: String,
    #[serde(flatten)]
    pub model_metadata: ModelMetadata,
    #[serde(default)]
    pub positions: Map<String, Position>,
    #[serde(default)]
    pub extension_files: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "Json")]
#[serde(into = "Json")]
pub enum Model {
    ODE(OdeModel),
    CellularAutomata(CaModel),
}

impl From<Json> for Model {
    fn from(value: Json) -> Self {
        let core = CoreModel {
            arguments: value
                .arguments
                .into_iter()
                .map(|arg| (arg.name().to_owned(), arg))
                .collect(),
            equations: value.equations,
            positions: value.metadata.positions,
        };
        let name = value.metadata.name;
        match value.metadata.model_metadata {
            ModelMetadata::CellularAutomata {} => Self::CellularAutomata(CaModel { name, core }),
            ModelMetadata::ODE(metadata) => Self::ODE(OdeModel {
                name,
                core,
                metadata,
                extension_files: value.metadata.extension_files,
            }),
        }
    }
}

impl From<Model> for Json {
    fn from(value: Model) -> Self {
        let (equations, model_metadata, name, extension_files) = match value {
            Model::CellularAutomata(model) => (
                model.core,
                ModelMetadata::CellularAutomata {},
                model.name,
                Vec::new(),
            ),
            Model::ODE(model) => (
                model.core,
                ModelMetadata::ODE(model.metadata),
                model.name,
                model.extension_files,
            ),
        };
        Self {
            arguments: equations.arguments.values().cloned().collect(),
            equations: equations.equations,
            metadata: Metadata {
                name,
                model_metadata,
                extension_files,
                positions: Map::new(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_eq;

    use super::*;

    use crate::models::Component;

    const GAME_OF_LIFE: &str = include_str!("../fixtures/game-of-life.json");

    /* fn fixture_game_of_life() -> Json {
        Json {
            metadata: Metadata {
                name: "Conway's Game of Life".into(),
                model_metadata: ModelMetadata::CellularAutomata {},
                positions: Map::new(),
            },
            arguments: vec![
                Argument::Value {
                    name: "dead".into(),
                    value: 0.5,
                },
                Argument::Value {
                    name: "alive".into(),
                    value: 0.5,
                },
                Argument::Composite {
                    name: "reproduction".into(),
                    operation: "==".into(),
                    composition: vec![
                        Component::Argument {
                            name: "alive".into(),
                            contribution: '+',
                        },
                        Component::Constant {
                            value: 3.0,
                            contribution: '+',
                        },
                    ],
                },
                Argument::Composite {
                    name: "overpopulation".into(),
                    operation: ">".into(),
                    composition: vec![
                        Component::Argument {
                            name: "alive".into(),
                            contribution: '+',
                        },
                        Component::Constant {
                            value: 3.0,
                            contribution: '+',
                        },
                    ],
                },
                Argument::Composite {
                    name: "underpopulation".into(),
                    operation: "<".into(),
                    composition: vec![
                        Component::Argument {
                            name: "alive".into(),
                            contribution: '+',
                        },
                        Component::Constant {
                            value: 2.0,
                            contribution: '+',
                        },
                    ],
                },
                Argument::Composite {
                    name: "dead_equation".into(),
                    operation: "+".into(),
                    composition: vec![
                        Component::Argument {
                            name: "overpopulation".into(),
                            contribution: '+',
                        },
                        Component::Argument {
                            name: "underpopulation".into(),
                            contribution: '+',
                        },
                        Component::Argument {
                            name: "reproduction".into(),
                            contribution: '-',
                        },
                    ],
                },
                Argument::Composite {
                    name: "alive_equation".into(),
                    operation: "-".into(),
                    composition: vec![
                        Component {
                            value: 1.0,
                            contribution: '+',
                        },
                        Component {
                            name: "dead_equation".into(),
                            contribution: '+',
                        },
                    ],
                },
            ],
            equations: [
                Equation {
                    name: "dead_equation".to_string(),
                    operates_on: Some("dead".to_string()),
                    argument: "dead_equation".to_string(),
                    contribution: '+',
                },
                Equation {
                    name: "alive_equation".to_string(),
                    operates_on: Some("alive".to_string()),
                    argument: "alive_equation".to_string(),
                    contribution: '+',
                },
            ]
            .to_vec()
        }
    } */

    /* #[test]
    fn deserialize_game_of_life_json() {
        let json = serde_json::from_str::<Json>(GAME_OF_LIFE).unwrap();
        let expected = fixture_game_of_life();
        assert_json_eq!(expected, json);
    }

    #[test]
    fn serialize_game_of_life_json() {
        let serialized = serde_json::to_string(&fixture_game_of_life()).unwrap();
        let expected = include_str!("../fixtures/game-of-life-serialized.json");
        assert_eq!(serialized, expected);
    } */

    /* #[test]
    fn deserialize_game_of_life_model() {
        let model = serde_json::from_str::<Model>(GAME_OF_LIFE).unwrap();
        let expected = crate::models::cellular_automata::Model {
            name: "Conway's Game of Life".into(),
            equations: Equations {
                arguments: [
                    Argument::Value {
                        name: "dead".into(),
                        value: 0.5,
                    },
                    Argument::Value {
                        name: "alive".into(),
                        value: 0.5,
                    },
                    Argument::Composite {
                        name: "reproduction".into(),
                        operation: "==".into(),
                        composition: vec![
                            Component::Argument {
                                name: "alive".into(),
                                contribution: '+',
                            },
                            Component::Constant {
                                value: 3.0,
                                contribution: '+',
                            },
                        ],
                    },
                    Argument::Composite {
                        name: "overpopulation".into(),
                        operation: ">".into(),
                        composition: vec![
                            Component::Argument {
                                name: "alive".into(),
                                contribution: '+',
                            },
                            Component::Constant {
                                value: 3.0,
                                contribution: '+',
                            },
                        ],
                    },
                    Argument::Composite {
                        name: "underpopulation".into(),
                        operation: "<".into(),
                        composition: vec![
                            Component::Argument {
                                name: "alive".into(),
                                contribution: '+',
                            },
                            Component::Constant {
                                value: 2.0,
                                contribution: '+',
                            },
                        ],
                    },
                    Argument::Composite {
                        name: "dead_equation".into(),
                        operation: "+".into(),
                        composition: vec![
                            Component::Argument {
                                name: "overpopulation".into(),
                                contribution: '+',
                            },
                            Component::Argument {
                                name: "underpopulation".into(),
                                contribution: '+',
                            },
                            Component::Argument {
                                name: "reproduction".into(),
                                contribution: '-',
                            },
                        ],
                    },
                    Argument::Composite {
                        name: "alive_equation".into(),
                        operation: "-".into(),
                        composition: vec![
                            Component::Constant {
                                value: 1.0,
                                contribution: '+',
                            },
                            Component::Argument {
                                name: "dead_equation".into(),
                                contribution: '+',
                            },
                        ],
                    },
                ]
                .into_iter()
                .map(|arg| (arg.name().to_owned(), arg))
                .collect(),
                equations: vec![("dead", "dead_equation"), ("alive", "alive_equation")]
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect()
            },
        };
        let expected = Json::from(Model::CellularAutomata(expected));
        assert_json_eq!(model, expected);
    } */
}
