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
            ( Grammar::EOF, Grammar::EOF ) => true,
            ( _, _ ) => false
        }
    }
}

fn identify( input: Vec<char> ) -> Grammar {
    Grammar::Identifier( input.into_iter().collect() )
}

fn scan_identifier<'a, I>( characters: &mut I ) -> Grammar
    where I: Iterator<Item = char>
{
    identify(
        characters
            .take_while( |c| { !c.is_whitespace() } )
            .collect()
    )
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
        if c.is_alphanumeric() {
            tokens.push( scan_identifier( &mut characters ) );

        } else {
            tokens.push( characterize( *c ) );
        }
        characters.next();
    }

    tokens.push( Grammar::EOF );

    tokens
}

#[cfg(test)]
mod test;
