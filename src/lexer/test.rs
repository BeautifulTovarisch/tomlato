use super::{
    Grammar,
    tokenize,
    characterize,
    scan_identifier,
    valid_in_identifier,
    scan_string_literal
};

#[test]
fn test_characterize() {
    assert_eq!( characterize( '.' ), Grammar::Dot );
    assert_eq!( characterize( ' ' ), Grammar::Space );
    assert_eq!( characterize( '=' ), Grammar::Equals );
    assert_eq!( characterize( '{' ), Grammar::LeftBrace );
    assert_eq!( characterize( '}' ), Grammar::RightBrace );
    assert_eq!( characterize( '"' ), Grammar::DoubleQuote );
    assert_eq!( characterize( '\'' ), Grammar::SingleQuote );
    assert_eq!( characterize( '[' ), Grammar::LeftBracket );
    assert_eq!( characterize( ']' ), Grammar::RightBracket );
    assert_eq!( characterize( '\t' ), Grammar::Tab );
    assert_eq!( characterize( '\x08' ), Grammar::Backspace );
    assert_eq!( characterize( '\n' ), Grammar::LineFeed );
}

#[test]
fn test_valid_in_identifier() {
    assert!( valid_in_identifier( '_' ) );
    assert!( valid_in_identifier( '-' ) );

    assert!( valid_in_identifier( 'A' ) );
    assert!( valid_in_identifier( '藏' ) );
    assert!( valid_in_identifier( 'ڲ' ) );

    assert_eq!( valid_in_identifier( '"' ), false );
    assert_eq!( valid_in_identifier( '~' ), false );
    assert_eq!( valid_in_identifier( '`' ), false );
    assert_eq!( valid_in_identifier( '!' ), false );
    assert_eq!( valid_in_identifier( '@' ), false );
    assert_eq!( valid_in_identifier( '#' ), false );
    assert_eq!( valid_in_identifier( '$' ), false );
    assert_eq!( valid_in_identifier( '%' ), false );
    assert_eq!( valid_in_identifier( '^' ), false );
    assert_eq!( valid_in_identifier( '&' ), false );

    assert_eq!( valid_in_identifier( '\'' ), false );

    // You get the picture...
}

#[test]
fn test_scan_identifier() {
    let mut characters = "variable".chars();

    assert_eq!(
        scan_identifier( &mut characters ),
        vec![ Grammar::Identifier( String::from( "variable" ) ) ]
    );

    let mut characters = "some-variable ".chars();

    assert_eq!(
        scan_identifier( &mut characters ),
        vec![ Grammar::Identifier( String::from( "some-variable" ) ), Grammar::Space ]
    );

    let mut characters = "third_variable.".chars();

    assert_eq!(
        scan_identifier( &mut characters ),
        vec![ Grammar::Identifier( String::from( "third_variable" ) ), Grammar::Dot ]
    );

    let mut characters = "nested.variable".chars();

    assert_eq!(
        scan_identifier( &mut characters ),
        vec![ Grammar::Identifier( String::from( "nested" ) ), Grammar::Dot ]
    );

    assert_eq!(
        scan_identifier( &mut characters ),
        vec![ Grammar::Identifier( String::from( "variable" ) ) ]
    );
}

#[test]
fn test_scan_string_literal() {
    let mut characters = "\"Some String Literal\"".chars();

    assert_eq!(
        scan_string_literal( '"', &mut characters ),
        Grammar::StringLiteral( String::from( "Some String Literal" ) )
    );

    let mut characters = r#"String literal containing comment - #"#.chars();

    assert_eq!(
        scan_string_literal( '"', &mut characters ),
        Grammar::StringLiteral( String::from( "String literal containing comment - #" ) )
    );


    // Separate groups of StringLiterals between " or '
    // e.g [ Grammar::DoubleQuote, StringLiteral, StringLiteral, Grammar::DoubleQuote ]
    let mut characters = r#" And when \"'s are in the string, along with # \"#.chars();

    assert_eq!(
        scan_string_literal( '"', &mut characters ),
        Grammar::StringLiteral( String::from( " And when \"'s are in the string, along with # \"" ) )
    );
}

#[test]
fn test_tokenize() {
    use super::Grammar::*;

    let toml = "TOML . ={}\"'\
[]\t\x08\x0C\\abc another_identifier also-valid";

    let expected = vec![
        Identifier( String::from( "TOML" ) ),
        Space,
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
        Space,
        Identifier( String::from( "another_identifier" ) ),
        Space,
        Identifier( String::from( "also-valid" ) ),
        EOF
    ];

    assert_eq!( tokenize( toml ), expected );
}
