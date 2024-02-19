use std::{collections::BTreeMap, fmt::Write, hash::Hash};

use strum::EnumString;

use crate::extensions::format::Format;

#[derive(Debug, Clone)]
pub struct ExpressionTree<OriginType: Hash> {
    pub unary_op: Sign,
    join_op: Operation,
    pub format: Format,
    pub members: BTreeMap<OriginType, ExpressionNode<OriginType>>,
}

impl<OriginType: Hash> Default for ExpressionTree<OriginType> {
    fn default() -> Self {
        let join_op = Default::default();
        Self {
            join_op,
            format: join_op.into(),
            members: Default::default(),
            unary_op: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionNode<OriginType: Hash> {
    Leaf(Leaf),
    SubExpr(ExpressionTree<OriginType>),
}

#[derive(Debug, Clone)]
pub struct Leaf {
    pub symbol: String,
    pub unary_op: Sign,
}

#[derive(Default, Hash, Debug, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum Sign {
    #[default]
    #[strum(serialize = "+")]
    Positive,
    #[strum(serialize = "-")]
    Negative,
}

impl Sign {
    pub fn toggle(&mut self) {
        *self = match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }

    pub fn to_multiplier(self) -> f64 {
        match self {
            Sign::Positive => 1.0,
            Sign::Negative => -1.0,
        }
    }
}

impl From<Sign> for char {
    fn from(value: Sign) -> Self {
        match value {
            Sign::Positive => '+',
            Sign::Negative => '-',
        }
    }
}

#[derive(Debug)]
pub struct NotASign;

impl TryFrom<char> for Sign {
    type Error = NotASign;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Positive),
            '-' => Ok(Self::Negative),
            _ => Err(NotASign),
        }
    }
}

impl<OriginType: Hash> ExpressionTree<OriginType> {
    pub fn resolve_into_equation(&self) -> String {
        let mut result = self.format.format_args(
            self.members
                .values()
                .map(ExpressionNode::resolve_into_equation_part)
                .collect(),
        );

        if let Sign::Negative = self.unary_op {
            result.insert_str(0, "-(");
            result.push(')');
        }

        result
    }

    pub fn join_op(&self) -> Operation {
        self.join_op
    }

    pub fn set_join_op(&mut self, join_op: Operation) {
        self.join_op = join_op;
        self.format = join_op.into();
    }
}

impl<OriginType: Hash> ExpressionNode<OriginType> {
    pub fn resolve_into_equation_part(&self) -> String {
        match self {
            ExpressionNode::Leaf(Leaf { symbol, unary_op }) => match unary_op {
                Sign::Positive => symbol.clone(),
                Sign::Negative => format!("-{}", symbol),
            },
            ExpressionNode::SubExpr(exprtree) => {
                let mut eq: String = exprtree.resolve_into_equation();
                eq.insert(0, '(');
                eq.push(')');
                eq
            }
        }
    }

    pub fn get_unary(&self) -> &Sign {
        match self {
            ExpressionNode::Leaf(Leaf { unary_op, .. })
            | ExpressionNode::SubExpr(ExpressionTree { unary_op, .. }) => unary_op,
        }
    }

    pub fn set_unary(&mut self, new_unary: Sign) {
        let unary_op = match self {
            ExpressionNode::Leaf(Leaf { unary_op, .. })
            | ExpressionNode::SubExpr(ExpressionTree { unary_op, .. }) => unary_op,
        };

        *unary_op = new_unary;
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::EnumIter,
    strum::VariantNames,
    strum::EnumString,
    strum::FromRepr,
    strum::VariantArray,
)]
#[repr(u8)]
pub enum Operation {
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "-")]
    Sub,
    #[strum(serialize = "/", serialize = "÷")]
    Div,
    #[default]
    #[strum(serialize = "*", serialize = "×")]
    Mul,
}

pub struct NotAnOperationChar;

impl TryFrom<char> for Operation {
    type Error = NotAnOperationChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '/' => Ok(Self::Div),
            '*' => Ok(Self::Mul),
            _ => Err(NotAnOperationChar),
        }
    }
}

impl From<Operation> for char {
    fn from(value: Operation) -> Self {
        match value {
            Operation::Add => '+',
            Operation::Sub => '-',
            Operation::Div => '/',
            Operation::Mul => '*',
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = (*self).into();
        f.write_char(c)
    }
}

impl From<Operation> for Format {
    fn from(value: Operation) -> Self {
        format!("$@{value}").parse().unwrap()
    }
}
