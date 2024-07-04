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
    let ch = chars[*index];

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

        _ => todo!("implement tokenization for {}", ch),
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

    Ok(Token::Null)
}

fn tokenize_false(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "false".chars() {
        if chars[*index] != expected_char {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }

    Ok(Token::False)
}

fn tokenize_true(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "true".chars() {
        if chars[*index] != expected_char {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }

    Ok(Token::True)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizeError {
    UnfinishedLiteralValue,
}

#[cfg(test)]
mod tests {
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
}
