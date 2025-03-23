use anyhow::{Result, Context};

// Takes program text as input and tokenizes it.
// pub fn lex(input: &str) -> Vec<Token> {
//     todo!("Lexer has not yet been written.")
// }

pub struct Lexer;

impl Lexer {
    // Takes program text as input and tokenizes it.
    pub fn lex(input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut input = input.chars().peekable();
        while let Some(ch) = input.next() {
            match ch {
                '(' | ')' | '[' | ']' | '{' | '}' | '+' | '-' | '*' | '/' | ',' | '.' | ';'  => {
                    match Self::generate_single_token(ch) {
                        Ok(token) => tokens.push(token),
                        Err(err_message) => println!("{err_message}")
                    }
                }
                // NOTE: yea this sux
                '!' => {  
                    if let Some(&next) = input.peek() {
                        if next == '=' {
                            tokens.push(Token{kind: TokenKind::BangEqual, lexeme: "!=".to_string()});
                        }
                    } else {
                        tokens.push(Token{kind: TokenKind::Bang, lexeme: "!".to_string()});
                    }
                }
                '=' => {
                    if let Some(&next) = input.peek() {
                        if next == '=' {
                            tokens.push(Token{kind: TokenKind::EqualEqual, lexeme: "==".to_string()});
                        }
                    } else {
                        tokens.push(Token{kind: TokenKind::Equal, lexeme: "=".to_string()});
                    }
                }
                '>' => {
                    if let Some(&next) = input.peek() {
                        if next == '=' {
                            tokens.push(Token{kind: TokenKind::GreaterEqual, lexeme: ">=".to_string()});
                        }
                    } else {
                        tokens.push(Token{kind: TokenKind::Greater, lexeme: ">".to_string()});
                    }
                }
                '<' => {
                    if let Some(&next) = input.peek() {
                        if next == '=' {
                            tokens.push(Token{kind: TokenKind::LessEqual, lexeme: "<=".to_string()});
                        }
                    } else {
                        tokens.push(Token{kind: TokenKind::Less, lexeme: "<".to_string()});
                    }
                }
                _ => {
                    println!("hi :)");
                }
            }
        }                               
        return tokens;
    }
    
    fn generate_single_token(input: char) -> Result<Token, String> {
        return match input {
            '(' => Ok(Token{kind: TokenKind::LeftParen, lexeme: input.to_string()}),
            ')' => Ok(Token{kind: TokenKind::RightParen, lexeme: input.to_string()}),
            '[' => Ok(Token{kind: TokenKind::LeftBrace, lexeme: input.to_string()}),
            ']' => Ok(Token{kind: TokenKind::RightBrace, lexeme: input.to_string()}),
            '{' => Ok(Token{kind: TokenKind::LeftCurly, lexeme: input.to_string()}),
            '}' => Ok(Token{kind: TokenKind::RightCurly, lexeme: input.to_string()}),
            '+' => Ok(Token{kind: TokenKind::Plus, lexeme: input.to_string()}),
            '-' => Ok(Token{kind: TokenKind::Minus, lexeme: input.to_string()}),
            '*' => Ok(Token{kind: TokenKind::Star, lexeme: input.to_string()}),
            '/' => Ok(Token{kind: TokenKind::Slash, lexeme: input.to_string()}),
            ',' => Ok(Token{kind: TokenKind::Comma, lexeme: input.to_string()}),
            '.' => Ok(Token{kind: TokenKind::Dot, lexeme: input.to_string()}),
            ';' => Ok(Token{kind: TokenKind::Semicolon, lexeme: input.to_string()}),
             _  => Err("error: generate_single_token() called on invalid input".to_string())};
    }

    fn generate_double_token(input: &str) -> Result<Token, String> {
        return match input {
            "!=" => Ok(Token{kind: TokenKind::BangEqual, lexeme: input.to_string()}),
            "==" => Ok(Token{kind: TokenKind::EqualEqual, lexeme: input.to_string()}),
            ">=" => Ok(Token{kind: TokenKind::GreaterEqual, lexeme: input.to_string()}),
            "<=" => Ok(Token{kind: TokenKind::LessEqual, lexeme: input.to_string()}),
            _ => Err("error: generate_double_token() called on invalid input".to_string())
        }
    }
}

/// Struct representing the tokens in our language.
pub struct Token {
    kind: TokenKind,
    lexeme: String,
}

/// Enum representing the kinds of tokens in our language.
#[derive(Debug, PartialEq)]
enum TokenKind {
    // Single-character tokens
    LeftParen, RightParen,       // (  )
    LeftBrace, RightBrace,       // [  ]
    LeftCurly, RightCurly,       // {  }
    Plus, Minus, Star, Slash,    // +  -  *  /
    Comma, Dot, Semicolon,       // ,  .  ;

    // One or two character tokens
    Bang, BangEqual,             // !  !=
    Equal, EqualEqual,           // =  ==
    Greater, GreaterEqual,       // >  >=
    Less, LessEqual,             // <  <=
    // Literals
    Identifier, String, Number,

    // Keywords
    If, Else,                    // if    else
    For, While,                  // for   while
    True, False,                 // true  false
    Func, Return,                // fn    return
    Let, Const,                  // let   const
    Struct, 
}

/// Allows us to print TokenKinds or convert them to a string.
impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "[",
            TokenKind::RightBrace => "]",
            TokenKind::LeftCurly => "{",
            TokenKind::RightCurly => "}",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Comma => ",",
            TokenKind::Dot => ".",
            TokenKind::Semicolon => ";",
            TokenKind::Bang => "!",
            TokenKind::BangEqual => "!=",
            TokenKind::Equal => "=",
            TokenKind::EqualEqual => "==",
            TokenKind::Greater => ">",
            TokenKind::GreaterEqual => ">=",
            TokenKind::Less => "<",
            TokenKind::LessEqual => "<=",
            TokenKind::Identifier => "IDENTIFIER",
            TokenKind::String => "STRING",
            TokenKind::Number => "NUMBER",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::For => "for",
            TokenKind::While => "while",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Func => "fn",
            TokenKind::Return => "return",
            TokenKind::Let => "let",
            TokenKind::Const => "const",
            TokenKind::Struct => "struct",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub struct LexerError {
    message: String,
    line_number: usize,
    column_number: usize,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.message, self.line_number, self.column_number)
    }
}

#[cfg(test)]
mod tests;