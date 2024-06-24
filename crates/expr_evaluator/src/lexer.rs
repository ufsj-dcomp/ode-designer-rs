use std::collections::HashMap;
use std::{io};
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};
use std::path::Path; 

use std::str;
use serde::Serialize;
use serde::Deserialize;
use std::io::Error;

#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub enum ReservedWords {
    Function,
    Main,
    Let,
    Int,
    Float, 
    Char,
    If, 
    Else,
    While,
    Print,
    Println,
    Return,
}

#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub enum Punctuation {    
    Arrow,
    Dot,
    Colon,
    Semicolon,
    Comma,
    LBrace,
    RBrace,
    LParen,
    RParen,
}

#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub enum Operators {
    Assign,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Plus,
    Minus,
    Multiplication,
    Division,
    Arrow,
}
#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub enum TokenKind {
    Identifier(String),
    IntConst(i32),
    FloatConst(f64),
    CharConst(String),    
    FormattedString(String),
    ReservedWords(ReservedWords),
    Punctuation(Punctuation),
    Operators(Operators),
    Error(String)
}

impl From<String> for TokenKind {
    fn from(other: String) -> TokenKind {
        TokenKind::Identifier(other)
    }
}

impl<'a> From<&'a str> for TokenKind {
    fn from(other: &'a str) -> TokenKind {
        TokenKind::Identifier(other.to_string())
    }
}

impl From<i32> for TokenKind {
    fn from(other: i32) -> TokenKind {
        TokenKind::IntConst(other)
    }
}

impl From<f64> for TokenKind {
    fn from(other: f64) -> TokenKind {
        TokenKind::FloatConst(other)
    }
}

#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub struct Token {    
    pub start: usize, 
    pub end: usize,
    pub line_number: usize,
    pub token_type: TokenKind,
}

impl Token {
    pub fn new(start: usize, end: usize, ln: usize, token_type: TokenKind) -> Self {
        Self {
            start: start,
            end: end,
            line_number: ln,
            token_type: token_type
        }
    }
}

//peek next char withou consumming it
pub fn peek_next_char(data: &str) -> char {
    match data.chars().peekable().next() {
        Some(c) => c,
        None => ' '
    }
}

fn execute_predicate<F>(data: &str, mut pred: F) -> Result<(&str,usize), usize>  where F: FnMut(char) -> bool {
    let mut current_index = 0;

    for ch in data.chars() {
        let should_continue = pred(ch);

        if !should_continue {
            break;
        }

        current_index += ch.len_utf8();
    }

    if current_index == 0 {
        Err(0)
    } else {
        Ok((&data[..current_index],current_index))
    }
}

fn tokenize_ident_reservedword(data: &str) -> Result<(TokenKind,usize),usize> {
    let mut reserved_workds: HashMap<String,TokenKind> = HashMap::new();    
    reserved_workds.insert(String::from("fn"),TokenKind::ReservedWords(ReservedWords::Function));
    reserved_workds.insert(String::from("let"),TokenKind::ReservedWords(ReservedWords::Let));
    reserved_workds.insert(String::from("int"),TokenKind::ReservedWords(ReservedWords::Int));
    reserved_workds.insert(String::from("float"),TokenKind::ReservedWords(ReservedWords::Float));
    reserved_workds.insert(String::from("char"),TokenKind::ReservedWords(ReservedWords::Char));
    reserved_workds.insert(String::from("if"),TokenKind::ReservedWords(ReservedWords::If));
    reserved_workds.insert(String::from("else"),TokenKind::ReservedWords(ReservedWords::Else));
    reserved_workds.insert(String::from("while"),TokenKind::ReservedWords(ReservedWords::While));
    reserved_workds.insert(String::from("print"),TokenKind::ReservedWords(ReservedWords::Print));
    reserved_workds.insert(String::from("println"),TokenKind::ReservedWords(ReservedWords::Println));
    reserved_workds.insert(String::from("return"),TokenKind::ReservedWords(ReservedWords::Return));
    
    // identifiers can't start with a number
    match data.chars().next() {
        Some(ch) if ch.is_digit(10) => panic!("Identifiers can't start with a number"),
        None => panic!("EOF not expected"),
        _ => {},
    }

    let (s, bytes_read) = execute_predicate(data, |ch| 
            ch == '_' || ch.is_alphanumeric())?;
    
    let tok: TokenKind;
    if reserved_workds.contains_key(s){
        tok = reserved_workds.get(s).unwrap().clone();
    }
    else {
        tok = TokenKind::Identifier(s.to_string());
    }
    Ok((tok, bytes_read))
}

