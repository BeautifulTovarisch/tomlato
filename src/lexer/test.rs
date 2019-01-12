use super::{ Token, Grammar, tokenize };

#[test]
fn test_dot() {
    assert_eq!( tokenize( '.' ), Grammar::Dot )
}

#[test]
fn test_space() {
    assert_eq!( tokenize( ' ' ), Grammar::Space )
}

#[test]
fn test_equals() {
    assert_eq!( tokenize( '=' ), Grammar::Equals )
}

#[test]
fn test_left_brace() {
    assert_eq!( tokenize( '{' ), Grammar::LeftBrace )
}

#[test]
fn test_right_brace() {
    assert_eq!( tokenize( '}' ), Grammar::RightBrace )
}

#[test]
fn test_double_quote() {
    assert_eq!( tokenize( '"' ), Grammar::DoubleQuote )
}

#[test]
fn test_single_quote() {
    assert_eq!( tokenize( '\'' ), Grammar::SingleQuote )
}

#[test]
fn test_left_bracket() {
    assert_eq!( tokenize( '[' ), Grammar::LeftBracket )
}

#[test]
fn test_right_bracket() {
    assert_eq!( tokenize( ']' ), Grammar::RightBracket )
}

#[test]
fn test_tab() {
    assert_eq!( tokenize( '\t' ), Grammar::Tab )
}

fn test_backspace() {
    assert_eq!( tokenize( '\x08' ), Grammar::Backspace )
}

#[test]
fn test_char_literal() {

}
// #[test]
// #[test]
// #[test]
// #[test]
// #[test]
// #[test]
// #[test]
// #[test]

#[test]
fn test_line_feed() {
    assert_eq!( tokenize( '\n' ), Grammar::LineFeed )
}
