use super::*;

#[test]
fn tokenkind_tostring() {
    assert!(TokenKind::Equal.to_string() == "=");
    assert!(TokenKind::LeftCurly.to_string() == "{");
    assert!(TokenKind::EqualEqual.to_string() == "==");
}