/// Tokenize a numeric literal.
fn tokenize_number(data: &str) -> Result<(TokenKind, usize),usize> {
    let mut seen_dot = false;

    let (decimal, bytes_read) = execute_predicate(data, |c| {
        if c.is_digit(10) {
            true
        } else if c == '.' {
            if !seen_dot {
                seen_dot = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    })?;

    let n: f64 = decimal.parse::<f64>().unwrap();
    Ok((TokenKind::FloatConst(n), bytes_read))
}

pub fn tokenize_assing_or_equal(data: &str) -> Result<(TokenKind, usize),usize> {
    let tok: (TokenKind, usize);
    let mut seen_equal = false;
    let mut count = 0;
    let (_str,_bytes_read) = execute_predicate(data, |c| {
        if count == 2{
            seen_equal = true;
            false
        }
        else if c == '=' {            
            count += 1;
            true
        }
        else {
            false
        }
    })?;
    if seen_equal{
        tok = (TokenKind::Operators(Operators::Equal), 2);
    }
    else {
        tok = (TokenKind::Operators(Operators::Assign), 1);
    }    
    Ok(tok)
}

pub fn tokenize_minus_or_arrow(data: &str) -> Result<(TokenKind, usize),usize> {
    let tok: (TokenKind, usize);
    let mut seen_arrow = false;
    let mut count = 0;
    let (_str,_bytes_read) = execute_predicate(data, |c| {
        if c == '-' {
            count += 1;
            true
        }
        else if count == 1 && c == '>' {
            seen_arrow = true;
            false
        }
        else {
            false
        }
    })?;
    if seen_arrow{
        tok = (TokenKind::Operators(Operators::Arrow), 2);
    }
    else {
        tok = (TokenKind::Operators(Operators::Minus), 1);
    }    
    Ok(tok)
}

pub fn tokenize_less_or_lessthan(data: &str) -> Result<(TokenKind, usize),usize> {
    let tok: (TokenKind, usize);
    let mut seen_equal = false;
    let mut count = 0;
    let (_str,_bytes_read) = execute_predicate(data, |c| {
        if c == '<' {
            count += 1;
            true
        }
        else if count == 1 && c == '=' {
            seen_equal = true;
            false
        }
        else {
            false
        }
    })?;
    if seen_equal{
        tok = (TokenKind::Operators(Operators::LessThanEqual), 2);
    }
    else {
        tok = (TokenKind::Operators(Operators::LessThan), 1);
    }    
    Ok(tok)
}

pub fn tokenize_greater_or_greaterthan(data: &str) -> Result<(TokenKind, usize),usize> {
    let tok: (TokenKind, usize);
    let mut seen_equal = false;
    let mut count = 0;
    let (_str,_bytes_read) = execute_predicate(data, |c| {
        if c == '>' {
            count += 1;
            true
        }
        else if count == 1 && c == '=' {
            seen_equal = true;
            false
        }
        else {
            false
        }
    })?;
    if seen_equal{
        tok = (TokenKind::Operators(Operators::GreaterThanEqual), 2);
    }
    else {
        tok = (TokenKind::Operators(Operators::GreaterThan), 1);
    }    
    Ok(tok)
}

pub fn tokenize_char(data: &str) -> Result<(TokenKind, usize),usize> {
    
    let mut count = 0;
    let (str,bytes_read) = execute_predicate(data, |c| {
        if c == '\'' && count == 0 {
            count += 1;
            true
        }
        else if count == 1 && c != '\'' {
            true
        }
        else {
            false
        }
    })?;
    
    Ok((TokenKind::CharConst(str[1..2].to_string()),bytes_read+1))
}

pub fn tokenize_formatted_string(data: &str) -> Result<(TokenKind, usize),usize> {
    let mut count: i32 = 0;
    let (lexema, bytes_read) = execute_predicate(data, |c| {
        //consome os caracteres enquanto for diferente de " (fecha aspas duplas)
        if c == '"' && count == 0 {
            count = 1;
            true
        } 
        else if c == '"' && count == 1 {
            count = 2;
            false 
        } 
        else {
            true
        }
    })?;
       
    Ok((TokenKind::FormattedString(lexema[1..].to_string()), bytes_read+1))
}

/// Try to lex a single token from the input stream.
pub fn tokenize_single_token(data: &str) -> Result<(TokenKind, usize),usize> {
    
    let next = match data.chars().next() {
        Some(c) => c,
        None => panic!("Error"),
    };
    
    let (tok, length) = match next {
        '.' => (TokenKind::Punctuation(Punctuation::Dot), 1),
        ':' => (TokenKind::Punctuation(Punctuation::Colon), 1),
        ';' => (TokenKind::Punctuation(Punctuation::Semicolon), 1),
        ',' => (TokenKind::Punctuation(Punctuation::Comma), 1),
        '{' => (TokenKind::Punctuation(Punctuation::LBrace), 1),
        '}' => (TokenKind::Punctuation(Punctuation::RBrace), 1),
        '(' => (TokenKind::Punctuation(Punctuation::LParen), 1),
        ')' => (TokenKind::Punctuation(Punctuation::RParen), 1),
        '+' => (TokenKind::Operators(Operators::Plus), 1),
        '-' => tokenize_minus_or_arrow(data).expect("Couldn't tokenize minus or arrow!"),
        '*' => (TokenKind::Operators(Operators::Multiplication), 1),
        '/' => (TokenKind::Operators(Operators::Division), 1),
        '<' => tokenize_less_or_lessthan(data).expect("Couldn't tokenize the less than operator"),
        '>' => tokenize_greater_or_greaterthan(data).expect("Couldn't tokenize the greater than operator"),
        '\'' => tokenize_char(data).expect("Couldn't tokenize a char const"),
        '0'..='9' => tokenize_number(data).expect("Couldn't tokenize a number"),
        '=' => tokenize_assing_or_equal(data).expect("Couldn't tokenize assign or equal!"),
        '"' => tokenize_formatted_string(data).expect("Couldn't tokenize a formatted string"),
        c @ '_' | c if c.is_alphabetic() => tokenize_ident_reservedword(data)
                                        .expect("Couldn't tokenize an identifier or reserved word"),
        other => (TokenKind::Error(format!("Unkown char {}", other)),1),
    };
    
    Ok((tok, length))
}

fn skip_whitespace(data: &str) -> usize {
    match execute_predicate(data, |ch| ch.is_whitespace()) {
        Ok((_, bytes_skipped)) => bytes_skipped,
        _ => 0,
    }
}

fn skip_comments(src: &str) -> usize {
    let pairs = [("//", "\n")];

    for &(pattern, matcher) in &pairs {
        if src.starts_with(pattern) {
            let leftovers = skip_until(src, matcher);
            return src.len() - leftovers.len();
        }
    }

    0
}

fn skip_until<'a>(mut src: &'a str, pattern: &str) -> &'a str {
    while !src.is_empty() && !src.starts_with(pattern) {
        let next_char_size = src.chars().next().expect("The string isn't empty").len_utf8();
        src = &src[next_char_size..];
    }

    &src[pattern.len()..]
}

