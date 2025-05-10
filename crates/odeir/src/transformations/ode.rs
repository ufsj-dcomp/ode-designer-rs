use std::path::PathBuf;

use minijinja::{context, Environment};

use crate::{models::ode::OdeModel, Map, Model};

/*const ODE_TEMPLATE: &str = include_str!("../../templates/ode.txt.jinja");

pub fn render_ode(model: Model) -> String {
    let env = Environment::new();

    let mut ctx = context! {
        model => model,
    };

    env.render_str(ODE_TEMPLATE, &mut ctx).unwrap()
}*/

const TXT_TEMPLATE: &str = include_str!("../../templates/ode.txt.jinja");

pub fn render_txt_with_equations(model: &OdeModel, extension_lookup_paths: &[&PathBuf]) -> String {
    let env = Environment::new();

    let populations = model.get_populations().collect::<Vec<_>>();
    let constants = model.get_constants().collect::<Vec<_>>();
    let equations = model
        .equations
        .iter()
        .cloned()
        .filter_map(|eq| Some((eq.operates_on.clone()?, eq)))
        .collect::<Map<_, _>>();

    let extensions: Vec<String> = model
        .extension_files
        .iter()
        .filter_map(|filename| {
            let filename_as_path = PathBuf::from(filename);
            let filename_as_path = &filename_as_path;
            let full_path = extension_lookup_paths
                .iter()
                .find(|path| path.ends_with(filename))
                .unwrap_or(&filename_as_path);

            std::fs::read_to_string(full_path).ok()
        })
        .collect();

    let mut ctx = context! {
        model => model,
        equations => equations,
        populations => populations,
        constants => constants,
        extensions => extensions,
    };

    env.render_str(TXT_TEMPLATE, &mut ctx).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_render_ode_abc_json() {
        use super::*;

        const ABC_JSON_STR: &str = include_str!("../../tests/fixtures/abc.json");

        let model = serde_json::from_str::<Model>(ABC_JSON_STR).unwrap();

        let ode = render_ode(model);

        const EXPECTED: &str = "CoreModel:

        - dA/dt =+ (A  * B ) +
        - dB/dt =- (A  * B ) +
        - dC/dt =+ (A  * B  / C ) +";

        assert_eq!(ode, EXPECTED);
    }
}
