use super::*;

///=================== Lexer test helpers. ===================
macro_rules! validate_tokens {
    ($program:expr, $($expected_lexeme:expr),+) => {
        let mut tokens = Lexer::lex($program)
            .expect(&format!("Internal error: Could not lex the given program: {}.", $program))
            .into_iter();
        $(
            let token = tokens.next().expect(&format!( 
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
    validate_tokens!(
        "let x = 1 + 1;", 

        "let", "x", "=", "1", "+", "1", ";"
    );
}

#[test]
/// Tests that a program involving strings can be lexed properly.
fn lex_string_program() {
    validate_tokens!(
        r#"let my_string = "Hello World";"#, 
    
        "let", "my_string", "=", "Hello World", ";"
    );
}

#[test]
/// Tests that a program involving numbers can be lexed properly.
fn lex_number_program() {
    validate_tokens!(
        "const PI = 3.14;", 

        "const", "PI", "=", "3.14", ";"
    );
}

#[test]
fn lex_multi_lines() {
    validate_tokens!(r#"
        let x = 1;
        const y = 2.32;
        let _z = "starting underscore";
    "#, 
    
    "let", "x", "=", "1", ";",
    "const", "y", "=", "2.32", ";",
    "let", "_z", "=", "starting underscore", ";"
    );
}

#[test]
fn lex_structure() {
    validate_tokens!(r#"
        struct Point2D {
           let x = 3;
           const y = 2;
        };
    "#, 
    
    "struct", "Point2D", "{",
        "let", "x", "=", "3", ";",
        "const", "y", "=", "2", ";",
    "}", ";"
    );
}

#[test]
fn error_on_unterminated_string() {
    match Lexer::lex("let bad_string = \"I didn't end my string! Oops.") {
        Ok(_) => panic!("did not error as expected"),
        Err(e) => assert_eq!(e.message, "error: reached EOF without closing string"),
    }
}

#[test]
fn error_with_two_decimals() {
    match Lexer::lex("let bad_number = 1.275.508") {
        Ok(_) => panic!("did not error as expected"),
        Err(e) => assert_eq!(e.message, "error: numbers cannot have two '.'"),
    }
}