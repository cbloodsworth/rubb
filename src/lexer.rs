use anyhow::{Result, Context, anyhow};

// Takes program text as input and tokenizes it.
// pub fn lex(input: &str) -> Vec<Token> {
//     todo!("Lexer has not yet been written.")
// }

pub struct Lexer;

impl Lexer {
    /// Takes program text as input and tokenizes it.
    pub fn lex(input: &str) -> Result<Vec<Token>,LexerError> {
        let mut tokens: Vec<Token> = vec![];
        let mut input = input.chars().peekable();
        // TODO: Implement line and column tracking for proper error messages
        let line_number: usize = 0;
        let column_number: usize = 0;
        while let Some(ch) = input.next() {
            match ch {
                '(' | ')' | '[' | ']' | '{' | '}' | '+' | '-' | '*' | '/' | ',' | '.' | ';'  => {
                    match Self::generate_single_token(ch) {
                        Ok(token) => tokens.push(token),
                        Err(err_message) => println!("{err_message}")
                    }
                }
                '!' | '=' | '>' | '<' => {
                    if input.peek().is_some_and(|&next| next == '=') {
                        match Self::generate_double_token(ch) {
                            Ok(token) => tokens.push(token),
                            Err(err_message) => println!("{err_message}")
                        }
                    } else {
                        match Self::generate_single_token(ch) {
                            Ok(token) => tokens.push(token),
                            Err(err_message) => println!("{err_message}")
                        }
                    } 
                }
                '\"' => {
                    let mut substr: Vec<char> = vec![];
                    let mut valid = false;
                    for next in input.by_ref() {
                        if next == ch {
                            tokens.push(Self::generate_literal_token(substr, Some(TokenKind::String)));
                            valid = true;
                            break;
                        }
                        substr.push(next);
                    }
                    if !valid {
                        return LexerError::err("reached EOF without closing string",line_number, column_number);
                    }
                }
                c if c.is_alphabetic() || c == '_' => {
                    let mut substr: Vec<char> = vec![];
                    substr.push(ch);
                    while input.peek().is_some_and(|&next| next.is_alphanumeric() || next == '_') {
                        substr.push(input.next().unwrap());
                    }
                    tokens.push(Self::generate_literal_token(substr, None));
                }
                c if c.is_numeric() => {
                    let mut substr: Vec<char> = vec![];
                    substr.push(ch);
                    // NOTE: There's probably a better way to do this than using a flag
                    let mut dot = false;
                    while let Some(ch) = input.peek() {
                        match ch {
                            '.' => {
                                if dot {
                                    return LexerError::err("numbers cannot have two '.'", line_number, column_number);
                                }
                                dot = true;
                                substr.push(input.next().unwrap());
                            }
                            '_' => { continue; }
                            c if c.is_numeric() => { substr.push(input.next().unwrap()); }
                            _ => { break; }
                        }
                    }
                    tokens.push(Self::generate_literal_token(substr, Some(TokenKind::Number)));
                }
                c if c.is_whitespace() => { continue; }
                _ => {
                    println!("hi :)");
                }
            }
        }                               
        Ok(tokens)
    }
    
    fn generate_single_token(input: char) -> Result<Token, String> {
        let token = match input {
            '(' => Token{kind: TokenKind::LeftParen, lexeme: input.to_string()},
            ')' => Token{kind: TokenKind::RightParen, lexeme: input.to_string()},
            '[' => Token{kind: TokenKind::LeftBrace, lexeme: input.to_string()},
            ']' => Token{kind: TokenKind::RightBrace, lexeme: input.to_string()},
            '{' => Token{kind: TokenKind::LeftCurly, lexeme: input.to_string()},
            '}' => Token{kind: TokenKind::RightCurly, lexeme: input.to_string()},
            '+' => Token{kind: TokenKind::Plus, lexeme: input.to_string()},
            '-' => Token{kind: TokenKind::Minus, lexeme: input.to_string()},
            '*' => Token{kind: TokenKind::Star, lexeme: input.to_string()},
            '/' => Token{kind: TokenKind::Slash, lexeme: input.to_string()},
            ',' => Token{kind: TokenKind::Comma, lexeme: input.to_string()},
            '.' => Token{kind: TokenKind::Dot, lexeme: input.to_string()},
            ';' => Token{kind: TokenKind::Semicolon, lexeme: input.to_string()},
            '!' => Token{kind: TokenKind::Bang, lexeme: input.to_string()},
            '=' => Token{kind: TokenKind::Equal, lexeme: input.to_string()},
            '>' => Token{kind: TokenKind::Greater, lexeme: input.to_string()},
            '<' => Token{kind: TokenKind::Less, lexeme: input.to_string()},
             _  => return Err("generate_single_token() called on invalid input".to_string())
        };
        Ok(token)
    }

    // This function is only called for the two character tokens: '!=', '==', '<=', '>='
    fn generate_double_token(input: char) -> Result<Token, String> {
        let token = match input {
            '!' => Token{kind: TokenKind::BangEqual, lexeme: "!=".to_string()},
            '=' => Token{kind: TokenKind::EqualEqual, lexeme: "==".to_string()},
            '<' => Token{kind: TokenKind::LessEqual, lexeme: "<=".to_string()},
            '>' => Token{kind: TokenKind::GreaterEqual, lexeme: ">=".to_string()},
             _  => return Err("generate_double_token() called on invalid input".to_string()),
        };
        Ok(token)
    }

    fn generate_literal_token(input: Vec<char>, kind: Option<TokenKind>) -> Token {
        let lexeme: String = input.iter().collect();
        if let Some(kind) = kind { return Token{kind, lexeme} }
        let kind = match lexeme.to_lowercase().as_str() {
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "func" => TokenKind::Func,
            "return" => TokenKind::Return,
            "let" => TokenKind::Let,
            "const" => TokenKind::Const,
            "struct" => TokenKind::Struct,
            _ => TokenKind::Identifier,
        };
        Token{kind, lexeme}
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
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBrace => "[",
            Self::RightBrace => "]",
            Self::LeftCurly => "{",
            Self::RightCurly => "}",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::Semicolon => ";",
            Self::Bang => "!",
            Self::BangEqual => "!=",
            Self::Equal => "=",
            Self::EqualEqual => "==",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Identifier => "IDENTIFIER",
            Self::String => "STRING",
            Self::Number => "NUMBER",
            Self::If => "if",
            Self::Else => "else",
            Self::For => "for",
            Self::While => "while",
            Self::True => "true",
            Self::False => "false",
            Self::Func => "fn",
            Self::Return => "return",
            Self::Let => "let",
            Self::Const => "const",
            Self::Struct => "struct",
        };

        write!(f, "{s}")
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s_kind = match self.kind {
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

        write!(f, "kind: {s_kind}, lexeme: {}", self.lexeme)
    }
}


#[derive(Debug)]
pub struct LexerError {
    message: String,
    line_number: usize,
    column_number: usize,
}

impl LexerError {
    fn new<S: Into<String>>(msg: S, line_number: usize, column_number: usize) -> Self {
        Self {
            message: msg.into(),
            line_number,
            column_number,
        }
    }
    fn err<Never, S: Into<String>>(msg: S, line_number: usize, column_number: usize) -> Result<Never, Self> {
        Err(Self::new(msg, line_number, column_number))
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.message, self.line_number, self.column_number)
    }
}

impl From<LexerError> for anyhow::Error {
    fn from(err: LexerError) -> Self {
        anyhow!("LEXER ERROR: {err}")
    }
}

#[cfg(test)]
mod tests;