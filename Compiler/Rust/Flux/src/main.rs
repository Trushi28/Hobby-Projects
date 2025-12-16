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
    Let, Const, Func, Return, If, Else, While, For, In,
    Class, Extends, New, This, Super,
    Import, Export, Match, Case, Default,
    Temporal, Freeze, Thaw, Timeline,
    Print, // Built-in print function
    
    // Operators
    Plus, Minus, Multiply, Divide, Modulo,
    Assign, Equal, NotEqual, Less, Greater,
    LessEqual, GreaterEqual, And, Or, Not,
    Arrow, FatArrow, Pipe, Compose,
    PlusAssign, MinusAssign, MulAssign, DivAssign, // Compound assignment
    DoubleDot, // Range operator ..
    
    // Delimiters
    LeftParen, RightParen, LeftBrace, RightBrace,
    LeftBracket, RightBracket, Comma, Semicolon,
    Colon, Dot, Question, Bang,
    
    // Control flow
    Break, Continue,
    
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
                    self.advance();
                    if self.current_char == Some('=') {
                        tokens.push(TokenType::PlusAssign);
                        self.advance();
                    } else {
                        tokens.push(TokenType::Plus);
                    }
                }
                
                '-' => {
                    self.advance();
                    if self.current_char == Some('>') {
                        tokens.push(TokenType::Arrow);
                        self.advance();
                    } else if self.current_char == Some('=') {
                        tokens.push(TokenType::MinusAssign);
                        self.advance();
                    } else {
                        tokens.push(TokenType::Minus);
                    }
                }
                
                '*' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        tokens.push(TokenType::MulAssign);
                        self.advance();
                    } else {
                        tokens.push(TokenType::Multiply);
                    }
                }
                
                '/' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        tokens.push(TokenType::DivAssign);
                        self.advance();
                    } else {
                        tokens.push(TokenType::Divide);
                    }
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
                        "in" => TokenType::In,
                        "break" => TokenType::Break,
                        "continue" => TokenType::Continue,
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
                        "print" => TokenType::Print,
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
    For {
        var: String,
        iterable: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
    Break,
    Continue,
    CompoundAssignment {
        name: String,
        operator: String, // +=, -=, *=, /=
        value: Box<ASTNode>,
    },
    
    // Expressions
    Binary { 
        left: Box<ASTNode>, 
        operator: String, 
        right: Box<ASTNode> 
    },
    Unary { operator: String, operand: Box<ASTNode> },
    Call { callee: Box<ASTNode>, args: Vec<ASTNode> },
    MemberAccess { object: Box<ASTNode>, property: String },
    IndexAccess { object: Box<ASTNode>, index: Box<ASTNode> },
    
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Array(Vec<ASTNode>),
    Object(Vec<(String, ASTNode)>),
    
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
            TokenType::For => self.parse_for(),
            TokenType::Break => {
                self.advance();
                Ok(ASTNode::Break)
            }
            TokenType::Continue => {
                self.advance();
                Ok(ASTNode::Continue)
            }
            TokenType::Identifier(_) => {
                // Look ahead to check if this is an assignment or compound assignment
                let name = if let TokenType::Identifier(n) = self.peek() { 
                    n.clone() 
                } else { 
                    unreachable!() 
                };
                self.advance(); // consume identifier
                
                match self.peek() {
                    TokenType::Assign => {
                        self.advance(); // consume '='
                        let value = self.parse_expression()?;
                        Ok(ASTNode::Assignment { 
                            name, 
                            value: Box::new(value) 
                        })
                    }
                    TokenType::PlusAssign => {
                        self.advance();
                        let value = self.parse_expression()?;
                        Ok(ASTNode::CompoundAssignment { 
                            name, 
                            operator: "+=".to_string(),
                            value: Box::new(value) 
                        })
                    }
                    TokenType::MinusAssign => {
                        self.advance();
                        let value = self.parse_expression()?;
                        Ok(ASTNode::CompoundAssignment { 
                            name, 
                            operator: "-=".to_string(),
                            value: Box::new(value) 
                        })
                    }
                    TokenType::MulAssign => {
                        self.advance();
                        let value = self.parse_expression()?;
                        Ok(ASTNode::CompoundAssignment { 
                            name, 
                            operator: "*=".to_string(),
                            value: Box::new(value) 
                        })
                    }
                    TokenType::DivAssign => {
                        self.advance();
                        let value = self.parse_expression()?;
                        Ok(ASTNode::CompoundAssignment { 
                            name, 
                            operator: "/=".to_string(),
                            value: Box::new(value) 
                        })
                    }
                    _ => {
                        // Not an assignment, backtrack and parse as expression
                        self.current -= 1;
                        self.parse_expression()
                    }
                }
            }
            _ => {
                self.parse_expression()
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
    
    fn parse_for(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'for'
        
        // Get loop variable
        let var = if let TokenType::Identifier(name) = self.peek() {
            let n = name.clone();
            self.advance();
            n
        } else {
            return Err("Expected identifier after 'for'".to_string());
        };
        
        // Consume 'in'
        self.consume(TokenType::In)?;
        
        // Parse iterable expression
        let iterable = self.parse_expression()?;
        
        // Parse body
        self.consume(TokenType::LeftBrace)?;
        let mut body = Vec::new();
        while !matches!(self.peek(), TokenType::RightBrace) {
            body.push(self.parse_statement()?);
        }
        self.consume(TokenType::RightBrace)?;
        
        Ok(ASTNode::For {
            var,
            iterable: Box::new(iterable),
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
            TokenType::Print => {
                // Treat print as an identifier for function calls
                self.advance();
                Ok(ASTNode::Identifier("print".to_string()))
            }
            TokenType::Default => {
                // Default keyword in match is treated as identifier
                self.advance();
                Ok(ASTNode::Identifier("default".to_string()))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenType::RightParen)?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                // Array literal: [1, 2, 3]
                self.advance();
                let mut elements = Vec::new();
                
                while !matches!(self.peek(), TokenType::RightBracket) {
                    elements.push(self.parse_expression()?);
                    if matches!(self.peek(), TokenType::Comma) {
                        self.advance();
                    }
                }
                
                self.consume(TokenType::RightBracket)?;
                Ok(ASTNode::Array(elements))
            }
            TokenType::LeftBrace => {
                // Object literal: { key: value, ... }
                self.advance();
                let mut properties = Vec::new();
                
                while !matches!(self.peek(), TokenType::RightBrace) {
                    let key = if let TokenType::Identifier(k) = self.peek() {
                        let key = k.clone();
                        self.advance();
                        key
                    } else {
                        return Err("Expected property name in object literal".to_string());
                    };
                    
                    self.consume(TokenType::Colon)?;
                    let value = self.parse_expression()?;
                    properties.push((key, value));
                    
                    if matches!(self.peek(), TokenType::Comma) {
                        self.advance();
                    }
                }
                
                self.consume(TokenType::RightBrace)?;
                Ok(ASTNode::Object(properties))
            }
            TokenType::Match => {
                // Match expression in expression context
                self.parse_match()
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
// INTERPRETER - Tree-Walking Execution Engine
// ============================================================================

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<RuntimeValue>),
    Object(HashMap<String, RuntimeValue>),
    Function {
        params: Vec<String>,
        body: Vec<ASTNode>,
        closure: HashMap<String, RuntimeValue>,
    },
    Null,
    Return(Box<RuntimeValue>), // For early return handling
}

impl std::fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Boolean(b) => write!(f, "{}", b),
            RuntimeValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", items.join(", "))
            }
            RuntimeValue::Object(obj) => {
                let items: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{{{}}}", items.join(", "))
            }
            RuntimeValue::Function { .. } => write!(f, "<function>"),
            RuntimeValue::Null => write!(f, "null"),
            RuntimeValue::Return(v) => write!(f, "{}", v),
        }
    }
}

