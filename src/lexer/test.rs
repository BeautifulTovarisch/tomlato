use super::{ Grammar, tokenize, characterize, identify };

#[test]
fn test_dot() {
    assert_eq!( characterize( '.' ), Grammar::Dot )
}

#[test]
fn test_space() {
    assert_eq!( characterize( ' ' ), Grammar::Space )
}

#[test]
fn test_equals() {
    assert_eq!( characterize( '=' ), Grammar::Equals )
}

#[test]
fn test_left_brace() {
    assert_eq!( characterize( '{' ), Grammar::LeftBrace )
}

#[test]
fn test_right_brace() {
    assert_eq!( characterize( '}' ), Grammar::RightBrace )
}

#[test]
fn test_double_quote() {
    assert_eq!( characterize( '"' ), Grammar::DoubleQuote )
}

#[test]
fn test_single_quote() {
    assert_eq!( characterize( '\'' ), Grammar::SingleQuote )
}

#[test]
fn test_left_bracket() {
    assert_eq!( characterize( '[' ), Grammar::LeftBracket )
}

#[test]
fn test_right_bracket() {
    assert_eq!( characterize( ']' ), Grammar::RightBracket )
}

#[test]
fn test_tab() {
    assert_eq!( characterize( '\t' ), Grammar::Tab )
}

#[test]
fn test_backspace() {
    assert_eq!( characterize( '\x08' ), Grammar::Backspace )
}

#[test]
fn character() {
    assert_eq!( characterize( 't' ), Grammar::Character( 't' ) );
}

#[test]
fn identifier() {
    assert_eq!( identify( vec![ 'k', 'e', 'y' ] ), Grammar::Identifier( String::from( "key" ) ) )
}

#[test]
fn test_line_feed() {
    assert_eq!( characterize( '\n' ), Grammar::LineFeed )
}

#[test]
fn test_tokenize() {
    use super::Grammar::*;
    
    let toml = "title = \"TOML Example\"";

    let expected = vec![
        Character( 't' ),
        Character( 'i' ),
        Character( 't' ),
        Character( 'l' ),
        Character( 'e' ),
        Space,
        Equals,
        Space,
        DoubleQuote,
        Character( 'T' ),
        Character( 'O' ),
        Character( 'M' ),
        Character( 'L' ),
        Space,
        Character( 'E' ),
        Character( 'x' ),
        Character( 'a' ),
        Character( 'm' ),
        Character( 'p' ),
        Character( 'l' ),
        Character( 'e' ),
        DoubleQuote
    ];

    assert_eq!( expected, tokenize( toml ) );
}
