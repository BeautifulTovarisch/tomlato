#[derive(Debug)]
enum Grammar {
    // General Grammar
    Dot,
    Space,
    Equals,
    NewLine,
    EndOfFile,
    LeftBrace,
    Identifier( char ),
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
    Backslash

    // Values
       
}

#[derive(Debug)]
struct Token { character: Grammar }

impl PartialEq<Grammar> for Token {
    fn eq( &self, other: &Grammar ) -> bool {
        match( &self.character, other ) {
            ( &Grammar::Dot, &Grammar::Dot ) => true,
            ( &Grammar::Space, &Grammar::Space ) => true,
            ( &Grammar::Equals, &Grammar::Equals ) => true,
            ( &Grammar::NewLine, &Grammar::NewLine ) => true,
            ( &Grammar::EndOfFile, &Grammar::EndOfFile ) => true,
            ( &Grammar::LeftBrace, &Grammar::LeftBrace ) => true,
            ( &Grammar::Identifier( c1 ), &Grammar::Identifier( c2 ) ) => c1 == c2,
            ( _, _ ) => false
        }
    }
}

impl PartialEq for Token {
    fn eq( &self, other: &Token ) -> bool {
        self == other
    }
}

fn tokenize( input: char ) -> Token {
    match input {
        '.' => Token { character: Grammar::Dot },
        ' ' => Token { character: Grammar::Space },
        _ => panic!( "Invalid Token -> {} <-", input )
    }
} 

#[cfg(test)]
mod test;
