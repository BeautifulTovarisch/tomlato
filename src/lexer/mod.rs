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
    Quote,
    LineFeed,
    FormFeed,
    Backspace,
    Backslash,

    Character( char ),
    Identifier( String )
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
            ( Grammar::Quote, Grammar::Quote ) => true,
            ( Grammar::LineFeed, Grammar::LineFeed ) => true,
            ( Grammar::FormFeed, Grammar::FormFeed ) => true,
            ( Grammar::Backspace, Grammar::Backspace ) => true,
            ( Grammar::Backslash, Grammar::Backslash ) => true,
            ( Grammar::Character( c1 ), Grammar::Character( c2 ) ) => c1 == c2,
            ( Grammar::Identifier( ref id ), Grammar::Identifier( ref id2 ) ) => *id == *id2,
            ( _, _ ) => false
        }
    }
}

fn identify( input: Vec<char> ) -> Grammar {
    Grammar::Identifier( input.into_iter().collect() )
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
        '\n' => Grammar::LineFeed,
        '[' => Grammar::LeftBracket,
        ']' => Grammar::RightBracket,
        '\t' => Grammar::Tab,
        '\x08' => Grammar::Backspace,
        'a'...'z' => Grammar::Character( input ),
        'A'...'Z' => Grammar::Character( input ),
        _ => panic!( "Invalid Token \'{}\'", input )
    }
}

pub fn tokenize( toml: &str ) -> Vec<Grammar> {
    toml.chars().enumerate().map( | ( i, c ) | {
        characterize( c )
    }).collect()
}

#[cfg(test)]
mod test;
