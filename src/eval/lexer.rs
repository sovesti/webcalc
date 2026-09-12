use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BracketType {
    Opening,
    Closing,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    NumberInt(i64),
    NumberFloat(f64),
    Operator(Operator),
    Bracket(BracketType),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CalcError {
    InvalidPosition { position: usize },
    UnexpectedCharacter { position: usize, character: char },
    InvalidNumber { position: usize },
    EmptyExpression,
    UnexpectedEnd,
    UnexpectedToken { position: usize },
    ExpectedClosingBracket { position: usize },
    DivisionByZero,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPosition { position } => write!(f, "invalid position {position}"),
            Self::UnexpectedCharacter {
                position,
                character,
            } => write!(
                f,
                "unexpected character {character:?} at position {position}"
            ),
            Self::InvalidNumber { position } => write!(f, "invalid number at position {position}"),
            Self::EmptyExpression => write!(f, "empty expression"),
            Self::UnexpectedEnd => write!(f, "unexpected end of expression"),
            Self::UnexpectedToken { position } => {
                write!(f, "unexpected token at index {position}")
            }
            Self::ExpectedClosingBracket { position } => {
                write!(f, "expected closing bracket at token index {position}")
            }
            Self::DivisionByZero => write!(f, "division by zero"),
        }
    }
}

impl std::error::Error for CalcError {}

pub fn get_token(expr: &str, pos: usize) -> Result<(Token, usize), CalcError> {
    let rest = expr
        .get(pos..)
        .filter(|rest| !rest.is_empty())
        .ok_or(CalcError::InvalidPosition { position: pos })?;

    let token = match rest.as_bytes()[0] {
        b'0'..=b'9' => {
            let bytes = expr.as_bytes();
            let mut end = pos;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }

            if end < bytes.len() && bytes[end] == b'.' {
                end += 1;
                if end == bytes.len() || !bytes[end].is_ascii_digit() {
                    return Err(CalcError::InvalidNumber { position: pos });
                }
                while end < bytes.len() && bytes[end].is_ascii_digit() {
                    end += 1;
                }
                let value = expr[pos..end]
                    .parse::<f64>()
                    .map_err(|_| CalcError::InvalidNumber { position: pos })?;
                if !value.is_finite() {
                    return Err(CalcError::InvalidNumber { position: pos });
                }
                return Ok((Token::NumberFloat(value), end));
            }

            let value = expr[pos..end]
                .parse::<i64>()
                .map_err(|_| CalcError::InvalidNumber { position: pos })?;
            return Ok((Token::NumberInt(value), end));
        }
        b'+' => Token::Operator(Operator::Plus),
        b'-' => Token::Operator(Operator::Minus),
        b'*' => Token::Operator(Operator::Multiply),
        b'/' => Token::Operator(Operator::Divide),
        b'(' => Token::Bracket(BracketType::Opening),
        b')' => Token::Bracket(BracketType::Closing),
        _ => {
            return Err(CalcError::UnexpectedCharacter {
                position: pos,
                character: rest
                    .chars()
                    .next()
                    .ok_or(CalcError::InvalidPosition { position: pos })?,
            });
        }
    };

    Ok((token, pos + 1))
}

pub fn tokenize(expression: String) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < expression.len() {
        let character = expression[pos..]
            .chars()
            .next()
            .ok_or(CalcError::InvalidPosition { position: pos })?;
        if character.is_whitespace() {
            pos += character.len_utf8();
            continue;
        }

        let (token, next_pos) = get_token(&expression, pos)?;
        tokens.push(token);
        pos = next_pos;
    }

    Ok(tokens)
}