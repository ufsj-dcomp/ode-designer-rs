use std::collections::HashMap;

#[derive(Clone,Debug)]
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
}

use std::fmt;

use crate::lexer;
use crate::parser::Parser;

pub(crate) type Result<T> = std::result::Result<T, ExprError>;

#[derive(Debug, Clone)]
pub enum ExprError{
    UndefinedAST,
    EvaluationError,
    UndefinedVarError(String),
    UndefinedFunctionError(String)
}

impl fmt::Display for ExprError {    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprError::UndefinedAST => write!(f, "The abstract syntax tree is not defined!"),
            ExprError::EvaluationError => write!(f, "Evaluation error ocurred! Error evaluating an expression."),
            ExprError::UndefinedVarError(var) => write!(f, "The var {} was not defined", var),
            ExprError::UndefinedFunctionError(func) => write!(f, "The function {} was not defined", func),            
        }        
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Constant,
    Var,
    Function
}

#[derive(Clone,Debug)] 
pub struct LeafNode {
    pub node_type: NodeType,
    pub name: String,
    pub value: f64,
    pub args: Vec<Box<Node>>
}

impl LeafNode {

    pub fn new(node_type: NodeType, name: String) -> Self {
        Self {
            node_type: node_type,
            name: name,
            value: 0.0,
            args: vec![]
        }
    }
}

#[derive(Clone,Debug)] 
pub enum Node {
    Leaf(LeafNode), //constant or population
    UnaryExpr {
        op: Operator,
        expr: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        left_expr: Box<Node>,
        right_expr: Box<Node>,
    }
}

impl Node {
    
    pub fn eval(&self, context: &ExprContext) -> Result<f64> {
        match self {
            Node::Leaf(n) => {
                match n.node_type {
                    NodeType::Constant => Ok(n.value),
                    NodeType::Var => context.get_var(n.name.clone()),
                    NodeType::Function => {
                        match context.get_function(n.name.clone()){
                            Ok(f_ptr) => {
                                let mut f_args: Vec<f64> = vec![];
                                for arg in n.args.iter() {
                                    match arg.eval(context){
                                        Ok(v) => f_args.push(v),
                                        Err(_) => (),
                                    }
                                }
                                f_ptr(f_args)
                            },
                            Err(e) => {println!("Error: {}", e); Err(e)},
                        }                        
                    },
                }
            },
            Node::UnaryExpr { op, expr } => {
                let value = expr.eval(context);
                match op {
                    Operator::Plus => value,
                    Operator::Minus => {
                        match value {
                            Ok(v) => Ok(- v),
                            Err(_) => Err(ExprError::EvaluationError)
                        }                        
                    },
                    _ => value,
                }
            },
            Node::BinaryExpr {op, left_expr, right_expr } => {
                let left_expr_value: f64 = left_expr.eval(context).unwrap();                
                let right_expr_value: f64 = right_expr.eval(context).unwrap();
                
                match op {
                    Operator::Plus => Ok(left_expr_value + right_expr_value),
                    Operator::Minus => Ok(left_expr_value - right_expr_value),
                    Operator::Mult => Ok(left_expr_value * right_expr_value),
                    Operator::Div => {
                        if right_expr_value == 0.0 {
                            Err(ExprError::EvaluationError)
                        }
                        else {
                            return Ok(left_expr_value / right_expr_value)
                        }                        
                    },
                }
            }
        }
    }
}

type Func = fn(Vec<f64>) -> Result<f64>;

pub fn sqrt(values: Vec<f64>) -> Result<f64>{
    return Ok(f64::sqrt(values[0]));
}

pub fn exp(values: Vec<f64>) -> Result<f64>{
    return Ok(f64::exp(values[0]));
}

pub fn pow(values: Vec<f64>) -> Result<f64>{
    return Ok(f64::powf(values[0], values[1]));
}

#[derive(Debug,Clone)]
pub struct ExprContext {
	pub vars: HashMap<String, f64>,
	pub funcs: HashMap<String, Func>,
}

impl ExprContext{

    pub fn new() -> Self{
        Self {
            vars: HashMap::new(),
            funcs: {
                let mut functions: HashMap<String, Func> = HashMap::new();
                functions.insert(String::from("sqrt"), sqrt);
                functions.insert(String::from("exp"), exp);
                functions.insert(String::from("pow"), pow);
                functions
            }
        }
    }

    pub fn get_var(&self, name: String) -> Result<f64>{
        match self.vars.get(&name){
            Some(var) => Ok(var.clone()),
            None => Err(ExprError::UndefinedVarError(name)),
        }
    }

    pub fn set_var(&mut self, name: String, value: f64){
        self.vars.insert(name,value);
    }

    pub fn get_function(&self, name: String) -> Result<Func> {
        match self.funcs.get(&name){
            Some(f) => Ok(f.clone()),
            None => Err(ExprError::UndefinedFunctionError(name)),
        }
    }

    pub fn set_func(&mut self, name: String, f: Func){
        self.funcs.insert(name,f);
    }

}

#[derive(Debug, Clone)]
pub struct Expression {
	pub context: ExprContext,
	pub ast: Option<Box<Node>>,
}

impl Expression {

    pub fn new() -> Self {
        Self {
            context: ExprContext::new(),
            ast:  None,
        }
    }

    pub fn set_context(&mut self, ctx: ExprContext) {
        self.context = ctx;
    }

    pub fn parse_expr(&mut self, text: String) -> Result<bool>{
        //parse the expression and creates the ast tree        
        let tokens = lexer::tokenize_string(text);
        let mut parser = Parser::new(tokens);
        self.ast = Some(parser.parse());

        Ok(true)
    }

    pub fn eval(&self) -> Result<f64>{
        //call eval on root node
        if self.ast.is_some() {
            if let Some(ast) = &self.ast{
                return ast.eval(&self.context);
            }
        }
        return Err(ExprError::UndefinedAST);
    }

}
