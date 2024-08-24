use std::{iter::Peekable, str::Chars, fmt};

/// Example
/// GET /api/user/:id 200
/// Delay=1
///
/// {
///   "foo": true
/// }
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Identifier(String),
    Equal,
    LeftBrace,
    RightBrace,
    Quote,
    Colon,
    Comma,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(ident) => write!(f, "{}", ident),
            Token::Equal => write!(f, "="),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Quote => write!(f, "\""),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
        }
    }
}

pub fn parse(text: String) -> Vec<Token> {
    let mut peeks = text.chars().peekable();
    let mut tokens: Vec<Token> = vec![];

    while let Some(&symbol) = peeks.peek() {
        match symbol {
            symbol if is_valid_string(symbol) => {
                let content = read_string(&mut peeks);
                if symbol == ':' {
                    consume(&mut tokens, Token::Colon, &mut peeks, false);
                } else {
                    consume(&mut tokens, Token::Identifier(content), &mut peeks, false);
                }
            }
            symbol if symbol.is_whitespace() => {
                peeks.next();
            }
            '=' => {
                consume(&mut tokens, Token::Equal, &mut peeks, true);
            }
            '{' => {
                consume(&mut tokens, Token::LeftBrace, &mut peeks, true);
            }
            '}' => {
                consume(&mut tokens, Token::RightBrace, &mut peeks, true);
            }
            '"' => {
                consume(&mut tokens, Token::Quote, &mut peeks, true);
                let content = read_quoted_string(&mut peeks);
                consume(&mut tokens, Token::Identifier(content), &mut peeks, false);
                consume(&mut tokens, Token::Quote, &mut peeks, true);
            }
            ',' => {
                consume(&mut tokens, Token::Comma, &mut peeks, true);
            }
            ':' => {
                consume(&mut tokens, Token::Colon, &mut peeks, true);
            }
            _ => {
                dbg!(tokens);
                panic!("Invalid token -> [{symbol}]");
            }
        }
    }

    tokens
}

fn is_valid_string(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '/' || ch == ':'
}

fn read_string(peeks: &mut Peekable<Chars<'_>>) -> String {
    let mut string = String::new();

    while let Some(&ch) = peeks.peek() {
        if is_valid_string(ch) {
            string.push(ch);
            peeks.next();
        } else {
            break;
        }
    }

    return string;
}

fn read_quoted_string(peeks: &mut Peekable<Chars<'_>>) -> String {
    let mut string = String::new();

    while let Some(&ch) = peeks.peek() {
        if ch == '"' {
            break;
        }
        string.push(ch);
        peeks.next();
    }

    return string;
}

fn consume(tokens: &mut Vec<Token>, token: Token, peeks: &mut Peekable<Chars<'_>>, should_peek: bool) {
    tokens.push(token);


    if should_peek {
        peeks.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let example = "
        GET /api/user/:id 200
        Delay=1

        {
          \"foo\": true
        }
        "
        .to_string();

        assert_eq!(
            parse(example),
            vec![
                Token::Identifier("GET".to_string()),
                Token::Identifier("/api/user/:id".to_string()),
                Token::Identifier("200".to_string()),
                Token::Identifier("Delay".to_string()),
                Token::Equal,
                Token::Identifier("1".to_string()),
                Token::LeftBrace,
                Token::Quote,
                Token::Identifier("foo".to_string()),
                Token::Quote,
                Token::Colon,
                Token::Identifier("true".to_string()),
                Token::RightBrace
            ]
        );
    }

    #[test]
    fn parse_example_with_two_fields_without_delay() {
        let example = "
        PUT /foo 416

        {
          \"foo\": true,
          \"bar\": \"Hello World\"
        }
        "
        .to_string();

        assert_eq!(
            parse(example),
            vec![
                Token::Identifier("PUT".to_string()),
                Token::Identifier("/foo".to_string()),
                Token::Identifier("416".to_string()),
                Token::LeftBrace,
                Token::Quote,
                Token::Identifier("foo".to_string()),
                Token::Quote,
                Token::Colon,
                Token::Identifier("true".to_string()),
                Token::Comma,
                Token::Quote,
                Token::Identifier("bar".to_string()),
                Token::Quote,
                Token::Colon,
                Token::Quote,
                Token::Identifier("Hello World".to_string()),
                Token::Quote,
                Token::RightBrace
            ]
        );
    }

    #[test]
    fn parse_json_like_structure() {
        let example = r#"
        {
          "foo": "Hello world!"
        }
        "#
        .to_string();

        assert_eq!(
            parse(example),
            vec![
                Token::LeftBrace,
                Token::Quote,
                Token::Identifier("foo".to_string()),
                Token::Quote,
                Token::Colon,
                Token::Quote,
                Token::Identifier("Hello world!".to_string()),
                Token::Quote,
                Token::RightBrace
            ]
        );
    }
}