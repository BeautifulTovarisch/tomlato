#[derive(Debug)]
enum Grammar {
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

    Identifier( String )
}

#[derive(Debug)]
pub struct Token { character: Grammar }

// All we care about is the particular Grammar so this will satisfy PartialEq
impl PartialEq<Grammar> for Token {
    fn eq( &self, other: &Grammar ) -> bool {

        match( &self.character, other ) {
            ( &Grammar::Dot, &Grammar::Dot ) => true,
            ( &Grammar::Space, &Grammar::Space ) => true,
            ( &Grammar::Equals, &Grammar::Equals ) => true,
            ( &Grammar::LeftBrace, &Grammar::LeftBrace ) => true,
            ( &Grammar::RightBrace, &Grammar::RightBrace ) => true,
            ( &Grammar::DoubleQuote, &Grammar::DoubleQuote ) => true,
            ( &Grammar::SingleQuote, &Grammar::SingleQuote ) => true,
            ( &Grammar::LeftBracket, &Grammar::LeftBracket ) => true,
            ( &Grammar::RightBracket, &Grammar::RightBracket ) => true,
            ( &Grammar::Tab, &Grammar::Tab ) => true,
            ( &Grammar::Quote, &Grammar::Quote ) => true,
            ( &Grammar::LineFeed, &Grammar::LineFeed ) => true,
            ( &Grammar::FormFeed, &Grammar::FormFeed ) => true,
            ( &Grammar::Backspace, &Grammar::Backspace ) => true,
            ( &Grammar::Backslash, &Grammar::Backslash ) => true,
            ( &Grammar::Identifier( ref id ), &Grammar::Identifier( ref id2 ) ) => *id == *id2,
            ( _, _ ) => false
        }
    }
}

pub fn identify( input: Vec<char> ) -> Token {
    Token { character: Grammar::Identifier( input.into_iter().collect() ) }
}

// Single character tokenization

pub fn tokenize( input: char ) -> Token {
    match input {
        '.' => Token { character: Grammar::Dot },
        ' ' => Token { character: Grammar::Space },
        '=' => Token { character: Grammar::Equals },
        '{' => Token { character: Grammar::LeftBrace },
        '}' => Token { character: Grammar::RightBrace },
        '"' => Token { character: Grammar::DoubleQuote },
        '\'' => Token { character: Grammar::SingleQuote },
        '\n' => Token { character: Grammar::LineFeed },
        '[' => Token { character: Grammar::LeftBracket },
        ']' => Token { character: Grammar::RightBracket },
        '\t' => Token { character: Grammar::Tab },
        '\x08' => Token { character: Grammar::Backspace },
        _ => panic!( "Invalid Token \'{}\'", input )
    }
}

#[cfg(test)]
mod test;
