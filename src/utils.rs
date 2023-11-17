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
