/// Takes program text as input and tokenizes it.
pub fn lex(input: &str) -> Vec<Token> {
    todo!("Lexer has not yet been written.")
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

#[cfg(test)]
mod tests;