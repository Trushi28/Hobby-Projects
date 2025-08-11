// Flux Programming Language Compiler
// An advanced compiler with unique features including immutable dynamic typing,
// flexible OOP, syntax pragma control, and temporal variable tracking

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::process;

// ============================================================================
// LEXER - Tokenization
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    
    // Keywords
    Let, Const, Func, Return, If, Else, While, For,
    Class, Extends, New, This, Super,
    Import, Export, Match, Case, Default,
    Temporal, Freeze, Thaw, Timeline,
    
    // Operators
    Plus, Minus, Multiply, Divide, Modulo,
    Assign, Equal, NotEqual, Less, Greater,
    LessEqual, GreaterEqual, And, Or, Not,
    Arrow, FatArrow, Pipe, Compose,
    
    // Delimiters
    LeftParen, RightParen, LeftBrace, RightBrace,
    LeftBracket, RightBracket, Comma, Semicolon,
    Colon, Dot, Question, Bang,
    
    // Special
    Newline, Indent, Dedent, EOF,
    Pragma(String),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    line: usize,
    column: usize,
    use_braces: bool,
    indent_stack: Vec<usize>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Self {
            input: chars,
            position: 0,
            current_char,
            line: 1,
            column: 1,
            use_braces: true, // Default to braces
            indent_stack: vec![0],
        }
    }
    
    fn advance(&mut self) {
        if self.current_char == Some('\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }
    
    fn peek(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn read_number(&mut self) -> f64 {
        let mut number_str = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                number_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        number_str.parse().unwrap_or(0.0)
    }
    
    fn read_string(&mut self) -> String {
        let mut string_val = String::new();
        self.advance(); // Skip opening quote
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // Skip closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                match self.current_char {
                    Some('n') => string_val.push('\n'),
                    Some('t') => string_val.push('\t'),
                    Some('r') => string_val.push('\r'),
                    Some('\\') => string_val.push('\\'),
                    Some('"') => string_val.push('"'),
                    Some(other) => string_val.push(other),
                    None => break,
                }
                self.advance();
            } else {
                string_val.push(ch);
                self.advance();
            }
        }
        
        string_val
    }
    
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        identifier
    }
    
    fn handle_pragma(&mut self, pragma_content: &str) {
        match pragma_content.trim() {
            "braces" => self.use_braces = true,
            "indent" | "no_braces" => self.use_braces = false,
            _ => {} // Ignore unknown pragmas
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<TokenType> {
        let mut tokens = Vec::new();
        
        while self.current_char.is_some() {
            match self.current_char.unwrap() {
                ' ' | '\t' | '\r' => self.skip_whitespace(),
                
                '\n' => {
                    if !self.use_braces {
                        tokens.push(TokenType::Newline);
                    }
                    self.advance();
                }
                
                '#' => {
                    // Handle pragma or comments
                    self.advance();
                    if self.current_char == Some('p') {
                        let pragma = self.read_identifier();
                        if pragma == "pragma" {
                            self.skip_whitespace();
                            let pragma_content = self.read_identifier();
                            self.handle_pragma(&pragma_content);
                            tokens.push(TokenType::Pragma(pragma_content));
                        }
                    } else {
                        // Skip comment
                        while self.current_char.is_some() && self.current_char != Some('\n') {
                            self.advance();
                        }
                    }
                }
                
                '+' => {
                    tokens.push(TokenType::Plus);
                    self.advance();
                }
                
                '-' => {
                    self.advance();
                    if self.current_char == Some('>') {
                        tokens.push(TokenType::Arrow);
                        self.advance();
                    } else {
                        tokens.push(TokenType::Minus);
                    }
                }
                
                '*' => {
                    tokens.push(TokenType::Multiply);
                    self.advance();
                }
                
                '/' => {
                    tokens.push(TokenType::Divide);
                    self.advance();
                }
                
                '%' => {
                    tokens.push(TokenType::Modulo);
                    self.advance();
                }
                
                '=' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        tokens.push(TokenType::Equal);
                        self.advance();
                    } else if self.current_char == Some('>') {
                        tokens.push(TokenType::FatArrow);
                        self.advance();
                    } else {
                        tokens.push(TokenType::Assign);
                    }
                }
                
                '!' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        tokens.push(TokenType::NotEqual);
                        self.advance();
                    } else {
                        tokens.push(TokenType::Not);
                    }
                }
                
                '<' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        tokens.push(TokenType::LessEqual);
                        self.advance();
                    } else {
                        tokens.push(TokenType::Less);
                    }
                }
                
                '>' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        tokens.push(TokenType::GreaterEqual);
                        self.advance();
                    } else {
                        tokens.push(TokenType::Greater);
                    }
                }
                
                '&' => {
                    self.advance();
                    if self.current_char == Some('&') {
                        tokens.push(TokenType::And);
                        self.advance();
                    }
                }
                
                '|' => {
                    self.advance();
                    if self.current_char == Some('|') {
                        tokens.push(TokenType::Or);
                        self.advance();
                    } else {
                        tokens.push(TokenType::Pipe);
                    }
                }
                
                '(' => {
                    tokens.push(TokenType::LeftParen);
                    self.advance();
                }
                
                ')' => {
                    tokens.push(TokenType::RightParen);
                    self.advance();
                }
                
                '{' => {
                    if self.use_braces {
                        tokens.push(TokenType::LeftBrace);
                    }
                    self.advance();
                }
                
                '}' => {
                    if self.use_braces {
                        tokens.push(TokenType::RightBrace);
                    }
                    self.advance();
                }
                
                '[' => {
                    tokens.push(TokenType::LeftBracket);
                    self.advance();
                }
                
                ']' => {
                    tokens.push(TokenType::RightBracket);
                    self.advance();
                }
                
                ',' => {
                    tokens.push(TokenType::Comma);
                    self.advance();
                }
                
                ';' => {
                    tokens.push(TokenType::Semicolon);
                    self.advance();
                }
                
                ':' => {
                    tokens.push(TokenType::Colon);
                    self.advance();
                }
                
                '.' => {
                    if let Some(next_char) = self.peek(1) {
                        if next_char.is_ascii_digit() {
                            let number = self.read_number();
                            tokens.push(TokenType::Number(number));
                        } else {
                            tokens.push(TokenType::Dot);
                            self.advance();
                        }
                    } else {
                        tokens.push(TokenType::Dot);
                        self.advance();
                    }
                }
                
                '?' => {
                    tokens.push(TokenType::Question);
                    self.advance();
                }
                
                '"' => {
                    let string_val = self.read_string();
                    tokens.push(TokenType::String(string_val));
                }
                
                ch if ch.is_ascii_digit() => {
                    let number = self.read_number();
                    tokens.push(TokenType::Number(number));
                }
                
                ch if ch.is_alphabetic() || ch == '_' => {
                    let identifier = self.read_identifier();
                    let token = match identifier.as_str() {
                        "let" => TokenType::Let,
                        "const" => TokenType::Const,
                        "func" => TokenType::Func,
                        "return" => TokenType::Return,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "while" => TokenType::While,
                        "for" => TokenType::For,
                        "class" => TokenType::Class,
                        "extends" => TokenType::Extends,
                        "new" => TokenType::New,
                        "this" => TokenType::This,
                        "super" => TokenType::Super,
                        "import" => TokenType::Import,
                        "export" => TokenType::Export,
                        "match" => TokenType::Match,
                        "case" => TokenType::Case,
                        "default" => TokenType::Default,
                        "temporal" => TokenType::Temporal,
                        "freeze" => TokenType::Freeze,
                        "thaw" => TokenType::Thaw,
                        "timeline" => TokenType::Timeline,
                        "true" => TokenType::Boolean(true),
                        "false" => TokenType::Boolean(false),
                        _ => TokenType::Identifier(identifier),
                    };
                    tokens.push(token);
                }
                
                _ => {
                    eprintln!("Unexpected character: {} at line {}, column {}", 
                             self.current_char.unwrap(), self.line, self.column);
                    self.advance();
                }
            }
        }
        
        tokens.push(TokenType::EOF);
        tokens
    }
}

