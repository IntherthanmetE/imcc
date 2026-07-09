use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),
    Number(f64),
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Println,
    EOF,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position >= self.input.len() { None } else { Some(self.input[self.position]) }
    }

    fn advance(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            let c = self.input[self.position];
            self.position += 1;
            Some(c)
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\r' | '\n' => { self.advance(); }
                '+' => { tokens.push(Token::Plus); self.advance(); }
                '-' => { tokens.push(Token::Minus); self.advance(); }
                '*' => { tokens.push(Token::Star); self.advance(); }
                '/' => { tokens.push(Token::Slash); self.advance(); }
                '=' => { tokens.push(Token::Assign); self.advance(); }
                _ if c.is_digit(10) || c == '.' => {
                    let mut num_str = String::new();
                    while let Some(nc) = self.peek() {
                        if nc.is_digit(10) || nc == '.' {
                            num_str.push(nc);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let num = num_str.parse::<f64>().map_err(|_| "Invalid number".to_string())?;
                    tokens.push(Token::Number(num));
                }
                _ if c.is_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    while let Some(ic) = self.peek() {
                        if ic.is_alphanumeric() || ic == '_' {
                            ident.push(ic);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if ident == "println" {
                        tokens.push(Token::Println);
                    } else {
                        tokens.push(Token::Ident(ident));
                    }
                }
                _ => return Err(format!("Unexpected character: '{}'", c)),
            }
        }
        tokens.push(Token::EOF);
        Ok(tokens)
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
}

#[derive(Debug, Clone)]
enum Stmt {
    Assign(String, Expr),
    Println(Expr),
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) -> &Token {
        let t = self.peek();
        if *t != Token::EOF {
            self.position += 1;
        }
        &self.tokens[self.position - 1]
    }

    fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while *self.peek() != Token::EOF {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Println => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Stmt::Println(expr))
            }
            Token::Ident(name) => {
                let var_name = name.clone();
                self.advance();
                if *self.peek() == Token::Assign {
                    self.advance();
                    let expr = self.parse_expr()?;
                    Ok(Stmt::Assign(var_name, expr))
                } else {
                    Err(format!("Expected '=' after variable '{}'", var_name))
                }
            }
            _ => Err(format!("Syntax Error: Unexpected token {:?}", self.peek())),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;
        while *self.peek() == Token::Plus || *self.peek() == Token::Minus {
            let op = if *self.advance() == Token::Plus { "+".to_string() } else { "-".to_string() };
            let right = self.parse_factor()?;
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;
        while *self.peek() == Token::Star || *self.peek() == Token::Slash {
            let op = if *self.advance() == Token::Star { "*".to_string() } else { "/".to_string() };
            let right = self.parse_primary()?;
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Number(*n)),
            Token::Ident(s) => Ok(Expr::Variable(s.clone())),
            t => Err(format!("Expected number or variable, found {:?}", t)),
        }
    }
}

struct Runtime {
    variables: HashMap<String, f64>,
}

impl Runtime {
    fn new() -> Self {
        Self { variables: HashMap::new() }
    }

    fn run(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for stmt in statements {
            match stmt {
                Stmt::Assign(name, expr) => {
                    let val = self.eval(expr)?;
                    self.variables.insert(name, val);
                }
                Stmt::Println(expr) => {
                    let val = self.eval(expr)?;
                    println!("{}", val);
                }
            }
        }
        Ok(())
    }

    fn eval(&self, expr: Expr) -> Result<f64, String> {
        match expr {
            Expr::Number(n) => Ok(n),
            Expr::Variable(v) => self.variables.get(&v).cloned().ok_or(format!("Undefined variable: {}", v)),
            Expr::BinaryOp(left, op, right) => {
                let l = self.eval(*left)?;
                let r = self.eval(*right)?;
                match op.as_str() {
                    "+" => Ok(l + r),
                    "-" => Ok(l - r),
                    "*" => Ok(l * r),
                    "/" => {
                        if r == 0.0 { Err("Division by zero".to_string()) } else { Ok(l / r) }
                    }
                    _ => Err("Invalid operator".to_string()),
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No input file specified.");
        eprintln!("Usage: imcc <filename>");
        process::exit(1);
    }

    let file_path = &args[1];
    if !Path::new(file_path).exists() {
        eprintln!("Error: File not found: {}", file_path);
        process::exit(1);
    }

    let source_code = match fs::read_to_string(file_path) {
        Ok(code) => code,
        Err(_) => {
            eprintln!("Error: Could not read file: {}", file_path);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source_code);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer Error: {}", e);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse_program() {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("Parser Error: {}", e);
            process::exit(1);
        }
    };

    let mut runtime = Runtime::new();
    if let Err(e) = runtime.run(ast) {
        eprintln!("Runtime Error: {}", e);
        process::exit(1);
    }
}