/// Skip past any whitespace characters or comments.
fn skip(src: &str) -> usize {
    let mut remaining = src;

    loop {
        let ws: usize = skip_whitespace(remaining);
        remaining = &remaining[ws..];
        let comments = skip_comments(remaining);
        remaining = &remaining[comments..];

        if ws + comments == 0 {
            return src.len() - remaining.len();
        }
    }
}

struct Tokenizer<'a> {
    current_index: usize,
    remaining_text: &'a str,
    line_number: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(src: &str) -> Tokenizer {
        Tokenizer {
            current_index: 0,
            remaining_text: src,
            line_number: 0
        }
    }

    fn next_token(&mut self) -> Option<(TokenKind,usize,usize)> {
        self.skip_whitespace();
        
        if self.remaining_text.is_empty() {
            None
        } else {
            let start: usize = self.current_index;
            let tok: (TokenKind, usize) = self._next_token().expect("Couldn't read the next token");            
            let end: usize = self.current_index;
            Some((tok.0, start, end))
        }
    }

    fn skip_whitespace(&mut self) {
        let skipped = skip(self.remaining_text);
        self.chomp(skipped);
    }

    fn _next_token(&mut self) -> Result<(TokenKind,usize),usize> {
        let (tok, bytes_read) = tokenize_single_token(self.remaining_text)?;
        self.chomp(bytes_read);

        Ok((tok,bytes_read))
    }

    fn chomp(&mut self, num_bytes: usize) {
        self.remaining_text = &self.remaining_text[num_bytes..];
        self.current_index += num_bytes;
    }
}

/// Turn a string of valid code into a list of tokens, including the 
/// location of that token's start and end point in the original source code.
/// Note the token indices represent the half-open interval `[start, end)`, 
/// equivalent to `start .. end` in Rust.
pub fn tokenize(src: &str, line_number: usize) -> Vec<Token> {
    let mut tokenizer: Tokenizer<'_> = Tokenizer::new(src);
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(tok) = tokenizer.next_token() {
        let token = Token::new( tok.1, tok.2,  line_number, tok.0);
        tokens.push(token);
    }

    tokens
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = match File::open(&path) {
        Err(why) => panic!("Could not open file. Error cause: {}", why),
        Ok(file) => file
    };
    Ok(io::BufReader::new(file).lines())
} 

pub fn tokenize_file<P: AsRef<Path>>(path: P) -> Vec<Token> {    
    let mut tokens: Vec<Token> = vec![];    
    if let Ok(lines) = read_lines(path) {

        let mut line_number: usize = 1;
        
        for line in lines {
            if let Ok(v) = line {                
                tokens.extend_from_slice(&tokenize(&v, line_number));                
            }
            line_number += 1;
        }
    }
    tokens
} 

pub fn tokenize_string(str: String) -> Vec<Token> {    
    let mut tokens: Vec<Token> = vec![];    
    let mut line_number: usize = 1;
    
    for line in str.lines() {
        tokens.extend_from_slice(&tokenize(line, line_number));
        line_number += 1;
    }
    tokens
} 

pub fn save_tokens<P: AsRef<Path>>(tokens: &Vec<Token>, path: P) -> anyhow::Result<(),Error> {
    let file: File = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let writer: BufWriter<File> = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tokens)?;
    Ok(())
}