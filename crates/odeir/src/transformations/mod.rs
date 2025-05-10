use crate::models::Argument;

pub mod ode;
pub mod r4k;

impl crate::models::CoreModel {
    pub fn get_arguments_where<'a>(
        &'a self,
        mut cb: impl FnMut(&'a Argument) -> bool,
    ) -> impl Iterator<Item = &'a Argument> {
        self.arguments.values().filter(move |node| cb(node))
    }
    pub fn get_populations(&self) -> impl Iterator<Item = &'_ Argument> {
        self.get_arguments_where(|arg| match arg {
            Argument::Value { name, .. } => self
                .equations
                .iter()
                .any(|eq| eq.operates_on.as_ref() == Some(name)),
            _ => false,
        })
    }
    pub fn get_constants(&self) -> impl Iterator<Item = &'_ Argument> {
        self.get_arguments_where(|arg| match arg {
            Argument::Value { name, .. } => self
                .equations
                .iter()
                .all(|eq| eq.operates_on.as_ref() != Some(name)),
            _ => false,
        })
    }
}
