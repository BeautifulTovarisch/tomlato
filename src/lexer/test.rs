use super::{ Token, Grammar, tokenize };

#[test]
fn test_dot() {
    assert_eq!( tokenize( '.' ), Grammar::Dot )
}

#[test]
fn test_space() {
    assert_eq!( tokenize( ' ' ), Grammar::Space )
}
