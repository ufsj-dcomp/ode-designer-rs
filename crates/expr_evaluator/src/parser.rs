use crate::{expr::{LeafNode, Node, NodeType, Operator}, lexer::*};
use crate::expr::Operator::*;

pub enum ParserError {
    FunctionNameNotFoundError
}

#[derive(Clone,Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    c_token: Token,
    last_token: Token,
    index: usize,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            c_token: Token::new(0, 0, 0, TokenKind::Error(String::from(""))),
            last_token: Token::new(0, 0, 0, TokenKind::Error(String::from(""))),
            index: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        if self.index < self.tokens.len() {
            self.last_token = self.tokens.get(self.index).unwrap().clone(); 
            self.index += 1;
        }        
        return self.last_token.clone();
    }

    fn back_token(&mut self) {
        self.index -= 1;
    }

    fn expr(&mut self) -> Box<Node>{
        let node: Box<Node> = self.termo();
        return self.adicao_opc(node);
    }

    fn termo(&mut self) -> Box<Node>{
        let node = self.fator();
        return self.termo_opc(node);
    }

    fn adicao_opc(&mut self, node: Box<Node>) -> Box<Node>{
        self.c_token = self.next_token();
        
        match self.c_token.token_type.clone() {            
            TokenKind::Operators(Operators::Plus) => {      
                let right_node = self.termo();
                let binary_node = Box::new(Node::BinaryExpr { op: Operator::Plus, left_expr: node, right_expr: right_node });
                return self.adicao_opc(binary_node);
            },
            TokenKind::Operators(Operators::Minus) => {
                let right_node = self.termo();
                let binary_node = Box::new(Node::BinaryExpr { op: Operator::Minus, left_expr: node, right_expr: right_node });
                return self.adicao_opc(binary_node);
            },
            _ => {
                self.back_token();
                return node
            }
        }
    }

    fn termo_opc(&mut self, node: Box<Node>) -> Box<Node>{
        self.c_token = self.next_token();
        match self.c_token.token_type.clone() {
            TokenKind::Operators(Operators::Multiplication) => {
                let right_node = self.fator();
                let binary_node = Box::new(Node::BinaryExpr { op: Operator::Mult, left_expr: node, right_expr: right_node });
                return self.termo_opc(binary_node);
            },
            TokenKind::Operators(Operators::Division) => {
                let right_node = self.fator();
                let binary_node = Box::new(Node::BinaryExpr { op: Operator::Div, left_expr: node, right_expr: right_node });
                return self.termo_opc(binary_node);
            },
            _ => {
                self.back_token();
                return node
            } 
        }
    }

    fn fator(&mut self) -> Box<Node>{        
        let mut is_unary: bool = false;
        let mut is_minus: bool = false;
        self.c_token = self.next_token();

        match self.c_token.token_type.clone(){
            TokenKind::Operators(Operators::Minus) => {
                is_unary = true;
                is_minus = true;
            }
            TokenKind::Operators(Operators::Plus) => {
                is_unary = true;
            }
            _ => (),
        }
        
        let node: Box<Node> = self.fator2(is_unary);
        if is_minus {
            return Box::new(Node::UnaryExpr { op: Minus, expr: node });            
        }
        else {
            return node;
        }
    }

    fn fator2(&mut self, is_unary: bool) -> Box<Node>{
        if ! is_unary {
            self.back_token();
        }
        self.c_token = self.next_token();
        
        match self.c_token.token_type.clone() {
            TokenKind::Identifier(lexeme) =>  {
                self.c_token = self.next_token(); 
                match self.c_token.token_type.clone(){
                    TokenKind::Punctuation(Punctuation::LParen) =>{
                        return self.chamada_funcao(lexeme);
                    },
                    _ => { 
                        self.back_token();
                        return Box::new(Node::Leaf(LeafNode { node_type: NodeType::Var, name: lexeme, value: 0.0, args: vec![] }))
                    }
                }
            },
            TokenKind::FloatConst(value) => {
                return Box::new(Node::Leaf(LeafNode { node_type: NodeType::Constant, name: value.to_string(), value: value, args: vec![] }))
            },
            TokenKind::Punctuation(Punctuation::LParen) => {
                let node = self.expr();
                self.c_token = self.next_token(); 
                if self.c_token.token_type != TokenKind::Punctuation(Punctuation::RParen) {
                    println!("Erro sintatico na linha {}. ) esperado na entrada.", self.c_token.line_number);
                }
                return node;
            },
            _ => {
                println!("Erro sintatico na linha {}. Id, ( ou constante float esperados na entrada.", self.c_token.line_number);
                return Box::new(Node::Leaf(LeafNode::new(NodeType::Var, String::from("Error"))))
            }
        }
    }

    fn chamada_funcao(&mut self, function_name: String) -> Box<Node>{

        let mut function_node = LeafNode::new(NodeType::Function, function_name);
        let args = self.lista_args();
        function_node.args = args;
        
        if self.c_token.token_type != TokenKind::Punctuation(Punctuation::RParen) {
            println!("Erro sintatico na linha {}. ) esperado na entrada.", self.c_token.line_number);
        }
        
        return Box::new(Node::Leaf(function_node)) 
    }

    fn lista_args(&mut self) -> Vec<Box<Node>>{
        let mut args: Vec<Box<Node>> = vec![];
        self.c_token = self.next_token(); 

        match self.c_token.token_type.clone() {
            TokenKind::Operators(Operators::Plus) | TokenKind::Operators(Operators::Minus) | TokenKind::Identifier(_)
            | TokenKind::FloatConst(_) | TokenKind::Punctuation(Punctuation::LParen) => {
                self.back_token();
                let node = self.expr();
                args.push(node);
                args = self.lista_args2(args);
            }            
            _ => ()            
        }

        return args
    }

    fn lista_args2(&mut self, mut args: Vec<Box<Node>>) -> Vec<Box<Node>>{
        self.c_token = self.next_token();

        match self.c_token.token_type.clone() {
            TokenKind::Punctuation(Punctuation::Comma) => {
                let node = self.expr();
                args.push(node);
                args = self.lista_args2(args);
            },
            _ => () 
        }
        return args
    }

    pub fn parse(&mut self) -> Box<Node>{
        return self.expr();
    }

}
