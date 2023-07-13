#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Token<'a> {
    Identifier(&'a str),
    Integer(i64),

    Equals,
    Plus,
    Minus,
    Star,
    Slash,

    Bang,

    Comma,
    Semicolon,

    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    Function,
    Let,

    Illegal,
}

static KEYWORDS: phf::Map<&'static str, Token> = phf::phf_map! {
    "fn" => Token::Function,
    "let" => Token::Let,
};

struct Lexer<'a> {
    text: &'a str,
}

trait ValidIdentifier {
    fn is_valid_identifier(&self) -> bool;
}

impl ValidIdentifier for char {
    fn is_valid_identifier(&self) -> bool {
        self.is_alphabetic() || *self == '_'
    }
}

impl<'a> Lexer<'a> {
    fn new(text: &'a str) -> Self {
        Lexer { text }
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        // skip whitespace
        self.text = self.text.trim_start();

        // handle end of file
        let Some(character) = self.text.chars().nth(0) else {
            return None;
        };

        // special character tokens
        if let Some(token) = self.character_token(character) {
            (_, self.text) = self.text.split_at(1);
            return Some(token);
        }

        // identifiers
        if character.is_valid_identifier() {
            // TODO: see if this can be made better
            let mut index = 0;
            let mut chars = self.text.chars();
            while let Some(c) = chars.next() {
                if c.is_whitespace() || !c.is_valid_identifier() {
                    break;
                }

                index += 1;
            }

            let identifier;
            (identifier, self.text) = self.text.split_at(index);

            if let Some(keyword) = KEYWORDS.get(identifier) {
                return Some(keyword.clone());
            } else {
                return Some(Token::Identifier(identifier));
            }
        }

        // numbers
        if character.is_numeric() {
            // TODO: actually parse numbers
            (_, self.text) = self.text.split_at(1);
            return Some(Token::Integer(0));
        }

        // we dont know what the heck we got
        (_, self.text) = self.text.split_at(1);
        return Some(Token::Illegal);
    }

    fn character_token(&self, c: char) -> Option<Token<'a>> {
        match c {
            '!' => Some(Token::Bang),
            '=' => Some(Token::Equals),
            ';' => Some(Token::Semicolon),
            ',' => Some(Token::Comma),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            '(' => Some(Token::LeftParenthesis),
            ')' => Some(Token::RightParenthesis),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '/' => Some(Token::Slash),
            _ => {
                return None;
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn main() {
    let file = r#"
        let five = 5;
        let ten = 10;
        #

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
    "#;

    let lexer = Lexer::new(file);

    let tokens = lexer.collect::<Vec<_>>();
    dbg!(tokens);
}
