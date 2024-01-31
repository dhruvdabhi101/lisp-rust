#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen
}

#[derive(Debug)]
pub struct TokenError {
    err: String,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
    let program = input.replace("(", " ( ").replace(")", " ) ");
    let words = program.split_whitespace();

    let mut tokens: Vec<Token> = Vec::new();
    for word in words {
        match word {
            "("=> tokens.push(Token::LParen),
            ")"=> tokens.push(Token::RParen),
            _ => {
                let i = word.parse::<i64>();
                if i.is_ok() {
                    tokens.push(Token::Integer(i.unwrap()));
                } else {
                    tokens.push(Token::Symbol(word.to_string()));
            }
    }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "(+ 1 2)";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::LParen);
        assert_eq!(tokens[1], Token::Symbol("+".to_string()));
        assert_eq!(tokens[2], Token::Integer(1));
        assert_eq!(tokens[3], Token::Integer(2));
        assert_eq!(tokens[4], Token::RParen);
    }
}
