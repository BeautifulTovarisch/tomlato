#[derive(Debug)]
pub enum Grammar {
    // General Grammar
    Dot,
    Space,
    Equals,
    LeftBrace,
    RightBrace,
    DoubleQuote,
    SingleQuote,
    LeftBracket,
    RightBracket,

    // Escape characters
    Tab,
    LineFeed,
    FormFeed,
    Backspace,
    Backslash,

    StringLiteral( String ),
    NumberLiteral,
    Identifier( String ),
    EOF
}

// All we care about is the particular Grammar so this will satisfy PartialEq
impl PartialEq for Grammar {
    fn eq( &self, other: &Grammar ) -> bool {

        match( &self, other ) {
            ( Grammar::Dot, Grammar::Dot ) => true,
            ( Grammar::Space, Grammar::Space ) => true,
            ( Grammar::Equals, Grammar::Equals ) => true,
            ( Grammar::LeftBrace, Grammar::LeftBrace ) => true,
            ( Grammar::RightBrace, Grammar::RightBrace ) => true,
            ( Grammar::DoubleQuote, Grammar::DoubleQuote ) => true,
            ( Grammar::SingleQuote, Grammar::SingleQuote ) => true,
            ( Grammar::LeftBracket, Grammar::LeftBracket ) => true,
            ( Grammar::RightBracket, Grammar::RightBracket ) => true,
            ( Grammar::Tab, Grammar::Tab ) => true,
            ( Grammar::LineFeed, Grammar::LineFeed ) => true,
            ( Grammar::FormFeed, Grammar::FormFeed ) => true,
            ( Grammar::Backspace, Grammar::Backspace ) => true,
            ( Grammar::Backslash, Grammar::Backslash ) => true,
            ( Grammar::Identifier( ref id ), Grammar::Identifier( ref id2 ) ) => *id == *id2,
            ( Grammar::StringLiteral( ref str1 ), Grammar::StringLiteral( ref str2 ) ) =>
                *str1 == *str2,
            ( Grammar::EOF, Grammar::EOF ) => true,
            ( _, _ ) => false
        }
    }
}

fn valid_in_identifier( character: char ) -> bool {
    match character {
        '-' => true,
        '_' => true,
        c if c.is_alphanumeric() => true,
        _ => false
    }
}

fn scan_identifier<I>( characters: &mut I ) -> Vec<Grammar>
    where I: Iterator<Item = char>
{
    let mut tokens: Vec<Grammar> = Vec::with_capacity( 2 );

    let identifier: Vec<char> = characters
        .take_while( |c| {
            if !valid_in_identifier( *c ) {
                tokens.push( characterize( *c ) )
            }
           valid_in_identifier( *c )
        }).collect();

    let identifier: Grammar = Grammar::Identifier( identifier.into_iter().collect() );

    tokens.insert( 0, identifier );

    tokens
}

// We pass in the beginning quote ( ' or " ) to serve as the stopping point
fn scan_string_literal<I>( end_char: char, characters: &mut I ) -> Grammar
    where I: Iterator<Item = char>
{

    let literal: Vec<char> = characters
        .enumerate()
        .take_while( | ( index, character  ) | {
            *character != end_char || *index == 0
        })
        .map( | ( _, character ) | character )
        .filter( | character | *character != end_char )
        .collect();

    Grammar::StringLiteral( "".to_string() )
}

// Single character tokenization
fn characterize( input: char ) -> Grammar {
    match input {
        '.' => Grammar::Dot,
        ' ' => Grammar::Space,
        '=' => Grammar::Equals,
        '{' => Grammar::LeftBrace,
        '}' => Grammar::RightBrace,
        '"' => Grammar::DoubleQuote,
        '\'' => Grammar::SingleQuote,
        '[' => Grammar::LeftBracket,
        ']' => Grammar::RightBracket,
        '\\' => Grammar::Backslash,
        '\x0A' => Grammar::LineFeed,
        '\x0C' => Grammar::FormFeed,
        '\x08' => Grammar::Backspace,
        '\x09' => Grammar::Tab,
        _ => panic!( "Invalid Token \'{}\'", input )
    }
}

pub fn tokenize( toml: &str ) -> Vec<Grammar> {
    let mut tokens: Vec<Grammar> = Vec::new();

    let mut characters = toml.chars().peekable();

    while let Some( c ) = characters.peek() {
        if valid_in_identifier( *c ) {
            tokens.append( &mut scan_identifier( &mut characters ) );

        } else if *c == '"' || *c == '\'' {
            tokens.push( characterize( *c ) );
            characters.next();

        } else {
            tokens.push( characterize( *c ) );
            characters.next();
        }
    }

    tokens.push( Grammar::EOF );

    tokens
}

#[cfg(test)]
mod test;
