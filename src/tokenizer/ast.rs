use crate::app_requirements::{ApplicationRequirements, HttpMethods};
use super::tokens::Token;
use thiserror::Error;
use std::str::FromStr;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Missing HTTP method")]
    MissingHttpMethod,
    #[error("Invalid HTTP method: {0}")]
    InvalidHttpMethod(String),
    #[error("Missing path")]
    MissingPath,
    #[error("Missing HTTP status code")]
    MissingStatusCode,
    #[error("Invalid HTTP status code: {0}")]
    InvalidStatusCode(String),
    #[error("Invalid delay value: {0}")]
    InvalidDelay(String),
}

pub fn parse_requirements(tokens: &[Token]) -> Result<Vec<ApplicationRequirements>, ParseError> {
    let mut requirements = Vec::new();
    let mut iter = tokens.iter().peekable();

    while iter.peek().is_some() {
        let mut current_tokens = Vec::new();

        // Collect tokens for the current requirement
        while let Some(token) = iter.next() {
            current_tokens.push(token);
            if matches!(token, Token::RightBrace) {
                break;
            }
        }

        // Parse the current requirement
        let requirement = parse_single_requirement(&current_tokens)?;
        requirements.push(requirement);
    }

    Ok(requirements)
}

fn parse_single_requirement(tokens: &Vec<&Token>) -> Result<ApplicationRequirements, ParseError> {
    let mut iter = tokens.iter();

    let http_method = match iter.next() {
        Some(Token::Identifier(method)) => HttpMethods::from_str(method)
            .map_err(|_| ParseError::InvalidHttpMethod(method.clone()))?,
        _ => return Err(ParseError::MissingHttpMethod),
    };

    let path = match iter.next() {
        Some(Token::Identifier(path)) => path.clone(),
        _ => return Err(ParseError::MissingPath),
    };

    let http_response_status = match iter.next() {
        Some(Token::Identifier(status)) => status.parse()
            .map_err(|_| ParseError::InvalidStatusCode(status.clone()))?,
        _ => return Err(ParseError::MissingStatusCode),
    };

    let mut delay = None;
    let mut http_response_body = None;

    while let Some(token) = iter.next() {
        match token {
            Token::Identifier(ident) if ident == "Delay" => {
                if let Some(Token::Equal) = iter.next() {
                    if let Some(Token::Identifier(delay_value)) = iter.next() {
                        delay = Some(delay_value.parse()
                            .map_err(|_| ParseError::InvalidDelay(delay_value.clone()))?);
                    }
                }
            }
            Token::LeftBrace => {
                let mut json_content = String::from("{");
                while let Some(token) = iter.next() {
                    if matches!(token, Token::RightBrace) {
                        json_content.push('}');
                        break;
                    }
                    json_content.push_str(&token.to_string());
                }
                http_response_body = Some(json_content);
            }
            _ => {}
        }
    }

    Ok(ApplicationRequirements {
        path,
        http_method,
        http_response_status,
        http_response_body,
        http_response_path: None,
        delay,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_requirements::HttpMethods;

    #[test]
    fn parse_from_tokens_to_requirements() {
        let tokens = vec![
            Token::Identifier("GET".to_string()),
            Token::Identifier("/api/user/:id".to_string()),
            Token::Identifier("200".to_string()),
            Token::Identifier("Delay".to_string()),
            Token::Equal,
            Token::Identifier("1".to_string()),
        ];
        let result = parse_requirements(&tokens).unwrap();
        assert_eq!(
            result,
            vec![ApplicationRequirements {
                path: "/api/user/:id".to_string(),
                http_method: HttpMethods::GET,
                http_response_status: 200,
                http_response_body: None,
                http_response_path: None,
                delay: Some(1)
            }]
        );
    }

    #[test]
    fn parse_with_invalid_http_method() {
        let tokens = vec![
            Token::Identifier("INVALID".to_string()),
            Token::Identifier("/api/user/:id".to_string()),
            Token::Identifier("200".to_string()),
        ];
        let result = parse_requirements(&tokens);
        assert!(matches!(result, Err(ParseError::InvalidHttpMethod(_))));
    }

    #[test]
    fn parse_with_missing_path() {
        let tokens = vec![
            Token::Identifier("GET".to_string()),
        ];
        let result = parse_requirements(&tokens);
        assert!(matches!(result, Err(ParseError::MissingPath)));
    }

    #[test]
    fn parse_with_invalid_status_code() {
        let tokens = vec![
            Token::Identifier("GET".to_string()),
            Token::Identifier("/api/user/:id".to_string()),
            Token::Identifier("invalid".to_string()),
        ];
        let result = parse_requirements(&tokens);
        assert!(matches!(result, Err(ParseError::InvalidStatusCode(_))));
    }

    #[test]
    fn parse_with_http_response_body() {
        let tokens = vec![
            Token::Identifier("POST".to_string()),
            Token::Identifier("/api/user".to_string()),
            Token::Identifier("201".to_string()),
            Token::LeftBrace,
            Token::Quote,
            Token::Identifier("id".to_string()),
            Token::Quote,
            Token::Colon,
            Token::Identifier("123".to_string()),
            Token::Comma,
            Token::Quote,
            Token::Identifier("name".to_string()),
            Token::Quote,
            Token::Colon,
            Token::Quote,
            Token::Identifier("John Doe".to_string()),
            Token::Quote,
            Token::Comma,
            Token::Quote,
            Token::Identifier("active".to_string()),
            Token::Quote,
            Token::Colon,
            Token::Identifier("true".to_string()),
            Token::RightBrace,
        ];
        let result = parse_requirements(&tokens).unwrap();
        assert_eq!(
            result,
            vec![ApplicationRequirements {
                path: "/api/user".to_string(),
                http_method: HttpMethods::POST,
                http_response_status: 201,
                http_response_body: Some(r#"{"id":123,"name":"John Doe","active":true}"#.to_string()),
                http_response_path: None,
                delay: None
            }]
        );
    }
}