pub struct Interpreter {
    global_scope: HashMap<String, RuntimeValue>,
    scopes: Vec<HashMap<String, RuntimeValue>>,
    functions: HashMap<String, (Vec<String>, Vec<ASTNode>)>,
    temporal_vars: HashMap<String, Vec<RuntimeValue>>,
    const_vars: std::collections::HashSet<String>,
    temporal_var_names: std::collections::HashSet<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            global_scope: HashMap::new(),
            scopes: Vec::new(),
            functions: HashMap::new(),
            temporal_vars: HashMap::new(),
            const_vars: std::collections::HashSet::new(),
            temporal_var_names: std::collections::HashSet::new(),
        }
    }
    
    pub fn execute(&mut self, program: &ASTNode) -> Result<RuntimeValue, String> {
        match program {
            ASTNode::Program(statements) => {
                let mut result = RuntimeValue::Null;
                for stmt in statements {
                    result = self.execute_statement(stmt)?;
                }
                Ok(result)
            }
            _ => self.execute_statement(program),
        }
    }
    
    fn execute_statement(&mut self, stmt: &ASTNode) -> Result<RuntimeValue, String> {
        match stmt {
            ASTNode::VarDecl { name, value, is_const, is_temporal } => {
                let val = self.evaluate(value)?;
                
                // Check if already declared
                if self.get_variable(name).is_some() {
                    return Err(format!("Variable '{}' is already declared", name));
                }
                
                if *is_temporal {
                    self.temporal_vars.insert(name.clone(), vec![val.clone()]);
                    self.temporal_var_names.insert(name.clone());
                }
                
                if *is_const {
                    self.const_vars.insert(name.clone());
                }
                
                self.set_variable(name.clone(), val.clone());
                Ok(val)
            }
            
            ASTNode::Assignment { name, value } => {
                // Check if variable exists
                if self.get_variable(name).is_none() {
                    return Err(format!("Variable '{}' is not declared", name));
                }
                
                // Check if const
                if self.const_vars.contains(name) {
                    return Err(format!("Cannot reassign constant variable '{}'", name));
                }
                
                let val = self.evaluate(value)?;
                
                // Update temporal variable history
                if self.temporal_var_names.contains(name) {
                    if let Some(history) = self.temporal_vars.get_mut(name) {
                        history.push(val.clone());
                    }
                }
                
                // Update in original scope
                self.update_variable(name, val.clone());
                Ok(val)
            }
            
            ASTNode::CompoundAssignment { name, operator, value } => {
                // Check if variable exists
                let current = self.get_variable(name)
                    .ok_or_else(|| format!("Variable '{}' is not declared", name))?;
                
                // Check if const
                if self.const_vars.contains(name) {
                    return Err(format!("Cannot reassign constant variable '{}'", name));
                }
                
                let rhs = self.evaluate(value)?;
                
                // Perform the operation
                let new_val = match operator.as_str() {
                    "+=" => self.add(current, rhs)?,
                    "-=" => self.subtract(current, rhs)?,
                    "*=" => self.multiply(current, rhs)?,
                    "/=" => self.divide(current, rhs)?,
                    _ => return Err(format!("Unknown compound operator: {}", operator)),
                };
                
                // Update temporal variable history
                if self.temporal_var_names.contains(name) {
                    if let Some(history) = self.temporal_vars.get_mut(name) {
                        history.push(new_val.clone());
                    }
                }
                
                self.update_variable(name, new_val.clone());
                Ok(new_val)
            }
            
            ASTNode::Break => Ok(RuntimeValue::String("__break__".to_string())),
            ASTNode::Continue => Ok(RuntimeValue::String("__continue__".to_string())),
            
            ASTNode::FunctionDecl { name, params, body } => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
                Ok(RuntimeValue::Null)
            }
            
            ASTNode::Return(expr) => {
                let val = self.evaluate(expr)?;
                Ok(RuntimeValue::Return(Box::new(val)))
            }
            
            ASTNode::If { condition, then_branch, else_branch } => {
                let cond = self.evaluate(condition)?;
                if self.is_truthy(&cond) {
                    for stmt in then_branch {
                        let result = self.execute_statement(stmt)?;
                        // Propagate return, break, and continue
                        if matches!(result, RuntimeValue::Return(_)) {
                            return Ok(result);
                        }
                        if let RuntimeValue::String(s) = &result {
                            if s == "__break__" || s == "__continue__" {
                                return Ok(result);
                            }
                        }
                    }
                } else if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        let result = self.execute_statement(stmt)?;
                        if matches!(result, RuntimeValue::Return(_)) {
                            return Ok(result);
                        }
                        if let RuntimeValue::String(s) = &result {
                            if s == "__break__" || s == "__continue__" {
                                return Ok(result);
                            }
                        }
                    }
                }
                Ok(RuntimeValue::Null)
            }
            
            ASTNode::While { condition, body } => {
                'outer: loop {
                    let cond_val = self.evaluate(condition)?;
                    if !self.is_truthy(&cond_val) {
                        break;
                    }
                    for stmt in body {
                        let result = self.execute_statement(stmt)?;
                        if let RuntimeValue::String(s) = &result {
                            if s == "__break__" {
                                break 'outer;
                            } else if s == "__continue__" {
                                continue 'outer;
                            }
                        }
                        if matches!(result, RuntimeValue::Return(_)) {
                            return Ok(result);
                        }
                    }
                }
                Ok(RuntimeValue::Null)
            }
            
            ASTNode::For { var, iterable, body } => {
                let iter_val = self.evaluate(iterable)?;
                match iter_val {
                    RuntimeValue::Array(items) => {
                        'for_outer: for item in items {
                            self.scopes.push(HashMap::new());
                            self.set_variable(var.clone(), item);
                            for stmt in body {
                                let result = self.execute_statement(stmt)?;
                                if let RuntimeValue::String(s) = &result {
                                    if s == "__break__" {
                                        self.scopes.pop();
                                        break 'for_outer;
                                    } else if s == "__continue__" {
                                        self.scopes.pop();
                                        continue 'for_outer;
                                    }
                                }
                                if matches!(result, RuntimeValue::Return(_)) {
                                    self.scopes.pop();
                                    return Ok(result);
                                }
                            }
                            self.scopes.pop();
                        }
                    }
                    _ => return Err("For loop requires an iterable (array)".to_string()),
                }
                Ok(RuntimeValue::Null)
            }
            
            _ => self.evaluate(stmt),
        }
    }
    
    fn evaluate(&mut self, expr: &ASTNode) -> Result<RuntimeValue, String> {
        match expr {
            ASTNode::Number(n) => Ok(RuntimeValue::Number(*n)),
            ASTNode::String(s) => Ok(RuntimeValue::String(s.clone())),
            ASTNode::Boolean(b) => Ok(RuntimeValue::Boolean(*b)),
            
            ASTNode::Identifier(name) => {
                self.get_variable(name)
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            
            ASTNode::Array(elements) => {
                let values: Result<Vec<RuntimeValue>, String> = elements
                    .iter()
                    .map(|e| self.evaluate(e))
                    .collect();
                Ok(RuntimeValue::Array(values?))
            }
            
            ASTNode::Object(properties) => {
                let mut obj = HashMap::new();
                for (key, value) in properties {
                    obj.insert(key.clone(), self.evaluate(value)?);
                }
                Ok(RuntimeValue::Object(obj))
            }
            
            ASTNode::Binary { left, operator, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                
                match operator.as_str() {
                    "+" => self.add(left_val, right_val),
                    "-" => self.subtract(left_val, right_val),
                    "*" => self.multiply(left_val, right_val),
                    "/" => self.divide(left_val, right_val),
                    "%" => self.modulo(left_val, right_val),
                    "==" => self.equals(left_val, right_val),
                    "!=" => self.not_equals(left_val, right_val),
                    "<" => self.less_than(left_val, right_val),
                    ">" => self.greater_than(left_val, right_val),
                    "<=" => self.less_equal(left_val, right_val),
                    ">=" => self.greater_equal(left_val, right_val),
                    "&&" => Ok(RuntimeValue::Boolean(self.is_truthy(&left_val) && self.is_truthy(&right_val))),
                    "||" => Ok(RuntimeValue::Boolean(self.is_truthy(&left_val) || self.is_truthy(&right_val))),
                    _ => Err(format!("Unknown operator: {}", operator)),
                }
            }
            
            ASTNode::Unary { operator, operand } => {
                let val = self.evaluate(operand)?;
                match operator.as_str() {
                    "-" => match val {
                        RuntimeValue::Number(n) => Ok(RuntimeValue::Number(-n)),
                        _ => Err("Cannot negate non-number".to_string()),
                    },
                    "!" => Ok(RuntimeValue::Boolean(!self.is_truthy(&val))),
                    _ => Err(format!("Unknown unary operator: {}", operator)),
                }
            }
            
            ASTNode::Call { callee, args } => {
                let arg_values: Result<Vec<RuntimeValue>, String> = args
                    .iter()
                    .map(|a| self.evaluate(a))
                    .collect();
                let arg_values = arg_values?;
                
                if let ASTNode::Identifier(func_name) = callee.as_ref() {
                    // Built-in functions
                    match func_name.as_str() {
                        "print" => {
                            let output: Vec<String> = arg_values.iter()
                                .map(|v| format!("{}", v))
                                .collect();
                            println!("{}", output.join(" "));
                            return Ok(RuntimeValue::Null);
                        }
                        "len" => {
                            if let Some(arg) = arg_values.first() {
                                match arg {
                                    RuntimeValue::String(s) => return Ok(RuntimeValue::Number(s.len() as f64)),
                                    RuntimeValue::Array(arr) => return Ok(RuntimeValue::Number(arr.len() as f64)),
                                    _ => return Err("len() requires string or array".to_string()),
                                }
                            }
                        }
                        "abs" => {
                            if let Some(RuntimeValue::Number(n)) = arg_values.first() {
                                return Ok(RuntimeValue::Number(n.abs()));
                            }
                        }
                        "sqrt" => {
                            if let Some(RuntimeValue::Number(n)) = arg_values.first() {
                                return Ok(RuntimeValue::Number(n.sqrt()));
                            }
                        }
                        "typeof" => {
                            if let Some(arg) = arg_values.first() {
                                let type_name = match arg {
                                    RuntimeValue::Number(_) => "number",
                                    RuntimeValue::String(_) => "string",
                                    RuntimeValue::Boolean(_) => "boolean",
                                    RuntimeValue::Array(_) => "array",
                                    RuntimeValue::Object(_) => "object",
                                    RuntimeValue::Function { .. } => "function",
                                    RuntimeValue::Null => "null",
                                    RuntimeValue::Return(_) => "return",
                                };
                                return Ok(RuntimeValue::String(type_name.to_string()));
                            }
                        }
                        "range" => {
                            // range(end) or range(start, end)
                            match arg_values.as_slice() {
                                [RuntimeValue::Number(end)] => {
                                    let arr: Vec<RuntimeValue> = (0..(*end as i64))
                                        .map(|n| RuntimeValue::Number(n as f64))
                                        .collect();
                                    return Ok(RuntimeValue::Array(arr));
                                }
                                [RuntimeValue::Number(start), RuntimeValue::Number(end)] => {
                                    let arr: Vec<RuntimeValue> = ((*start as i64)..(*end as i64))
                                        .map(|n| RuntimeValue::Number(n as f64))
                                        .collect();
                                    return Ok(RuntimeValue::Array(arr));
                                }
                                _ => return Err("range() requires 1 or 2 number arguments".to_string()),
                            }
                        }
                        
                        // Math functions
                        "floor" => {
                            if let Some(RuntimeValue::Number(n)) = arg_values.first() {
                                return Ok(RuntimeValue::Number(n.floor()));
                            }
                        }
                        "ceil" => {
                            if let Some(RuntimeValue::Number(n)) = arg_values.first() {
                                return Ok(RuntimeValue::Number(n.ceil()));
                            }
                        }
                        "round" => {
                            if let Some(RuntimeValue::Number(n)) = arg_values.first() {
                                return Ok(RuntimeValue::Number(n.round()));
                            }
                        }
                        "min" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::Number(a), RuntimeValue::Number(b)] => {
                                    return Ok(RuntimeValue::Number(a.min(*b)));
                                }
                                [RuntimeValue::Array(arr)] => {
                                    let mut min_val = f64::INFINITY;
                                    for item in arr {
                                        if let RuntimeValue::Number(n) = item {
                                            if *n < min_val { min_val = *n; }
                                        }
                                    }
                                    return Ok(RuntimeValue::Number(min_val));
                                }
                                _ => return Err("min() requires 2 numbers or an array".to_string()),
                            }
                        }
                        "max" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::Number(a), RuntimeValue::Number(b)] => {
                                    return Ok(RuntimeValue::Number(a.max(*b)));
                                }
                                [RuntimeValue::Array(arr)] => {
                                    let mut max_val = f64::NEG_INFINITY;
                                    for item in arr {
                                        if let RuntimeValue::Number(n) = item {
                                            if *n > max_val { max_val = *n; }
                                        }
                                    }
                                    return Ok(RuntimeValue::Number(max_val));
                                }
                                _ => return Err("max() requires 2 numbers or an array".to_string()),
                            }
                        }
                        "pow" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::Number(base), RuntimeValue::Number(exp)] => {
                                    return Ok(RuntimeValue::Number(base.powf(*exp)));
                                }
                                _ => return Err("pow() requires 2 number arguments".to_string()),
                            }
                        }
                        "sin" => {
                            if let Some(RuntimeValue::Number(n)) = arg_values.first() {
                                return Ok(RuntimeValue::Number(n.sin()));
                            }
                        }
                        "cos" => {
                            if let Some(RuntimeValue::Number(n)) = arg_values.first() {
                                return Ok(RuntimeValue::Number(n.cos()));
                            }
                        }
                        "tan" => {
                            if let Some(RuntimeValue::Number(n)) = arg_values.first() {
                                return Ok(RuntimeValue::Number(n.tan()));
                            }
                        }
                        "log" => {
                            if let Some(RuntimeValue::Number(n)) = arg_values.first() {
                                return Ok(RuntimeValue::Number(n.ln()));
                            }
                        }
                        "log10" => {
                            if let Some(RuntimeValue::Number(n)) = arg_values.first() {
                                return Ok(RuntimeValue::Number(n.log10()));
                            }
                        }
                        
                        // String functions
                        "upper" => {
                            if let Some(RuntimeValue::String(s)) = arg_values.first() {
                                return Ok(RuntimeValue::String(s.to_uppercase()));
                            }
                        }
                        "lower" => {
                            if let Some(RuntimeValue::String(s)) = arg_values.first() {
                                return Ok(RuntimeValue::String(s.to_lowercase()));
                            }
                        }
                        "trim" => {
                            if let Some(RuntimeValue::String(s)) = arg_values.first() {
                                return Ok(RuntimeValue::String(s.trim().to_string()));
                            }
                        }
                        "split" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::String(s), RuntimeValue::String(delim)] => {
                                    let parts: Vec<RuntimeValue> = s.split(delim.as_str())
                                        .map(|p| RuntimeValue::String(p.to_string()))
                                        .collect();
                                    return Ok(RuntimeValue::Array(parts));
                                }
                                _ => return Err("split() requires string and delimiter".to_string()),
                            }
                        }
                        "join" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::Array(arr), RuntimeValue::String(delim)] => {
                                    let parts: Vec<String> = arr.iter()
                                        .map(|v| format!("{}", v))
                                        .collect();
                                    return Ok(RuntimeValue::String(parts.join(delim)));
                                }
                                _ => return Err("join() requires array and delimiter".to_string()),
                            }
                        }
                        "replace" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::String(s), RuntimeValue::String(from), RuntimeValue::String(to)] => {
                                    return Ok(RuntimeValue::String(s.replace(from.as_str(), to.as_str())));
                                }
                                _ => return Err("replace() requires 3 string arguments".to_string()),
                            }
                        }
                        "contains" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::String(s), RuntimeValue::String(sub)] => {
                                    return Ok(RuntimeValue::Boolean(s.contains(sub.as_str())));
                                }
                                [RuntimeValue::Array(arr), val] => {
                                    for item in arr {
                                        if self.values_equal(item, val) {
                                            return Ok(RuntimeValue::Boolean(true));
                                        }
                                    }
                                    return Ok(RuntimeValue::Boolean(false));
                                }
                                _ => return Err("contains() requires string/substring or array/value".to_string()),
                            }
                        }
                        "starts_with" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::String(s), RuntimeValue::String(prefix)] => {
                                    return Ok(RuntimeValue::Boolean(s.starts_with(prefix.as_str())));
                                }
                                _ => return Err("starts_with() requires 2 string arguments".to_string()),
                            }
                        }
                        "ends_with" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::String(s), RuntimeValue::String(suffix)] => {
                                    return Ok(RuntimeValue::Boolean(s.ends_with(suffix.as_str())));
                                }
                                _ => return Err("ends_with() requires 2 string arguments".to_string()),
                            }
                        }
                        "char_at" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::String(s), RuntimeValue::Number(idx)] => {
                                    let i = *idx as usize;
                                    if let Some(c) = s.chars().nth(i) {
                                        return Ok(RuntimeValue::String(c.to_string()));
                                    }
                                    return Err(format!("Index {} out of bounds", i));
                                }
                                _ => return Err("char_at() requires string and index".to_string()),
                            }
                        }
                        "substr" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::String(s), RuntimeValue::Number(start), RuntimeValue::Number(len)] => {
                                    let start = *start as usize;
                                    let len = *len as usize;
                                    let result: String = s.chars().skip(start).take(len).collect();
                                    return Ok(RuntimeValue::String(result));
                                }
                                _ => return Err("substr() requires string, start, and length".to_string()),
                            }
                        }
                        
                        // Array functions
                        "push" => {
                            match arg_values.as_slice() {
                                [RuntimeValue::Array(arr), value] => {
                                    let mut new_arr = arr.clone();
                                    new_arr.push(value.clone());
                                    return Ok(RuntimeValue::Array(new_arr));
                                }
                                _ => return Err("push() requires array and value".to_string()),
                            }
                        }
                        "pop" => {
                            if let Some(RuntimeValue::Array(arr)) = arg_values.first() {
                                let mut new_arr = arr.clone();
                                new_arr.pop();
                                return Ok(RuntimeValue::Array(new_arr));
                            }
                        }
                        "first" => {
                            if let Some(RuntimeValue::Array(arr)) = arg_values.first() {
                                return Ok(arr.first().cloned().unwrap_or(RuntimeValue::Null));
                            }
                        }
                        "last" => {
                            if let Some(RuntimeValue::Array(arr)) = arg_values.first() {
                                return Ok(arr.last().cloned().unwrap_or(RuntimeValue::Null));
                            }
                        }
                        "reverse" => {
                            if let Some(RuntimeValue::Array(arr)) = arg_values.first() {
                                let mut new_arr = arr.clone();
                                new_arr.reverse();
                                return Ok(RuntimeValue::Array(new_arr));
                            }
                            if let Some(RuntimeValue::String(s)) = arg_values.first() {
                                return Ok(RuntimeValue::String(s.chars().rev().collect()));
                            }
                        }
                        "sort" => {
                            if let Some(RuntimeValue::Array(arr)) = arg_values.first() {
                                let mut new_arr = arr.clone();
                                new_arr.sort_by(|a, b| {
                                    match (a, b) {
                                        (RuntimeValue::Number(x), RuntimeValue::Number(y)) => {
                                            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                                        }
                                        (RuntimeValue::String(x), RuntimeValue::String(y)) => x.cmp(y),
                                        _ => std::cmp::Ordering::Equal,
                                    }
                                });
                                return Ok(RuntimeValue::Array(new_arr));
                            }
                        }
                        "sum" => {
                            if let Some(RuntimeValue::Array(arr)) = arg_values.first() {
                                let mut total = 0.0;
                                for item in arr {
                                    if let RuntimeValue::Number(n) = item {
                                        total += n;
                                    }
                                }
                                return Ok(RuntimeValue::Number(total));
                            }
                        }
                        "avg" => {
                            if let Some(RuntimeValue::Array(arr)) = arg_values.first() {
                                let mut total = 0.0;
                                let mut count = 0;
                                for item in arr {
                                    if let RuntimeValue::Number(n) = item {
                                        total += n;
                                        count += 1;
                                    }
                                }
                                if count > 0 {
                                    return Ok(RuntimeValue::Number(total / count as f64));
                                }
                            }
                        }
                        
                        // Type conversion
                        "int" | "parseInt" => {
                            match arg_values.first() {
                                Some(RuntimeValue::Number(n)) => return Ok(RuntimeValue::Number(n.floor())),
                                Some(RuntimeValue::String(s)) => {
                                    if let Ok(n) = s.parse::<f64>() {
                                        return Ok(RuntimeValue::Number(n.floor()));
                                    }
                                    return Err(format!("Cannot parse '{}' as integer", s));
                                }
                                _ => return Err("parseInt() requires number or string".to_string()),
                            }
                        }
                        "float" | "parseFloat" => {
                            match arg_values.first() {
                                Some(RuntimeValue::Number(n)) => return Ok(RuntimeValue::Number(*n)),
                                Some(RuntimeValue::String(s)) => {
                                    if let Ok(n) = s.parse::<f64>() {
                                        return Ok(RuntimeValue::Number(n));
                                    }
                                    return Err(format!("Cannot parse '{}' as float", s));
                                }
                                _ => return Err("parseFloat() requires number or string".to_string()),
                            }
                        }
                        "str" | "toString" => {
                            if let Some(arg) = arg_values.first() {
                                return Ok(RuntimeValue::String(format!("{}", arg)));
                            }
                        }
                        "bool" => {
                            if let Some(arg) = arg_values.first() {
                                return Ok(RuntimeValue::Boolean(self.is_truthy(arg)));
                            }
                        }
                        
                        // Utility
                        "assert" => {
                            match arg_values.as_slice() {
                                [cond, RuntimeValue::String(msg)] => {
                                    if !self.is_truthy(cond) {
                                        return Err(format!("Assertion failed: {}", msg));
                                    }
                                    return Ok(RuntimeValue::Null);
                                }
                                [cond] => {
                                    if !self.is_truthy(cond) {
                                        return Err("Assertion failed".to_string());
                                    }
                                    return Ok(RuntimeValue::Null);
                                }
                                _ => return Err("assert() requires condition and optional message".to_string()),
                            }
                        }
                        "keys" => {
                            if let Some(RuntimeValue::Object(obj)) = arg_values.first() {
                                let keys: Vec<RuntimeValue> = obj.keys()
                                    .map(|k| RuntimeValue::String(k.clone()))
                                    .collect();
                                return Ok(RuntimeValue::Array(keys));
                            }
                        }
                        "values" => {
                            if let Some(RuntimeValue::Object(obj)) = arg_values.first() {
                                let vals: Vec<RuntimeValue> = obj.values().cloned().collect();
                                return Ok(RuntimeValue::Array(vals));
                            }
                        }
                        
                        _ => {}
                    }
                    
                    // User-defined functions
                    if let Some((params, body)) = self.functions.get(func_name).cloned() {
                        if params.len() != arg_values.len() {
                            return Err(format!(
                                "Function '{}' expected {} arguments, got {}", 
                                func_name, params.len(), arg_values.len()
                            ));
                        }
                        
                        // Create new scope with parameters
                        self.scopes.push(HashMap::new());
                        for (param, value) in params.iter().zip(arg_values.iter()) {
                            self.set_variable(param.clone(), value.clone());
                        }
                        
                        // Execute function body
                        let mut result = RuntimeValue::Null;
                        for stmt in &body {
                            result = self.execute_statement(stmt)?;
                            // Handle return - unwrap the Return wrapper
                            if let RuntimeValue::Return(val) = result {
                                result = *val;
                                break;
                            }
                        }
                        
                        self.scopes.pop();
                        return Ok(result);
                    }
                    
                    Err(format!("Undefined function: {}", func_name))
                } else {
                    Err("Cannot call non-function".to_string())
                }
            }
            
            ASTNode::Pipeline(exprs) => {
                if exprs.is_empty() {
                    return Ok(RuntimeValue::Null);
                }
                
                // Start with first expression
                let mut current = self.evaluate(&exprs[0])?;
                
                // Pass through each function in the pipeline
                for func_expr in &exprs[1..] {
                    if let ASTNode::Identifier(func_name) = func_expr {
                        if let Some((params, body)) = self.functions.get(func_name).cloned() {
                            if params.len() != 1 {
                                return Err(format!(
                                    "Pipeline function '{}' must take exactly 1 argument", 
                                    func_name
                                ));
                            }
                            
                            self.scopes.push(HashMap::new());
                            self.set_variable(params[0].clone(), current);
                            
                            let mut result = RuntimeValue::Null;
                            for stmt in &body {
                                result = self.execute_statement(stmt)?;
                                if let RuntimeValue::Return(val) = result {
                                    result = *val;
                                    break;
                                }
                            }
                            
                            self.scopes.pop();
                            current = result;
                        } else {
                            return Err(format!("Undefined pipeline function: {}", func_name));
                        }
                    }
                }
                
                Ok(current)
            }
            
            ASTNode::TemporalAccess { var, timestamp } => {
                let ts = self.evaluate(timestamp)?;
                let index = match ts {
                    RuntimeValue::Number(n) => n as usize,
                    _ => return Err("Temporal access index must be a number".to_string()),
                };
                
                if let Some(history) = self.temporal_vars.get(var) {
                    history.get(index)
                        .cloned()
                        .ok_or_else(|| format!("No value at temporal index {} for '{}'", index, var))
                } else {
                    Err(format!("'{}' is not a temporal variable", var))
                }
            }
            
            ASTNode::Match { expr, cases } => {
                let match_val = self.evaluate(expr)?;
                
                for (pattern, body) in cases {
                    let pattern_val = self.evaluate(pattern)?;
                    
                    // Check if pattern matches (default always matches)
                    let matches = if let ASTNode::Identifier(name) = pattern {
                        name == "default" || self.values_equal(&match_val, &pattern_val)
                    } else {
                        self.values_equal(&match_val, &pattern_val)
                    };
                    
                    if matches {
                        // Execute the matching case body
                        let mut result = RuntimeValue::Null;
                        for stmt in body {
                            result = self.execute_statement(stmt)?;
                        }
                        return Ok(result);
                    }
                }
                
                Ok(RuntimeValue::Null)
            }
            
            ASTNode::MemberAccess { object, property } => {
                let obj = self.evaluate(object)?;
                match obj {
                    RuntimeValue::Object(map) => {
                        map.get(property)
                            .cloned()
                            .ok_or_else(|| format!("Property '{}' not found", property))
                    }
                    _ => Err("Cannot access property on non-object".to_string()),
                }
            }
            
            ASTNode::IndexAccess { object, index } => {
                let obj = self.evaluate(object)?;
                let idx = self.evaluate(index)?;
                
                match (obj, idx) {
                    (RuntimeValue::Array(arr), RuntimeValue::Number(n)) => {
                        let i = n as usize;
                        arr.get(i)
                            .cloned()
                            .ok_or_else(|| format!("Array index {} out of bounds", i))
                    }
                    (RuntimeValue::String(s), RuntimeValue::Number(n)) => {
                        let i = n as usize;
                        s.chars()
                            .nth(i)
                            .map(|c| RuntimeValue::String(c.to_string()))
                            .ok_or_else(|| format!("String index {} out of bounds", i))
                    }
                    _ => Err("Cannot index non-array/string or with non-number".to_string()),
                }
            }
            
            _ => Ok(RuntimeValue::Null),
        }
    }
    
    // Helper methods for arithmetic operations
    fn add(&self, left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, String> {
        match (left, right) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a + b)),
            (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(RuntimeValue::String(a + &b)),
            (RuntimeValue::String(a), b) => Ok(RuntimeValue::String(format!("{}{}", a, b))),
            (a, RuntimeValue::String(b)) => Ok(RuntimeValue::String(format!("{}{}", a, b))),
            _ => Err("Cannot add these types".to_string()),
        }
    }
    
    fn subtract(&self, left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, String> {
        match (left, right) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a - b)),
            _ => Err("Cannot subtract non-numbers".to_string()),
        }
    }
    
    fn multiply(&self, left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, String> {
        match (left, right) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a * b)),
            _ => Err("Cannot multiply non-numbers".to_string()),
        }
    }
    
    fn divide(&self, left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, String> {
        match (left, right) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(RuntimeValue::Number(a / b))
                }
            }
            _ => Err("Cannot divide non-numbers".to_string()),
        }
    }
    
    fn modulo(&self, left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, String> {
        match (left, right) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a % b)),
            _ => Err("Cannot modulo non-numbers".to_string()),
        }
    }
    
    fn equals(&self, left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, String> {
        Ok(RuntimeValue::Boolean(self.values_equal(&left, &right)))
    }
    
    fn not_equals(&self, left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, String> {
        Ok(RuntimeValue::Boolean(!self.values_equal(&left, &right)))
    }
    
    fn less_than(&self, left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, String> {
        match (left, right) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Boolean(a < b)),
            _ => Err("Cannot compare non-numbers".to_string()),
        }
    }
    
    fn greater_than(&self, left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, String> {
        match (left, right) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Boolean(a > b)),
            _ => Err("Cannot compare non-numbers".to_string()),
        }
    }
    
    fn less_equal(&self, left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, String> {
        match (left, right) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Boolean(a <= b)),
            _ => Err("Cannot compare non-numbers".to_string()),
        }
    }
    
    fn greater_equal(&self, left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, String> {
        match (left, right) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Boolean(a >= b)),
            _ => Err("Cannot compare non-numbers".to_string()),
        }
    }
    
    fn values_equal(&self, a: &RuntimeValue, b: &RuntimeValue) -> bool {
        match (a, b) {
            (RuntimeValue::Number(x), RuntimeValue::Number(y)) => (x - y).abs() < f64::EPSILON,
            (RuntimeValue::String(x), RuntimeValue::String(y)) => x == y,
            (RuntimeValue::Boolean(x), RuntimeValue::Boolean(y)) => x == y,
            (RuntimeValue::Null, RuntimeValue::Null) => true,
            _ => false,
        }
    }
    
    fn is_truthy(&self, val: &RuntimeValue) -> bool {
        match val {
            RuntimeValue::Boolean(b) => *b,
            RuntimeValue::Number(n) => *n != 0.0,
            RuntimeValue::String(s) => !s.is_empty(),
            RuntimeValue::Null => false,
            RuntimeValue::Array(a) => !a.is_empty(),
            RuntimeValue::Object(o) => !o.is_empty(),
            RuntimeValue::Function { .. } => true,
            RuntimeValue::Return(v) => self.is_truthy(v),
        }
    }
    
    fn get_variable(&self, name: &str) -> Option<RuntimeValue> {
        // Check local scopes first (innermost to outermost)
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        // Then check global scope
        self.global_scope.get(name).cloned()
    }
    
    fn set_variable(&mut self, name: String, value: RuntimeValue) {
        // For new declarations, add to current scope
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        } else {
            self.global_scope.insert(name, value);
        }
    }
    
    fn update_variable(&mut self, name: &str, value: RuntimeValue) -> bool {
        // Update in the scope where it was originally declared
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return true;
            }
        }
        // Check global scope
        if self.global_scope.contains_key(name) {
            self.global_scope.insert(name.to_string(), value);
            return true;
        }
        false
    }
}

