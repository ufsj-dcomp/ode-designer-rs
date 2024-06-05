use imnodes::ImVec2;

macro_rules! fluent_map {
    ($($key:expr => $val:expr),* $(,)?) => {
        std::collections::HashMap::from_iter([
            $(($key, fluent_bundle::FluentValue::from($val))),*
        ])
    };
}

macro_rules! localized_error {
    ($locale:expr, $text:expr $(,)?) => {
        log::error!("{}.", $locale.get($text))
    };
    ($locale:expr, $text:expr, $($tail:tt)*) => {
        log::error!("{}.", &*$locale.fmt($text, &$crate::utils::fluent_map!{$($tail)*}))
    };
}

pub(crate) use fluent_map;
pub(crate) use localized_error;

#[derive(Debug)]
pub enum ModelFragment {
    Argument(odeir::Argument),
    Equation(odeir::Equation),
}

impl From<odeir::Argument> for ModelFragment {
    fn from(value: odeir::Argument) -> Self {
        ModelFragment::Argument(value)
    }
}

impl From<odeir::Equation> for ModelFragment {
    fn from(value: odeir::Equation) -> Self {
        ModelFragment::Equation(value)
    }
}

pub trait VecConversion<To> {
    fn convert(self) -> To;
}

impl VecConversion<odeir::Position> for ImVec2 {
    fn convert(self) -> odeir::Position {
        odeir::Position {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

impl VecConversion<[f32; 2]> for ImVec2 {
    fn convert(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl VecConversion<ImVec2> for odeir::Position {
    fn convert(self) -> ImVec2 {
        ImVec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl VecConversion<[f32; 2]> for odeir::Position {
    fn convert(self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }
}