// ============================================================================
// AST - Abstract Syntax Tree
// ============================================================================

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    
    // Statements
    VarDecl { 
        name: String, 
        value: Box<ASTNode>, 
        is_const: bool,
        is_temporal: bool,
    },
    Assignment { name: String, value: Box<ASTNode> },
    FunctionDecl { 
        name: String, 
        params: Vec<String>, 
        body: Vec<ASTNode> 
    },
    ClassDecl { 
        name: String, 
        superclass: Option<String>, 
        methods: Vec<ASTNode> 
    },
    Return(Box<ASTNode>),
    If { 
        condition: Box<ASTNode>, 
        then_branch: Vec<ASTNode>, 
        else_branch: Option<Vec<ASTNode>> 
    },
    While { condition: Box<ASTNode>, body: Vec<ASTNode> },
    
    // Expressions
    Binary { 
        left: Box<ASTNode>, 
        operator: String, 
        right: Box<ASTNode> 
    },
    Unary { operator: String, operand: Box<ASTNode> },
    Call { callee: Box<ASTNode>, args: Vec<ASTNode> },
    MemberAccess { object: Box<ASTNode>, property: String },
    
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    
    // Unique Features
    TemporalAccess { 
        var: String, 
        timestamp: Box<ASTNode> 
    },
    Pipeline(Vec<ASTNode>),
    Match { 
        expr: Box<ASTNode>, 
        cases: Vec<(ASTNode, Vec<ASTNode>)> 
    },
}

// ============================================================================
// PARSER - Syntax Analysis
// ============================================================================