// ============================================================================
// EXAMPLE USAGE & DEMO
// ============================================================================

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }
    
    match args[1].as_str() {
        "run" | "interpret" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                eprintln!("Usage: {} run <file.flux>", args[0]);
                return;
            }
            run_file(&args[2]);
        }
        "compile" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                eprintln!("Usage: {} compile <file.flux> [output.ll]", args[0]);
                return;
            }
            let output = if args.len() > 3 { &args[3] } else { "output.ll" };
            compile_file(&args[2], output);
        }
        "repl" => {
            run_repl();
        }
        "demo" => {
            run_demo();
        }
        "--help" | "-h" | "help" => {
            print_usage(&args[0]);
        }
        file if file.ends_with(".flux") => {
            // Default: run the file
            run_file(file);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage(&args[0]);
        }
    }
}

fn print_usage(program: &str) {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║            🦀 FLUX PROGRAMMING LANGUAGE v2.0 🦀                   ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("USAGE:");
    println!("    {} <command> [options]", program);
    println!("    {} <file.flux>                  Run a Flux program", program);
    println!();
    println!("COMMANDS:");
    println!("    run <file>         Interpret and execute a Flux program");
    println!("    compile <file>     Compile to LLVM IR");
    println!("    repl               Start interactive REPL");
    println!("    demo               Run demo examples");
    println!("    help               Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    {} run examples/temporal.flux", program);
    println!("    {} compile examples/pipeline.flux output.ll", program);
    println!("    {} repl", program);
    println!();
    println!("LANGUAGE FEATURES:");
    println!("    ⏰ Temporal Variables   - Track variable changes across time");
    println!("    🔗 Pipeline Operations  - Functional composition with | operator");
    println!("    ❄️  Immutable Typing     - Once assigned, variables cannot change type");
    println!("    🎯 Pattern Matching     - Advanced match expressions");
    println!("    📦 For-in Loops         - Iterate over arrays with break/continue");
    println!("    🧱 Object Literals      - Create objects with {{ key: value }}");
    println!("    ➕ Compound Operators   - +=, -=, *=, /= for concise updates");
    println!("    📚 50+ Built-ins        - Math, string, array functions");
}

