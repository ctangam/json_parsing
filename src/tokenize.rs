use std::num::ParseFloatError;

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,

    Null,
    False,
    True,

    Number(f64),
    String(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizeError {
    UnfinishedLiteralValue,
    ParseNumberError(ParseFloatError),
    UnclosedQuotes,
    UnexpectedEof,
    CharNotRecognized(char),
}

pub fn tokenize(input: String) -> Result<Vec<Token>, TokenizeError> {
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;

    let mut tokens = Vec::new();

    while index < chars.len() {
        let token = make_token(&chars, &mut index)?;
        tokens.push(token);
        index += 1;
    }

    Ok(tokens)
}

fn make_token(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    let mut ch = chars[*index];

    while ch.is_ascii_whitespace() {
        *index += 1;
        if *index >= chars.len() {
            return Err(TokenizeError::UnexpectedEof);
        }
        ch = chars[*index];
    }

    let token = match ch {
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        '[' => Token::LeftBracket,
        ']' => Token::RightBracket,
        ',' => Token::Comma,
        ':' => Token::Colon,

        'n' => tokenize_null(chars, index)?,
        'f' => tokenize_false(chars, index)?,
        't' => tokenize_true(chars, index)?,

        c if c.is_ascii_digit() || c == '-' => tokenize_float(chars, index)?,

        '"' => tokenize_string(chars, index)?,

        ch => return Err(TokenizeError::CharNotRecognized(ch)),
    };

    Ok(token)
}

fn tokenize_null(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "null".chars() {
        if chars[*index] != expected_char {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }

    *index -= 1;
    Ok(Token::Null)
}

fn tokenize_false(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "false".chars() {
        if chars[*index] != expected_char {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }

    *index -= 1;
    Ok(Token::False)
}

fn tokenize_true(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "true".chars() {
        if chars[*index] != expected_char {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }

    *index -= 1;
    Ok(Token::True)
}

fn tokenize_float(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    let mut unparsed_num = String::new();
    let mut has_decimal = false;
    let mut has_exp = false;
    let mut has_sign_after_exp = false;

    if chars[*index] == '-' {
        unparsed_num.push('-');
        *index += 1;
    }

    while *index < chars.len() {
        let ch = chars[*index];
        match ch {
            c if c.is_ascii_digit() => unparsed_num.push(c),
            c if c == '.' && !has_decimal => {
                unparsed_num.push('.');
                has_decimal = true;
            }

            c if c == 'e' || c == 'E' && !has_exp => {
                unparsed_num.push(c);
                has_exp = true;
            }
            c if c == '-' || c == '+' && has_exp && !has_sign_after_exp => {
                unparsed_num.push(c);
                has_sign_after_exp = true;
            }
            _ => break,
        }

        *index += 1;
    }

    unparsed_num
        .parse()
        .map(Token::Number)
        .map_err(|e| TokenizeError::ParseNumberError(e))
}

fn tokenize_string(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    let mut string = String::new();
    let mut is_escape = false;

    loop {
        *index += 1;

        if *index >= chars.len() {
            return Err(TokenizeError::UnclosedQuotes);
        }
        let ch = chars[*index];
        match ch {
            '"' if !is_escape => break,
            '\\' if !is_escape => is_escape = true,
            _ => is_escape = false,
        }
        string.push(ch);
    }

    Ok(Token::String(string))
}



#[cfg(test)]
impl Token {
    pub fn string(input: &str) -> Self {
        Self::String(String::from(input))
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenize::TokenizeError;

    use super::{tokenize, Token};

    #[test]
    fn just_comma() {
        let input = String::from(",");
        let expected = [Token::Comma];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn all_punctuation() {
        let input = String::from("[{]},:");
        let expected = [
            Token::LeftBracket,
            Token::LeftBrace,
            Token::RightBracket,
            Token::RightBrace,
            Token::Comma,
            Token::Colon,
        ];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_null() {
        let input = String::from("null");
        let expected = [Token::Null];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_false() {
        let input = String::from("false");
        let expected = [Token::False];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_true() {
        let input = String::from("true");
        let expected = [Token::True];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn true_comma() {
        let input = String::from("true,");
        let expected = [Token::True, Token::Comma];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn integer() {
        let input = String::from("123");
        let expected = [Token::Number(123.0)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn float() {
        let input = String::from("123.456");
        let expected = [Token::Number(123.456)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn negative_integer() {
        let input = String::from("-123");
        let expected = [Token::Number(-123.0)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn negative_float() {
        let input = String::from("-123.456");
        let expected = [Token::Number(-123.456)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn negative_float_with_exponent() {
        let input = String::from("-123.456e+2");
        let expected = [Token::Number(-123.456e2)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn negative_float_with_exponent_and_sign() {
        let input = String::from("-123.456e-2");
        let expected = [Token::Number(-123.456e-2)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_ken() {
        let input = String::from("\"ken\"");
        let expected = [Token::string("ken")];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn unclosed_string() {
        let input = String::from("\"unclosed");
        let expected = Err(TokenizeError::UnclosedQuotes);

        let actual = tokenize(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn escaped_quote() {
        let input = String::from(r#""the \" is OK""#);
        let expected = [Token::string(r#"the \" is OK"#)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }
}