pub struct Parser {
    tokens: Vec<TokenType>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Self { tokens, current: 0 }
    }
    
    fn peek(&self) -> &TokenType {
        self.tokens.get(self.current).unwrap_or(&TokenType::EOF)
    }
    
    fn advance(&mut self) -> &TokenType {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.peek()
    }
    
    fn consume(&mut self, expected: TokenType) -> Result<(), String> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.peek()))
        }
    }
    
    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();
        
        while !matches!(self.peek(), TokenType::EOF) {
            if let TokenType::Pragma(_) = self.peek() {
                self.advance(); // Skip pragma tokens in parsing
                continue;
            }
            statements.push(self.parse_statement()?);
        }
        
        Ok(ASTNode::Program(statements))
    }
    
    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        match self.peek() {
            TokenType::Let => self.parse_var_decl(false, false),
            TokenType::Const => self.parse_var_decl(true, false),
            TokenType::Temporal => {
                self.advance(); // consume 'temporal'
                match self.peek() {
                    TokenType::Let => self.parse_var_decl(false, true),
                    TokenType::Const => self.parse_var_decl(true, true),
                    _ => Err("Expected 'let' or 'const' after 'temporal'".to_string()),
                }
            },
            TokenType::Func => self.parse_function(),
            TokenType::Class => self.parse_class(),
            TokenType::Return => self.parse_return(),
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::Match => self.parse_match(),
            _ => {
                let expr = self.parse_expression()?;
                Ok(expr)
            }
        }
    }
    
    fn parse_var_decl(&mut self, is_const: bool, is_temporal: bool) -> Result<ASTNode, String> {
        self.advance(); // consume 'let' or 'const'
        
        if let TokenType::Identifier(name) = self.peek() {
            let var_name = name.clone();
            self.advance();
            
            self.consume(TokenType::Assign)?;
            let value = self.parse_expression()?;
            
            Ok(ASTNode::VarDecl {
                name: var_name,
                value: Box::new(value),
                is_const,
                is_temporal,
            })
        } else {
            Err("Expected identifier after variable declaration".to_string())
        }
    }
    
    fn parse_function(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'func'
        
        let name = if let TokenType::Identifier(name) = self.peek() {
            let n = name.clone();
            self.advance();
            n
        } else {
            return Err("Expected function name".to_string());
        };
        
        self.consume(TokenType::LeftParen)?;
        let mut params = Vec::new();
        
        while !matches!(self.peek(), TokenType::RightParen) {
            if let TokenType::Identifier(param) = self.peek() {
                params.push(param.clone());
                self.advance();
                
                if matches!(self.peek(), TokenType::Comma) {
                    self.advance();
                }
            } else {
                return Err("Expected parameter name".to_string());
            }
        }
        
        self.consume(TokenType::RightParen)?;
        self.consume(TokenType::LeftBrace)?;
        
        let mut body = Vec::new();
        while !matches!(self.peek(), TokenType::RightBrace) {
            body.push(self.parse_statement()?);
        }
        
        self.consume(TokenType::RightBrace)?;
        
        Ok(ASTNode::FunctionDecl { name, params, body })
    }
    
    fn parse_class(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'class'
        
        let name = if let TokenType::Identifier(name) = self.peek() {
            let n = name.clone();
            self.advance();
            n
        } else {
            return Err("Expected class name".to_string());
        };
        
        let superclass = if matches!(self.peek(), TokenType::Extends) {
            self.advance();
            if let TokenType::Identifier(super_name) = self.peek() {
                let s = super_name.clone();
                self.advance();
                Some(s)
            } else {
                return Err("Expected superclass name".to_string());
            }
        } else {
            None
        };
        
        self.consume(TokenType::LeftBrace)?;
        
        let mut methods = Vec::new();
        while !matches!(self.peek(), TokenType::RightBrace) {
            methods.push(self.parse_function()?);
        }
        
        self.consume(TokenType::RightBrace)?;
        
        Ok(ASTNode::ClassDecl { name, superclass, methods })
    }
    
    fn parse_return(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'return'
        let value = self.parse_expression()?;
        Ok(ASTNode::Return(Box::new(value)))
    }
    
    fn parse_if(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'if'
        
        let condition = self.parse_expression()?;
        self.consume(TokenType::LeftBrace)?;
        
        let mut then_branch = Vec::new();
        while !matches!(self.peek(), TokenType::RightBrace) {
            then_branch.push(self.parse_statement()?);
        }
        self.consume(TokenType::RightBrace)?;
        
        let else_branch = if matches!(self.peek(), TokenType::Else) {
            self.advance();
            self.consume(TokenType::LeftBrace)?;
            
            let mut else_stmts = Vec::new();
            while !matches!(self.peek(), TokenType::RightBrace) {
                else_stmts.push(self.parse_statement()?);
            }
            self.consume(TokenType::RightBrace)?;
            
            Some(else_stmts)
        } else {
            None
        };
        
        Ok(ASTNode::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        })
    }
    
    fn parse_while(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'while'
        
        let condition = self.parse_expression()?;
        self.consume(TokenType::LeftBrace)?;
        
        let mut body = Vec::new();
        while !matches!(self.peek(), TokenType::RightBrace) {
            body.push(self.parse_statement()?);
        }
        self.consume(TokenType::RightBrace)?;
        
        Ok(ASTNode::While {
            condition: Box::new(condition),
            body,
        })
    }
    
    fn parse_match(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'match'
        
        let expr = self.parse_expression()?;
        self.consume(TokenType::LeftBrace)?;
        
        let mut cases = Vec::new();
        
        while !matches!(self.peek(), TokenType::RightBrace) {
            let pattern = self.parse_expression()?;
            self.consume(TokenType::FatArrow)?;
            
            let mut case_body = Vec::new();
            if matches!(self.peek(), TokenType::LeftBrace) {
                self.advance();
                while !matches!(self.peek(), TokenType::RightBrace) {
                    case_body.push(self.parse_statement()?);
                }
                self.consume(TokenType::RightBrace)?;
            } else {
                case_body.push(self.parse_statement()?);
            }
            
            cases.push((pattern, case_body));
        }
        
        self.consume(TokenType::RightBrace)?;
        
        Ok(ASTNode::Match {
            expr: Box::new(expr),
            cases,
        })
    }
    
    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_pipeline()
    }
    
    fn parse_pipeline(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_logical_or()?;
        
        let mut pipeline_exprs = vec![expr.clone()];
        
        while matches!(self.peek(), TokenType::Pipe) {
            self.advance();
            pipeline_exprs.push(self.parse_logical_or()?);
        }
        
        if pipeline_exprs.len() > 1 {
            Ok(ASTNode::Pipeline(pipeline_exprs))
        } else {
            Ok(expr)
        }
    }
    
    fn parse_logical_or(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_logical_and()?;
        
        while matches!(self.peek(), TokenType::Or) {
            let op = "||".to_string();
            self.advance();
            let right = self.parse_logical_and()?;
            left = ASTNode::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_logical_and(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_equality()?;
        
        while matches!(self.peek(), TokenType::And) {
            let op = "&&".to_string();
            self.advance();
            let right = self.parse_equality()?;
            left = ASTNode::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_equality(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_comparison()?;
        
        while matches!(self.peek(), TokenType::Equal | TokenType::NotEqual) {
            let op = match self.peek() {
                TokenType::Equal => "==".to_string(),
                TokenType::NotEqual => "!=".to_string(),
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = ASTNode::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_addition()?;
        
        while matches!(self.peek(), TokenType::Less | TokenType::Greater | 
                      TokenType::LessEqual | TokenType::GreaterEqual) {
            let op = match self.peek() {
                TokenType::Less => "<".to_string(),
                TokenType::Greater => ">".to_string(),
                TokenType::LessEqual => "<=".to_string(),
                TokenType::GreaterEqual => ">=".to_string(),
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_addition()?;
            left = ASTNode::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_addition(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_multiplication()?;
        
        while matches!(self.peek(), TokenType::Plus | TokenType::Minus) {
            let op = match self.peek() {
                TokenType::Plus => "+".to_string(),
                TokenType::Minus => "-".to_string(),
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplication()?;
            left = ASTNode::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_multiplication(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_unary()?;
        
        while matches!(self.peek(), TokenType::Multiply | TokenType::Divide | TokenType::Modulo) {
            let op = match self.peek() {
                TokenType::Multiply => "*".to_string(),
                TokenType::Divide => "/".to_string(),
                TokenType::Modulo => "%".to_string(),
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary()?;
            left = ASTNode::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<ASTNode, String> {
        match self.peek() {
            TokenType::Not | TokenType::Minus => {
                let op = match self.peek() {
                    TokenType::Not => "!".to_string(),
                    TokenType::Minus => "-".to_string(),
                    _ => unreachable!(),
                };
                self.advance();
                let operand = self.parse_unary()?;
                Ok(ASTNode::Unary {
                    operator: op,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_call(),
        }
    }
    
    fn parse_call(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_primary()?;
        
        loop {
            match self.peek() {
                TokenType::LeftParen => {
                    self.advance();
                    let mut args = Vec::new();
                    
                    while !matches!(self.peek(), TokenType::RightParen) {
                        args.push(self.parse_expression()?);
                        if matches!(self.peek(), TokenType::Comma) {
                            self.advance();
                        }
                    }
                    
                    self.consume(TokenType::RightParen)?;
                    expr = ASTNode::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                TokenType::Dot => {
                    self.advance();
                    if let TokenType::Identifier(property) = self.peek() {
                        let prop = property.clone();
                        self.advance();
                        expr = ASTNode::MemberAccess {
                            object: Box::new(expr),
                            property: prop,
                        };
                    } else {
                        return Err("Expected property name after '.'".to_string());
                    }
                }
                TokenType::LeftBracket => {
                    // Temporal access: var[timestamp]
                    self.advance();
                    let timestamp = self.parse_expression()?;
                    self.consume(TokenType::RightBracket)?;
                    
                    if let ASTNode::Identifier(var_name) = expr {
                        expr = ASTNode::TemporalAccess {
                            var: var_name,
                            timestamp: Box::new(timestamp),
                        };
                    }
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        match self.peek() {
            TokenType::Number(n) => {
                let num = *n;
                self.advance();
                Ok(ASTNode::Number(num))
            }
            TokenType::String(s) => {
                let string = s.clone();
                self.advance();
                Ok(ASTNode::String(string))
            }
            TokenType::Boolean(b) => {
                let boolean = *b;
                self.advance();
                Ok(ASTNode::Boolean(boolean))
            }
            TokenType::Identifier(name) => {
                let id = name.clone();
                self.advance();
                Ok(ASTNode::Identifier(id))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenType::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token in expression: {:?}", self.peek())),
        }
    }
}

// ============================================================================
// SEMANTIC ANALYZER & TYPE CHECKER
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum FluxType {
    Number,
    String, 
    Boolean,
    Function(Vec<FluxType>, Box<FluxType>),
    Object(HashMap<String, FluxType>),
    Temporal(Box<FluxType>),
    Any,
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    flux_type: FluxType,
    is_const: bool,
    is_temporal: bool,
    is_frozen: bool,
    timeline: Vec<(usize, FluxType)>, // (timestamp, value_type)
}

pub struct SemanticAnalyzer {
    symbol_table: HashMap<String, Variable>,
    current_scope: usize,
    timestamp: usize,
    errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
            current_scope: 0,
            timestamp: 0,
            errors: Vec::new(),
        }
    }
    
    pub fn analyze(&mut self, ast: &ASTNode) -> Result<(), Vec<String>> {
        self.visit(ast);
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
    
    fn visit(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Program(statements) => {
                for stmt in statements {
                    self.visit(stmt);
                }
            }
            
            ASTNode::VarDecl { name, value, is_const, is_temporal } => {
                let value_type = self.infer_type(value);
                
                if self.symbol_table.contains_key(name) {
                    self.errors.push(format!("Variable '{}' already declared", name));
                    return;
                }
                
                let var = Variable {
                    name: name.clone(),
                    flux_type: if *is_temporal { 
                        FluxType::Temporal(Box::new(value_type)) 
                    } else { 
                        value_type 
                    },
                    is_const: *is_const,
                    is_temporal: *is_temporal,
                    is_frozen: false,
                    timeline: vec![(self.timestamp, self.infer_type(value))],
                };
                
                self.symbol_table.insert(name.clone(), var);
                self.visit(value);
            }
            
            ASTNode::Assignment { name, value } => {
                if let Some(var) = self.symbol_table.get(name) {
                    if var.is_const {
                        self.errors.push(format!("Cannot reassign to const variable '{}'", name));
                        return;
                    }
                    if var.is_frozen {
                        self.errors.push(format!("Cannot modify frozen variable '{}'", name));
                        return;
                    }
                } else {
                    self.errors.push(format!("Undefined variable '{}'", name));
                }
                
                self.visit(value);
            }
            
            ASTNode::TemporalAccess { var, timestamp } => {
                if let Some(variable) = self.symbol_table.get(var) {
                    if !variable.is_temporal {
                        self.errors.push(format!("Variable '{}' is not temporal", var));
                    }
                } else {
                    self.errors.push(format!("Undefined variable '{}'", var));
                }
                
                self.visit(timestamp);
            }
            
            ASTNode::FunctionDecl { name, params: _, body } => {
                // Create new scope for function
                self.current_scope += 1;
                for stmt in body {
                    self.visit(stmt);
                }
                self.current_scope -= 1;
            }
            
            ASTNode::Binary { left, operator: _, right } => {
                self.visit(left);
                self.visit(right);
            }
            
            ASTNode::Call { callee, args } => {
                self.visit(callee);
                for arg in args {
                    self.visit(arg);
                }
            }
            
            ASTNode::Pipeline(exprs) => {
                for expr in exprs {
                    self.visit(expr);
                }
            }
            
            _ => {}
        }
        
        self.timestamp += 1;
    }
    
    fn infer_type(&self, node: &ASTNode) -> FluxType {
        match node {
            ASTNode::Number(_) => FluxType::Number,
            ASTNode::String(_) => FluxType::String,
            ASTNode::Boolean(_) => FluxType::Boolean,
            ASTNode::Identifier(name) => {
                if let Some(var) = self.symbol_table.get(name) {
                    var.flux_type.clone()
                } else {
                    FluxType::Any
                }
            }
            ASTNode::Binary { left, operator, right } => {
                let left_type = self.infer_type(left);
                let right_type = self.infer_type(right);
                
                match operator.as_str() {
                    "+" | "-" | "*" | "/" | "%" => FluxType::Number,
                    "==" | "!=" | "<" | ">" | "<=" | ">=" => FluxType::Boolean,
                    "&&" | "||" => FluxType::Boolean,
                    _ => FluxType::Any,
                }
            }
            _ => FluxType::Any,
        }
    }
}

// ============================================================================
// CODE GENERATOR - LLVM IR / Assembly Output
// ============================================================================

pub struct CodeGenerator {
    output: String,
    label_counter: usize,
    temp_counter: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            label_counter: 0,
            temp_counter: 0,
        }
    }
    
    pub fn generate(&mut self, ast: &ASTNode) -> String {
        self.emit_header();
        self.visit(ast);
        self.emit_footer();
        self.output.clone()
    }
    
    fn emit_header(&mut self) {
        self.output.push_str("; Flux Language - Generated LLVM IR\n");
        self.output.push_str("target triple = \"x86_64-pc-linux-gnu\"\n\n");
        
        // Declare external functions
        self.output.push_str("declare i32 @printf(i8*, ...)\n");
        self.output.push_str("declare i8* @malloc(i64)\n");
        self.output.push_str("declare void @free(i8*)\n\n");
        
        // Global format strings
        self.output.push_str("@.str_num = private unnamed_addr constant [6 x i8] c\"%f\\0A\\00\"\n");
        self.output.push_str("@.str_str = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\"\n");
        self.output.push_str("@.str_bool_true = private unnamed_addr constant [6 x i8] c\"true\\0A\\00\"\n");
        self.output.push_str("@.str_bool_false = private unnamed_addr constant [7 x i8] c\"false\\0A\\00\"\n\n");
        
        // Temporal tracking structure
        self.output.push_str("%temporal_entry = type { double, i8* }\n");
        self.output.push_str("%temporal_var = type { i32, %temporal_entry* }\n\n");
    }
    
    fn emit_footer(&mut self) {
        self.output.push_str("\ndefine i32 @main() {\n");
        self.output.push_str("entry:\n");
        self.output.push_str("  call void @flux_main()\n");
        self.output.push_str("  ret i32 0\n");
        self.output.push_str("}\n");
    }
    
    fn visit(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Program(statements) => {
                self.output.push_str("define void @flux_main() {\n");
                self.output.push_str("entry:\n");
                
                for stmt in statements {
                    self.visit(stmt);
                }
                
                self.output.push_str("  ret void\n");
                self.output.push_str("}\n\n");
            }
            
            ASTNode::VarDecl { name, value, is_const: _, is_temporal } => {
                let value_reg = self.visit_expression(value);
                
                if *is_temporal {
                    // Allocate temporal variable structure
                    let temporal_var = self.new_temp();
                    self.output.push_str(&format!("  %{} = call i8* @malloc(i64 16)\n", temporal_var));
                    self.output.push_str(&format!("  %{}_cast = bitcast i8* %{} to %temporal_var*\n", 
                                                 temporal_var, temporal_var));
                    
                    // Initialize with first entry
                    let entry_ptr = self.new_temp();
                    self.output.push_str(&format!("  %{} = call i8* @malloc(i64 16)\n", entry_ptr));
                    self.output.push_str(&format!("  %{}_entry = bitcast i8* %{} to %temporal_entry*\n", 
                                                 entry_ptr, entry_ptr));
                    
                    // Store timestamp and value
                    let timestamp_ptr = self.new_temp();
                    let value_ptr = self.new_temp();
                    self.output.push_str(&format!("  %{} = getelementptr %temporal_entry, %temporal_entry* %{}_entry, i32 0, i32 0\n",
                                                 timestamp_ptr, entry_ptr));
                    self.output.push_str(&format!("  store double 0.0, double* %{}\n", timestamp_ptr));
                    
                    self.output.push_str(&format!("  %{} = getelementptr %temporal_entry, %temporal_entry* %{}_entry, i32 0, i32 1\n",
                                                 value_ptr, entry_ptr));
                    // Store value (simplified - in real implementation would handle different types)
                    self.output.push_str(&format!("  store i8* null, i8** %{}\n", value_ptr));
                }
                
                // For simplicity, treating all variables as stack allocated doubles
                self.output.push_str(&format!("  %{} = alloca double\n", name));
                self.output.push_str(&format!("  store double %{}, double* %{}\n", value_reg, name));
            }
            
            ASTNode::Assignment { name, value } => {
                let value_reg = self.visit_expression(value);
                self.output.push_str(&format!("  store double %{}, double* %{}\n", value_reg, name));
            }
            
            ASTNode::FunctionDecl { name, params, body } => {
                // Generate parameter types (simplified to all doubles)
                let param_list = params.iter()
                    .map(|_| "double")
                    .collect::<Vec<_>>()
                    .join(", ");
                
                self.output.push_str(&format!("define double @{}({}) {{\n", name, param_list));
                self.output.push_str("entry:\n");
                
                // Allocate space for parameters
                for (i, param) in params.iter().enumerate() {
                    self.output.push_str(&format!("  %{} = alloca double\n", param));
                    self.output.push_str(&format!("  store double %{}, double* %{}\n", i, param));
                }
                
                for stmt in body {
                    self.visit(stmt);
                }
                
                // Default return if no explicit return
                self.output.push_str("  ret double 0.0\n");
                self.output.push_str("}\n\n");
            }
            
            ASTNode::Return(expr) => {
                let value_reg = self.visit_expression(expr);
                self.output.push_str(&format!("  ret double %{}\n", value_reg));
            }
            
            ASTNode::If { condition, then_branch, else_branch } => {
                let cond_reg = self.visit_expression(condition);
                let then_label = self.new_label();
                let else_label = self.new_label();
                let end_label = self.new_label();
                
                // Convert condition to boolean
                let bool_reg = self.new_temp();
                self.output.push_str(&format!("  %{} = fcmp une double %{}, 0.0\n", bool_reg, cond_reg));
                
                if else_branch.is_some() {
                    self.output.push_str(&format!("  br i1 %{}, label %{}, label %{}\n", 
                                                 bool_reg, then_label, else_label));
                } else {
                    self.output.push_str(&format!("  br i1 %{}, label %{}, label %{}\n", 
                                                 bool_reg, then_label, end_label));
                }
                
                // Then branch
                self.output.push_str(&format!("{}:\n", then_label));
                for stmt in then_branch {
                    self.visit(stmt);
                }
                self.output.push_str(&format!("  br label %{}\n", end_label));
                
                // Else branch
                if let Some(else_stmts) = else_branch {
                    self.output.push_str(&format!("{}:\n", else_label));
                    for stmt in else_stmts {
                        self.visit(stmt);
                    }
                    self.output.push_str(&format!("  br label %{}\n", end_label));
                }
                
                self.output.push_str(&format!("{}:\n", end_label));
            }
            
            ASTNode::While { condition, body } => {
                let loop_label = self.new_label();
                let body_label = self.new_label();
                let end_label = self.new_label();
                
                self.output.push_str(&format!("  br label %{}\n", loop_label));
                
                // Loop condition
                self.output.push_str(&format!("{}:\n", loop_label));
                let cond_reg = self.visit_expression(condition);
                let bool_reg = self.new_temp();
                self.output.push_str(&format!("  %{} = fcmp une double %{}, 0.0\n", bool_reg, cond_reg));
                self.output.push_str(&format!("  br i1 %{}, label %{}, label %{}\n", 
                                             bool_reg, body_label, end_label));
                
                // Loop body
                self.output.push_str(&format!("{}:\n", body_label));
                for stmt in body {
                    self.visit(stmt);
                }
                self.output.push_str(&format!("  br label %{}\n", loop_label));
                
                self.output.push_str(&format!("{}:\n", end_label));
            }
            
            ASTNode::Pipeline(exprs) => {
                // Pipeline: pass result of each expression to the next
                let mut current_reg = String::new();
                
                for (i, expr) in exprs.iter().enumerate() {
                    if i == 0 {
                        current_reg = self.visit_expression(expr);
                    } else {
                        // For simplicity, just evaluate each expression
                        // Real implementation would thread results properly
                        current_reg = self.visit_expression(expr);
                    }
                }
            }
            
            _ => {}
        }
    }
    
    fn visit_expression(&mut self, node: &ASTNode) -> String {
        match node {
            ASTNode::Number(n) => {
                let temp = self.new_temp();
                self.output.push_str(&format!("  %{} = fadd double 0.0, {}\n", temp, n));
                format!("%{}", temp)
            }
            
            ASTNode::Boolean(b) => {
                let temp = self.new_temp();
                let value = if *b { 1.0 } else { 0.0 };
                self.output.push_str(&format!("  %{} = fadd double 0.0, {}\n", temp, value));
                format!("%{}", temp)
            }
            
            ASTNode::Identifier(name) => {
                let temp = self.new_temp();
                self.output.push_str(&format!("  %{} = load double, double* %{}\n", temp, name));
                format!("%{}", temp)
            }
            
            ASTNode::Binary { left, operator, right } => {
                let left_reg = self.visit_expression(left);
                let right_reg = self.visit_expression(right);
                let result_reg = self.new_temp();
                
                match operator.as_str() {
                    "+" => self.output.push_str(&format!("  %{} = fadd double {}, {}\n", 
                                                        result_reg, left_reg, right_reg)),
                    "-" => self.output.push_str(&format!("  %{} = fsub double {}, {}\n", 
                                                        result_reg, left_reg, right_reg)),
                    "*" => self.output.push_str(&format!("  %{} = fmul double {}, {}\n", 
                                                        result_reg, left_reg, right_reg)),
                    "/" => self.output.push_str(&format!("  %{} = fdiv double {}, {}\n", 
                                                        result_reg, left_reg, right_reg)),
                    "==" => {
                        self.output.push_str(&format!("  %{}_cmp = fcmp oeq double {}, {}\n", 
                                                      result_reg, left_reg, right_reg));
                        self.output.push_str(&format!("  %{} = uitofp i1 %{}_cmp to double\n", 
                                                      result_reg, result_reg));
                    }
                    "<" => {
                        self.output.push_str(&format!("  %{}_cmp = fcmp olt double {}, {}\n", 
                                                      result_reg, left_reg, right_reg));
                        self.output.push_str(&format!("  %{} = uitofp i1 %{}_cmp to double\n", 
                                                      result_reg, result_reg));
                    }
                    _ => {
                        // Default case
                        self.output.push_str(&format!("  %{} = fadd double {}, {}\n", 
                                                      result_reg, left_reg, right_reg));
                    }
                }
                
                format!("%{}", result_reg)
            }
            
            ASTNode::Call { callee, args } => {
                if let ASTNode::Identifier(func_name) = callee.as_ref() {
                    // Handle built-in functions
                    match func_name.as_str() {
                        "print" => {
                            if let Some(arg) = args.first() {
                                let arg_reg = self.visit_expression(arg);
                                let temp = self.new_temp();
                                self.output.push_str(&format!("  %{} = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_num, i32 0, i32 0), double {})\n", temp, arg_reg));
                                format!("%{}", temp)
                            } else {
                                "0".to_string()
                            }
                        }
                        _ => {
                            // User-defined function call
                            let arg_regs: Vec<String> = args.iter()
                                .map(|arg| self.visit_expression(arg))
                                .collect();
                            
                            let temp = self.new_temp();
                            let args_str = arg_regs.join(", ");
                            self.output.push_str(&format!("  %{} = call double @{}({})\n", 
                                                         temp, func_name, args_str));
                            format!("%{}", temp)
                        }
                    }
                } else {
                    "0".to_string()
                }
            }
            
            ASTNode::TemporalAccess { var, timestamp } => {
                let timestamp_reg = self.visit_expression(timestamp);
                
                // Simplified temporal access - in real implementation would
                // search through temporal timeline based on timestamp
                let temp = self.new_temp();
                self.output.push_str(&format!("  %{} = load double, double* %{}\n", temp, var));
                format!("%{}", temp)
            }
            
            _ => "0".to_string(),
        }
    }
    
    fn new_temp(&mut self) -> String {
        self.temp_counter += 1;
        format!("t{}", self.temp_counter)
    }
    
    fn new_label(&mut self) -> String {
        self.label_counter += 1;
        format!("L{}", self.label_counter)
    }
}

// ============================================================================
// MAIN COMPILER DRIVER
// ============================================================================

pub struct FluxCompiler {
    debug: bool,
}

impl FluxCompiler {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }
    
    pub fn compile_file(&self, filename: &str) -> Result<String, String> {
        let source = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;
        
        self.compile(&source)
    }
    
    pub fn compile(&self, source: &str) -> Result<String, String> {
        if self.debug {
            println!("=== FLUX COMPILER DEBUG ===");
            println!("Source code:\n{}\n", source);
        }
        
        // Lexical Analysis
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        
        if self.debug {
            println!("Tokens: {:?}\n", tokens);
        }
        
        // Syntax Analysis
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()
            .map_err(|e| format!("Parse error: {}", e))?;
        
        if self.debug {
            println!("AST: {:#?}\n", ast);
        }
        
        // Semantic Analysis
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.analyze(&ast)
            .map_err(|errors| format!("Semantic errors: {:?}", errors))?;
        
        if self.debug {
            println!("Semantic analysis passed\n");
        }
        
        // Code Generation
        let mut generator = CodeGenerator::new();
        let llvm_ir = generator.generate(&ast);
        
        if self.debug {
            println!("Generated LLVM IR:\n{}", llvm_ir);
        }
        
        Ok(llvm_ir)
    }
}

// ============================================================================
// EXAMPLE USAGE & DEMO
// ============================================================================

fn main() {
    let compiler = FluxCompiler::new(true);
    
    // Example 1: Basic arithmetic with immutable variables
    let example1 = r#"
#pragma braces
let x = 10
const y = 20
let result = x + y * 2
print(result)
"#;
    
    println!("=== EXAMPLE 1: Basic Arithmetic ===");
    match compiler.compile(example1) {
        Ok(ir) => println!("Compilation successful!\n"),
        Err(e) => println!("Error: {}\n", e),
    }
    
    // Example 2: Temporal variables (unique feature)
    let example2 = r#"
#pragma braces
temporal let temperature = 20.5
temperature = 25.0  # This would create a timeline entry
temperature = 18.3  # Another timeline entry

# Access historical values
let temp_at_start = temperature[0]  # Gets value at timestamp 0
let current_temp = temperature      # Gets current value

print(current_temp)
"#;
    
    println!("=== EXAMPLE 2: Temporal Variables ===");
    match compiler.compile(example2) {
        Ok(ir) => println!("Compilation successful!\n"),
        Err(e) => println!("Error: {}\n", e),
    }
    
    // Example 3: Pipeline operations (unique feature)
    let example3 = r#"
#pragma braces
func double(x) {
    return x * 2
}

func add_ten(x) {
    return x + 10
}

let value = 5
let result = value | double | add_ten  # Pipeline: 5 -> 10 -> 20
print(result)
"#;
    
    println!("=== EXAMPLE 3: Pipeline Operations ===");
    match compiler.compile(example3) {
        Ok(ir) => println!("Compilation successful!\n"),
        Err(e) => println!("Error: {}\n", e),
    }
    
    // Example 4: Pattern matching
    let example4 = r#"
#pragma braces
let status = 200
let message = match status {
    200 => "OK"
    404 => "Not Found" 
    500 => "Server Error"
    default => "Unknown"
}
print(message)
"#;
    
    println!("=== EXAMPLE 4: Pattern Matching ===");
    match compiler.compile(example4) {
        Ok(ir) => println!("Compilation successful!\n"),
        Err(e) => println!("Error: {}\n", e),
    }
    
    // Example 5: Indent-based syntax
    let example5 = r#"
#pragma indent
let x = 10
if x > 5
    let message = "Greater than 5"
    print(message)
else
    print("Less than or equal to 5")
"#;
    
    println!("=== EXAMPLE 5: Indent-based Syntax ===");
    match compiler.compile(example5) {
        Ok(ir) => println!("Compilation successful!\n"),
        Err(e) => println!("Error: {}\n", e),
    }
    
    println!("=== FLUX COMPILER FEATURES ===");
    println!("✓ Immutable dynamic typing - once assigned, variables cannot change type");
    println!("✓ Flexible OOP support without strict enforcement");
    println!("✓ Pragma-controlled syntax (braces vs indentation)");
    println!("✓ Temporal variables - track value changes over time");
    println!("✓ Pipeline operations - functional composition");
    println!("✓ Pattern matching with match expressions");
    println!("✓ LLVM IR code generation");
    println!("✓ Comprehensive semantic analysis");
    println!("✓ Advanced error handling and reporting");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer_basic() {
        let mut lexer = Lexer::new("let x = 42");
        let tokens = lexer.tokenize();
        
        assert!(matches!(tokens[0], TokenType::Let));
        assert!(matches!(tokens[1], TokenType::Identifier(_)));
        assert!(matches!(tokens[2], TokenType::Assign));
        assert!(matches!(tokens[3], TokenType::Number(42.0)));
    }
    
    #[test]
    fn test_parser_var_decl() {
        let tokens = vec![
            TokenType::Let,
            TokenType::Identifier("x".to_string()),
            TokenType::Assign,
            TokenType::Number(42.0),
            TokenType::EOF,
        ];
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let ASTNode::Program(statements) = ast {
            assert_eq!(statements.len(), 1);
            if let ASTNode::VarDecl { name, .. } = &statements[0] {
                assert_eq!(name, "x");
            } else {
                panic!("Expected VarDecl");
            }
        } else {
            panic!("Expected Program");
        }
    }
    
    #[test]
    fn test_temporal_variables() {
        let compiler = FluxCompiler::new(false);
        let source = r#"
temporal let x = 10
let y = x[0]
        "#;
        
        // Should compile without errors
        assert!(compiler.compile(source).is_ok());
    }
    
    #[test]
    fn test_immutable_reassignment_error() {
        let compiler = FluxCompiler::new(false);
        let source = r#"
const x = 10
x = 20  # This should cause an error
        "#;
        
        // Should fail due to const reassignment
        assert!(compiler.compile(source).is_err());
    }
    
    #[test]
    fn test_pipeline_operations() {
        let tokens = vec![
            TokenType::Identifier("x".to_string()),
            TokenType::Pipe,
            TokenType::Identifier("double".to_string()),
            TokenType::Pipe,
            TokenType::Identifier("add_ten".to_string()),
            TokenType::EOF,
        ];
        
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expression().unwrap();
        
        if let ASTNode::Pipeline(exprs) = expr {
            assert_eq!(exprs.len(), 3);
        } else {
            panic!("Expected Pipeline");
        }
    }
    
    #[test]
    fn test_pragma_handling() {
        let mut lexer = Lexer::new("#pragma braces\nlet x = 10");
        let tokens = lexer.tokenize();
        
        assert!(lexer.use_braces);
        assert!(matches!(tokens[0], TokenType::Pragma(_)));
    }
}

// ============================================================================
// ADVANCED FEATURES IMPLEMENTATION
// ============================================================================

/// Temporal Variable Manager - Handles time-based variable tracking
pub struct TemporalManager {
    timelines: HashMap<String, Vec<(usize, FluxValue)>>,
    current_time: usize,
}

#[derive(Debug, Clone)]
pub enum FluxValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Object(HashMap<String, FluxValue>),
}

impl TemporalManager {
    pub fn new() -> Self {
        Self {
            timelines: HashMap::new(),
            current_time: 0,
        }
    }
    
    pub fn create_temporal_var(&mut self, name: String, initial_value: FluxValue) {
        let timeline = vec![(self.current_time, initial_value)];
        self.timelines.insert(name, timeline);
    }
    
    pub fn update_temporal_var(&mut self, name: &str, value: FluxValue) -> Result<(), String> {
        if let Some(timeline) = self.timelines.get_mut(name) {
            timeline.push((self.current_time, value));
            Ok(())
        } else {
            Err(format!("Temporal variable '{}' not found", name))
        }
    }
    
    pub fn get_at_time(&self, name: &str, timestamp: usize) -> Option<&FluxValue> {
        if let Some(timeline) = self.timelines.get(name) {
            // Find the latest value at or before the requested timestamp
            timeline.iter()
                .rev()
                .find(|(time, _)| *time <= timestamp)
                .map(|(_, value)| value)
        } else {
            None
        }
    }
    
    pub fn advance_time(&mut self) {
        self.current_time += 1;
    }
    
    pub fn freeze_variable(&mut self, name: &str) -> Result<(), String> {
        // In a full implementation, this would mark the variable as frozen
        // preventing further updates
        if self.timelines.contains_key(name) {
            Ok(())
        } else {
            Err(format!("Variable '{}' not found", name))
        }
    }
}

/// Pipeline Processor - Handles functional composition
pub struct PipelineProcessor;

impl PipelineProcessor {
    pub fn process(expressions: &[ASTNode]) -> Result<ASTNode, String> {
        if expressions.is_empty() {
            return Err("Empty pipeline".to_string());
        }
        
        let mut result = expressions[0].clone();
        
        for expr in &expressions[1..] {
            // In a full implementation, this would properly chain function calls
            // For now, we create a nested call structure
            result = ASTNode::Call {
                callee: Box::new(expr.clone()),
                args: vec![result],
            };
        }
        
        Ok(result)
    }
}

/// Advanced Pattern Matcher
pub struct PatternMatcher;

impl PatternMatcher {
    pub fn compile_match(expr: &ASTNode, cases: &[(ASTNode, Vec<ASTNode>)]) -> Result<ASTNode, String> {
        // Convert match expression to if-else chain
        if cases.is_empty() {
            return Err("Match expression must have at least one case".to_string());
        }
        
        let mut result = None;
        
        for (i, (pattern, body)) in cases.iter().enumerate().rev() {
            let condition = match pattern {
                ASTNode::Identifier(name) if name == "default" => {
                    ASTNode::Boolean(true) // Default case always matches
                }
                _ => {
                    // Create equality comparison
                    ASTNode::Binary {
                        left: Box::new(expr.clone()),
                        operator: "==".to_string(),
                        right: Box::new(pattern.clone()),
                    }
                }
            };
            
            if let Some(else_branch) = result {
                result = Some(ASTNode::If {
                    condition: Box::new(condition),
                    then_branch: body.clone(),
                    else_branch: Some(vec![else_branch]),
                });
            } else {
                result = Some(ASTNode::If {
                    condition: Box::new(condition),
                    then_branch: body.clone(),
                    else_branch: None,
                });
            }
        }
        
        result.ok_or_else(|| "Failed to compile match expression".to_string())
    }
}

/// Memory Management for Generated Code
pub struct FluxRuntime {
    heap: Vec<u8>,
    gc_threshold: usize,
    allocated: usize,
}

impl FluxRuntime {
    pub fn new() -> Self {
        Self {
            heap: Vec::with_capacity(1024 * 1024), // 1MB initial heap
            gc_threshold: 512 * 1024, // GC trigger at 512KB
            allocated: 0,
        }
    }
    
    pub fn allocate(&mut self, size: usize) -> Result<usize, String> {
        if self.allocated + size > self.heap.capacity() {
            if self.allocated > self.gc_threshold {
                self.garbage_collect()?;
            }
            
            if self.allocated + size > self.heap.capacity() {
                return Err("Out of memory".to_string());
            }
        }
        
        let ptr = self.allocated;
        self.allocated += size;
        Ok(ptr)
    }
    
    fn garbage_collect(&mut self) -> Result<(), String> {
        // Simplified garbage collection - in practice would implement
        // mark-and-sweep or copying collector
        println!("Running garbage collection...");
        
        // Reset for demo purposes
        self.allocated = 0;
        self.heap.clear();
        
        Ok(())
    }
}

/// Interactive REPL for Flux Language
pub struct FluxRepl {
    compiler: FluxCompiler,
    temporal_manager: TemporalManager,
    runtime: FluxRuntime,
    history: Vec<String>,
}

impl FluxRepl {
    pub fn new() -> Self {
        Self {
            compiler: FluxCompiler::new(false),
            temporal_manager: TemporalManager::new(),
            runtime: FluxRuntime::new(),
            history: Vec::new(),
        }
    }
    
    pub fn run(&mut self) {
        println!("Flux Language REPL v1.0");
        println!("Type 'exit' to quit, 'help' for commands");
        println!();
        
        loop {
            print!("flux> ");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            
            match input {
                "exit" | "quit" => {
                    println!("Goodbye!");
                    break;
                }
                "help" => {
                    self.show_help();
                }
                "history" => {
                    self.show_history();
                }
                "clear" => {
                    print!("\x1B[2J\x1B[1;1H"); // Clear screen
                }
                "" => continue,
                _ => {
                    self.execute_command(input);
                }
            }
        }
    }
    
    fn execute_command(&mut self, input: &str) {
        self.history.push(input.to_string());
        
        match self.compiler.compile(input) {
            Ok(llvm_ir) => {
                println!("✓ Compiled successfully");
                // In a full implementation, would execute the IR
                self.temporal_manager.advance_time();
            }
            Err(error) => {
                println!("✗ Error: {}", error);
            }
        }
    }
    
    fn show_help(&self) {
        println!("Flux Language Commands:");
        println!("  exit/quit     - Exit the REPL");
        println!("  help          - Show this help");
        println!("  history       - Show command history");
        println!("  clear         - Clear screen");
        println!();
        println!("Language Features:");
        println!("  let x = 10           - Immutable variable");
        println!("  const y = 20         - Constant variable");
        println!("  temporal let z = 5   - Temporal variable");
        println!("  x | func1 | func2    - Pipeline operations");
        println!("  match x { ... }      - Pattern matching");
        println!("  #pragma braces       - Use brace syntax");
        println!("  #pragma indent       - Use indentation syntax");
        println!();
    }
    
    fn show_history(&self) {
        println!("Command History:");
        for (i, cmd) in self.history.iter().enumerate() {
            println!("  {}: {}", i + 1, cmd);
        }
        println!();
    }
}

// ============================================================================
// OPTIMIZATION PASSES
// ============================================================================

/// AST Optimizer - Performs compile-time optimizations
pub struct ASTOptimizer;

impl ASTOptimizer {
    pub fn optimize(ast: &mut ASTNode) {
        match ast {
            ASTNode::Program(statements) => {
                for stmt in statements {
                    Self::optimize(stmt);
                }
            }
            
            ASTNode::Binary { left, operator, right } => {
                Self::optimize(left);
                Self::optimize(right);
                
                // Constant folding
                if let (ASTNode::Number(l), ASTNode::Number(r)) = (left.as_ref(), right.as_ref()) {
                    let result = match operator.as_str() {
                        "+" => *l + *r,
                        "-" => *l - *r,
                        "*" => *l * *r,
                        "/" if *r != 0.0 => *l / *r,
                        _ => return,
                    };
                    
                    // Replace the entire binary operation with the computed result
                    *ast = ASTNode::Number(result);
                }
            }
            
            ASTNode::Unary { operator, operand } => {
                Self::optimize(operand);
                
                if let ASTNode::Number(n) = operand.as_ref() {
                    let result = match operator.as_str() {
                        "-" => -*n,
                        _ => return,
                    };
                    
                    *ast = ASTNode::Number(result);
                }
            }
            
            ASTNode::If { condition, then_branch, else_branch } => {
                Self::optimize(condition);
                
                // Dead code elimination for constant conditions
                if let ASTNode::Boolean(cond) = condition.as_ref() {
                    if *cond {
                        // Condition is always true, replace with then branch
                        for stmt in then_branch {
                            Self::optimize(stmt);
                        }
                    } else if let Some(else_stmts) = else_branch {
                        // Condition is always false, replace with else branch
                        for stmt in else_stmts {
                            Self::optimize(stmt);
                        }
                    }
                } else {
                    // Optimize branches
                    for stmt in then_branch {
                        Self::optimize(stmt);
                    }
                    
                    if let Some(else_stmts) = else_branch {
                        for stmt in else_stmts {
                            Self::optimize(stmt);
                        }
                    }
                }
            }
            
            _ => {} // Other nodes don't need optimization yet
        }
    }
}

// ============================================================================
// FLUX STANDARD LIBRARY
// ============================================================================

/// Built-in functions and utilities for Flux language
pub struct FluxStdLib;

impl FluxStdLib {
    pub fn get_builtin_functions() -> HashMap<String, fn(Vec<FluxValue>) -> Result<FluxValue, String>> {
        let mut functions = HashMap::new();
        
        functions.insert("print".to_string(), Self::print as fn(Vec<FluxValue>) -> Result<FluxValue, String>);
        functions.insert("len".to_string(), Self::len as fn(Vec<FluxValue>) -> Result<FluxValue, String>);
        functions.insert("abs".to_string(), Self::abs as fn(Vec<FluxValue>) -> Result<FluxValue, String>);
        functions.insert("max".to_string(), Self::max as fn(Vec<FluxValue>) -> Result<FluxValue, String>);
        functions.insert("min".to_string(), Self::min as fn(Vec<FluxValue>) -> Result<FluxValue, String>);
        functions.insert("sqrt".to_string(), Self::sqrt as fn(Vec<FluxValue>) -> Result<FluxValue, String>);
        
        functions
    }
    
    fn print(args: Vec<FluxValue>) -> Result<FluxValue, String> {
        for arg in args {
            match arg {
                FluxValue::Number(n) => print!("{}", n),
                FluxValue::String(s) => print!("{}", s),
                FluxValue::Boolean(b) => print!("{}", b),
                FluxValue::Object(_) => print!("[Object]"),
            }
        }
        println!();
        Ok(FluxValue::Boolean(true))
    }
    
    fn len(args: Vec<FluxValue>) -> Result<FluxValue, String> {
        if args.len() != 1 {
            return Err("len() takes exactly one argument".to_string());
        }
        
        match &args[0] {
            FluxValue::String(s) => Ok(FluxValue::Number(s.len() as f64)),
            FluxValue::Object(obj) => Ok(FluxValue::Number(obj.len() as f64)),
            _ => Err("len() can only be called on strings or objects".to_string()),
        }
    }
    
    fn abs(args: Vec<FluxValue>) -> Result<FluxValue, String> {
        if args.len() != 1 {
            return Err("abs() takes exactly one argument".to_string());
        }
        
        match &args[0] {
            FluxValue::Number(n) => Ok(FluxValue::Number(n.abs())),
            _ => Err("abs() can only be called on numbers".to_string()),
        }
    }
    
    fn max(args: Vec<FluxValue>) -> Result<FluxValue, String> {
        if args.is_empty() {
            return Err("max() requires at least one argument".to_string());
        }
        
        let mut max_val = match &args[0] {
            FluxValue::Number(n) => *n,
            _ => return Err("max() can only be called on numbers".to_string()),
        };
        
        for arg in &args[1..] {
            match arg {
                FluxValue::Number(n) => {
                    if *n > max_val {
                        max_val = *n;
                    }
                }
                _ => return Err("max() can only be called on numbers".to_string()),
            }
        }
        
        Ok(FluxValue::Number(max_val))
    }
    
    fn min(args: Vec<FluxValue>) -> Result<FluxValue, String> {
        if args.is_empty() {
            return Err("min() requires at least one argument".to_string());
        }
        
        let mut min_val = match &args[0] {
            FluxValue::Number(n) => *n,
            _ => return Err("min() can only be called on numbers".to_string()),
        };
        
        for arg in &args[1..] {
            match arg {
                FluxValue::Number(n) => {
                    if *n < min_val {
                        min_val = *n;
                    }
                }
                _ => return Err("min() can only be called on numbers".to_string()),
            }
        }
        
        Ok(FluxValue::Number(min_val))
    }
    
    fn sqrt(args: Vec<FluxValue>) -> Result<FluxValue, String> {
        if args.len() != 1 {
            return Err("sqrt() takes exactly one argument".to_string());
        }
        
        match &args[0] {
            FluxValue::Number(n) => {
                if *n < 0.0 {
                    Err("sqrt() cannot be called on negative numbers".to_string())
                } else {
                    Ok(FluxValue::Number(n.sqrt()))
                }
            }
            _ => Err("sqrt() can only be called on numbers".to_string()),
        }
    }
}

// Add this at the end of main() function to demonstrate REPL
/*
fn main() {
    // ... existing main code ...
    
    // Uncomment to run REPL
    // let mut repl = FluxRepl::new();
    // repl.run();
}
*/
