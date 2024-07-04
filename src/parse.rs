use crate::{tokenize::Token, value::Value};

type ParseResult = Result<Value, TokenParseError>;

fn parse_tokens(tokens: Vec<Token>, index: &mut usize) -> ParseResult {
    let token = &tokens[*index];

    match token {
        Token::Null => Ok(Value::Null),
        Token::False => Ok(Value::Boolean(false)),
        Token::True => Ok(Value::Boolean(true)),
        Token::Number(number) => Ok(Value::Number(*number)),
        Token::String(string) => parse_string(string),
        Token::LeftBracket => todo!(),
        Token::LeftBrace => todo!(),

        _ => todo!(),
    }
}

fn parse_string(s: &str) -> ParseResult {
    let mut output = String::new();

    let mut is_escaping = false;
    let mut chars = s.chars();

    while let Some(next_char) = chars.next() {
        if is_escaping {
            match next_char {
                '"' => output.push('"'),
                '\\' => output.push('\\'),
                'b' => output.push('\u{8}'),
                'f' => output.push('\u{12}'),
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                'u' => {
                    let mut sum = 0;
                    for i in 0..4 {
                        let next_char = chars.next().ok_or(TokenParseError::UnfinishedEscape)?;
                        let digit = next_char
                            .to_digit(16)
                            .ok_or(TokenParseError::InvalidHexValue)?;
                        sum += (16u32).pow(3 - i) * digit;
                    }
                    let unescaped_char =
                        char::from_u32(sum).ok_or(TokenParseError::InvalidCodePointValue)?;
                    output.push(unescaped_char);
                }
                // any other character *may* be escaped, ex. `\q` just push that letter `q`
                _ => output.push(next_char),
            }
            is_escaping = false;
        } else if next_char == '\\' {
            is_escaping = true;
        } else {
            output.push(next_char)
        }
    }

    Ok(Value::String(output))
}

#[derive(Debug, PartialEq)]
enum TokenParseError {
        /// An escape sequence was started without 4 hexadecimal digits afterwards
        UnfinishedEscape,
        /// A character in an escape sequence was not valid hexadecimal
        InvalidHexValue,
        /// Invalid unicode value
        InvalidCodePointValue,
}

#[cfg(test)]
mod tests {
    use crate::tokenize::Token;
    use crate::value::Value;

    use super::parse_tokens;

    fn check(input: Vec<Token>, expected: Value) {
        let actual = parse_tokens(input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_null() {
        let input = vec![Token::Null];
        let expected = Value::Null;

        check(input, expected);
    }

    #[test]
    fn parses_string_no_escapes() {
        let input = vec![Token::String("hello world".into())];
        let expected = Value::String("hello world".into());

        check(input, expected);
    }

    #[test]
    fn parses_string_non_ascii() {
        let input = vec![Token::string("olá_こんにちは_नमस्ते_привіт")];
        let expected = Value::String(String::from("olá_こんにちは_नमस्ते_привіт"));

        check(input, expected);
    }

    #[test]
    fn parses_string_with_emoji() {
        let input = vec![Token::string("hello 💩 world")];
        let expected = Value::String(String::from("hello 💩 world"));

        check(input, expected);
    }

    #[test]
    fn parses_string_unescape_backslash() {
        let input = vec![Token::String(r#"hello\\world"#.into())];
        let expected = Value::String(r#"hello\world"#.into());

        check(input, expected);
    }
}