fn run_file(filename: &str) {
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            return;
        }
    };
    
    println!("🚀 Running: {}", filename);
    println!("{}", "─".repeat(50));
    
    // Lexical Analysis
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    
    // Parse
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("❌ Parse Error: {}", e);
            return;
        }
    };
    
    // Interpret
    let mut interpreter = Interpreter::new();
    match interpreter.execute(&ast) {
        Ok(_) => {
            println!("{}", "─".repeat(50));
            println!("✅ Program finished successfully");
        }
        Err(e) => {
            eprintln!("❌ Runtime Error: {}", e);
        }
    }
}

fn compile_file(input: &str, output: &str) {
    let source = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input, e);
            return;
        }
    };
    
    println!("📦 Compiling: {} -> {}", input, output);
    
    let compiler = FluxCompiler::new(false);
    match compiler.compile(&source) {
        Ok(llvm_ir) => {
            match fs::write(output, &llvm_ir) {
                Ok(_) => {
                    println!("✅ LLVM IR written to: {}", output);
                    println!();
                    println!("To create an executable, run:");
                    println!("    llc -filetype=obj {} -o output.o", output);
                    println!("    clang output.o -o program");
                    println!("    ./program");
                }
                Err(e) => eprintln!("Error writing output: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Compilation Error: {}", e),
    }
}

fn run_repl() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║            🦀 FLUX REPL v1.0 - Interactive Mode 🦀                ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Type Flux code to execute. Commands:");
    println!("  :help     - Show help");
    println!("  :clear    - Clear screen");
    println!("  :quit     - Exit REPL");
    println!();
    
    let mut interpreter = Interpreter::new();
    let mut input_buffer = String::new();
    
    loop {
        let prompt = if input_buffer.is_empty() { "flux> " } else { "  ... " };
        print!("{}", prompt);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err() {
            break;
        }
        
        let trimmed = line.trim();
        
        // Handle commands
        if trimmed.starts_with(':') {
            match trimmed {
                ":quit" | ":q" | ":exit" => {
                    println!("Goodbye! 👋");
                    break;
                }
                ":clear" | ":cls" => {
                    print!("\x1B[2J\x1B[1;1H");
                    continue;
                }
                ":help" | ":h" => {
                    println!("REPL Commands:");
                    println!("  :help     - Show this help");
                    println!("  :clear    - Clear screen");
                    println!("  :quit     - Exit REPL");
                    println!();
                    println!("Language Features:");
                    println!("  let x = 10           - Variable declaration");
                    println!("  temporal let y = 5   - Temporal variable");
                    println!("  func foo(a) {{ ... }} - Function declaration");
                    println!("  x | foo | bar        - Pipeline operations");
                    println!("  print(x)             - Print value");
                    continue;
                }
                _ => {
                    println!("Unknown command: {}", trimmed);
                    continue;
                }
            }
        }
        
        // Check if line continues
        if trimmed.ends_with('{') && !trimmed.contains('}') {
            input_buffer.push_str(&line);
            continue;
        }
        
        input_buffer.push_str(&line);
        
        // Try to parse and execute
        let source = input_buffer.clone();
        input_buffer.clear();
        
        if source.trim().is_empty() {
            continue;
        }
        
        // Add implicit #pragma braces for REPL
        let source = format!("#pragma braces\n{}", source);
        
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(ast) => {
                match interpreter.execute(&ast) {
                    Ok(result) => {
                        match result {
                            RuntimeValue::Null => {}
                            other => println!("=> {}", other),
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(e) => eprintln!("Parse Error: {}", e),
        }
    }
}

fn run_source(source: &str) {
    // Add implicit #pragma braces
    let source = format!("#pragma braces\n{}", source);
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            let mut interpreter = Interpreter::new();
            match interpreter.execute(&ast) {
                Ok(_) => {}
                Err(e) => eprintln!("Runtime Error: {}", e),
            }
        }
        Err(e) => eprintln!("Parse Error: {}", e),
    }
}

fn run_demo() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║               🦀 FLUX COMPILER DEMO 🦀                            ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();
    
    // Demo 1: Basic Arithmetic
    println!("═══ Demo 1: Basic Arithmetic ═══");
    let source1 = r#"
let x = 10
let y = 20
let result = x + y * 2
print("x =", x)
print("y =", y)
print("x + y * 2 =", result)
"#;
    run_source(source1);
    
    // Demo 2: Temporal Variables
    println!("\n═══ Demo 2: Temporal Variables ═══");
    let source2 = r#"
temporal let temperature = 20.5
print("Initial:", temperature)
temperature = 25.0
print("After update:", temperature)
temperature = 18.3
print("Final:", temperature)
print("History - t=0:", temperature[0])
print("History - t=1:", temperature[1])
print("History - t=2:", temperature[2])
"#;
    run_source(source2);
    
    // Demo 3: Pipeline Operations
    println!("\n═══ Demo 3: Pipeline Operations ═══");
    let source3 = r#"
func double(x) {
    return x * 2
}

func add_ten(x) {
    return x + 10
}

let value = 5
print("Starting value:", value)
let result = value | double | add_ten
print("After pipeline (double then add_ten):", result)
"#;
    run_source(source3);
    
    // Demo 4: Pattern Matching
    println!("\n═══ Demo 4: Pattern Matching ═══");
    let source4 = r#"
let status = 404

let message = match status {
    200 => "OK"
    404 => "Not Found"
    500 => "Server Error"
    default => "Unknown"
}

print("Status", status, "->", message)
"#;
    run_source(source4);
    
    // Demo 5: Arrays and For Loops
    println!("\n═══ Demo 5: Arrays and For Loops ═══");
    let source5 = r#"
let numbers = [1, 2, 3, 4, 5]
print("Array:", numbers)

let sum = 0
for n in numbers {
    sum += n
}
print("Sum:", sum)
"#;
    run_source(source5);
    
    // Demo 6: Functions and Recursion
    println!("\n═══ Demo 6: Functions ═══");
    let source6 = r#"
func factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print("factorial(5) =", factorial(5))
print("factorial(10) =", factorial(10))
"#;
    run_source(source6);
    
    // Demo 7: Compound Operators and Break/Continue
    println!("\n═══ Demo 7: Compound Operators & Control Flow ═══");
    let source7 = r#"
let counter = 0
for i in range(10) {
    if i == 5 {
        print("Skipping 5")
        continue
    }
    if i == 8 {
        print("Breaking at 8")
        break
    }
    counter += 1
}
print("Counter:", counter)

let product = 1
for i in range(1, 6) {
    product *= i
}
print("Product of 1-5:", product)
"#;
    run_source(source7);
    
    // Demo 8: Built-in Functions
    println!("\n═══ Demo 8: Built-in Functions ═══");
    let source8 = r#"
let arr = [5, 2, 8, 1, 9, 3]
print("Original:", arr)
print("Sorted:", sort(arr))
print("Sum:", sum(arr))
print("Avg:", avg(arr))
print("Min:", min(arr))
print("Max:", max(arr))

let text = "  Hello World  "
print("Original:", text)
print("Trimmed:", trim(text))
print("Upper:", upper(trim(text)))
print("Lower:", lower(trim(text)))

let words = split("a,b,c,d", ",")
print("Split result:", words)
print("Join result:", join(words, "-"))
print("Contains 'b':", contains(words, "b"))

print("pow(2, 10) =", pow(2, 10))
print("sqrt(144) =", sqrt(144))
print("floor(3.7) =", floor(3.7))
print("ceil(3.2) =", ceil(3.2))
"#;
    run_source(source8);
    
    // Demo 9: Object Literals
    println!("\n═══ Demo 9: Objects ═══");
    let source9 = r#"
let person = {
    name: "Alice",
    age: 30,
    city: "Paris"
}
print("Person:", person)
print("Keys:", keys(person))
print("Values:", values(person))
print("Name:", person.name)
"#;
    run_source(source9);
    
    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("✅ All demos completed!");
    println!();
    println!("FLUX LANGUAGE FEATURES:");
    println!("  ✓ Immutable dynamic typing");
    println!("  ✓ Temporal variables with history tracking");
    println!("  ✓ Pipeline operations for functional composition");
    println!("  ✓ Pattern matching with match expressions");
    println!("  ✓ Arrays and for-in loops");
    println!("  ✓ User-defined functions with recursion");
    println!("  ✓ Compound operators (+=, -=, *=, /=)");
    println!("  ✓ Break and continue in loops");
    println!("  ✓ 50+ built-in functions");
    println!("  ✓ Object literals and properties");
    println!("  ✓ LLVM IR code generation");
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
        println!("  match x {{ ... }}      - Pattern matching");
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
