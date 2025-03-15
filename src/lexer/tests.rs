use super::*;

///=================== Lexer test helpers. ===================
macro_rules! validate_tokens {
    ($tokens:expr, $($expected_lexeme:expr),+) => {
        $(
            let token = $tokens.next().expect(&format!( 
                    "Expected token with lexeme \"{}\", got end of file.", $expected_lexeme));

            // Check that the lexeme is what we expect
            assert_eq!(token.lexeme, $expected_lexeme, 
                "Token's actual lexeme differed from expected lexeme");
        )+
    };
}


///=================== Begin lexer unit tests. ===================
 
#[test]
/// Tests that TokenKind's display impl works as we expect in the basic case.
fn tokenkind_to_string() {
    assert!(TokenKind::Equal.to_string() == "=");
    assert!(TokenKind::LeftCurly.to_string() == "{");
    assert!(TokenKind::EqualEqual.to_string() == "==");
}

#[test]
/// Tests that a basic program can be lexed properly.
fn lex_basic_program() {
    let mut tokens = lex("let x = 1 + 1;").into_iter();

    validate_tokens!(tokens, "let", "x", "=", "1", "+", "1", ";");
}

#[test]
/// Tests that a program involving strings can be lexed properly.
fn lex_string_program() {
    let mut tokens = lex("let my_string = \"Hello World\";").into_iter();

    validate_tokens!(tokens, "let", "my_string", "=", "\"Hello World\"", ";");
}

#[test]
/// Tests that a program involving numbers can be lexed properly.
fn lex_number_program() {
    let mut tokens = lex("const PI = 3.14;").into_iter();

    validate_tokens!(tokens, "const", "PI", "=", "3.14", ";");
}

#[test]
#[should_panic(expected = "bad ident")]
fn error_on_bad_ident_1() {
    let mut tokens = lex("let 41 = \"Bad\"");
}

#[test]
#[should_panic(expected = "bad ident")]
fn error_on_bad_ident_2() {
    let mut tokens = lex("let 💀 = \"Bad\"");
}

#[test]
fn lex_multi_lines() {
    let mut tokens = lex(r#"
        let x = 1;
        const y = 2.32;
        let _z = "starting underscore";
    "#).into_iter();

    validate_tokens!(tokens, "let", "x", "=", "1", ";");
    validate_tokens!(tokens, "const", "y", "=", "2.32", ";");
    validate_tokens!(tokens, "let", "_z", "=", "\"starting underscore\"", ";");
}

#[test]
fn lex_structure() {
    let mut tokens = lex(r#"
        struct Point2D {
           let x = 3;
           const y = 2;
        };
    "#).into_iter();

    validate_tokens!(tokens, "struct", "Point2D", "{");
    validate_tokens!(tokens, "let", "x", "=", "3", ";");
    validate_tokens!(tokens, "const", "y", "=", "2", ";");
    validate_tokens!(tokens, "}", ";");
}