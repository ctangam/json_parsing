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

pub fn tokenize(input: String) -> Vec<Token> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::{tokenize, Token};

    #[test]
    fn just_comma() {
        let input = String::from(",");
        let expected = [Token::Comma];

        let actual = tokenize(input);

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
}