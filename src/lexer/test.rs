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
fn test_identifier() {
    assert_eq!( identify( vec![ 'k', 'e', 'y' ] ), Grammar::Identifier( String::from( "key" ) ) )
}

#[test]
fn test_line_feed() {
    assert_eq!( characterize( '\n' ), Grammar::LineFeed )
}

#[test]
fn test_tokenize() {
    use super::Grammar::*;

    let toml = ". ={}\"'\
[]\t\x08\x0C\\abc";

    let expected = vec![
        Dot,
        Space,
        Equals,
        LeftBrace,
        RightBrace,
        DoubleQuote,
        SingleQuote,
        LeftBracket,
        RightBracket,
        Tab,
        Backspace,
        FormFeed,
        Backslash,
        Identifier( String::from( "abc" ) ),
        EOF
    ];

    assert_eq!( tokenize( toml ), expected );
}
