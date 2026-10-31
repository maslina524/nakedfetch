use alloc::vec::Vec;
use core::{error::Error, fmt, str::FromStr};

use crate::{
    Map, Value,
    lexer::{Token, TokenStream},
};

#[derive(Debug)]
pub enum ParseError {
    Unexpected { expected: Token, got: Token },
    UnexpectedToken(Token),
    ExpectedInObject,
    ExpectedInArray,
    ObjectKeyMustBeStr,
    InvalidNum,
    ObjectKeyNotFound,
    Eoi,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unexpected { expected, got } => {
                write!(f, "Expected {expected:?}, got {got:?}")
            },
            Self::UnexpectedToken(t) => write!(f, "Unexpected token: {t:?}"),
            Self::ExpectedInObject => write!(f, "Expected `,` or `{{` in object"),
            Self::ExpectedInArray => write!(f, "Expected `,` or `[` in object"),
            Self::ObjectKeyMustBeStr => write!(f, "Object key must be a string"),
            Self::InvalidNum => write!(f, "Invalid number format"),
            Self::ObjectKeyNotFound => write!(f, "Expected object key"),
            Self::Eoi => write!(f, "End of input"),
        }
    }
}

impl Error for ParseError {}

pub struct Parser {
    iter: TokenStream,
    current: Option<Token>,
}

impl Parser {
    pub fn new(mut iter: TokenStream) -> Self {
        let current = iter.next();
        Self { iter, current }
    }

    const fn peek(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.current.take();
        self.current = self.iter.next();
        token
    }

    fn consume(&mut self, expected: &Token) -> Result<Token, ParseError> {
        let token = self.next().ok_or(ParseError::Eoi)?;
        if token == *expected {
            Ok(token)
        } else {
            Err(ParseError::Unexpected {
                expected: expected.clone(),
                got: token,
            })
        }
    }

    pub fn parse_value(&mut self) -> Result<Value, ParseError> {
        match self.peek() {
            Some(Token::LCurly) => Ok(Value::Dict(self.parse_object()?)),
            Some(Token::LBrace) => Ok(Value::Array(self.parse_array()?)),
            Some(Token::String(s)) => {
                let s = s.clone();
                self.next();
                Ok(Value::String(s))
            },
            Some(Token::Number(s)) => {
                let s = s.clone();
                self.next();
                let num = f64::from_str(&s).map_err(|_| ParseError::InvalidNum)?;
                Ok(Value::Number(num))
            },
            Some(Token::True) => {
                self.next();
                Ok(Value::Bool(true))
            },
            Some(Token::False) => {
                self.next();
                Ok(Value::Bool(false))
            },
            Some(Token::Null) => {
                self.next();
                Ok(Value::Null)
            },
            Some(t) => Err(ParseError::UnexpectedToken(t.clone())),
            None => Err(ParseError::Eoi),
        }
    }

    pub fn parse_object(&mut self) -> Result<Map, ParseError> {
        self.next(); // consume '{'
        let mut map = Map::new();

        if matches!(self.peek(), Some(Token::RCurly)) {
            self.next();
            return Ok(map);
        }

        loop {
            let key_token = self.next().ok_or(ParseError::ObjectKeyNotFound)?;
            let Token::String(key) = key_token else {
                return Err(ParseError::ObjectKeyMustBeStr);
            };

            self.consume(&Token::Colon)?;

            let value = self.parse_value()?;
            map.insert(key, value);

            match self.peek() {
                Some(Token::Comma) => {
                    self.next(); // consume ','
                    if matches!(self.peek(), Some(Token::RCurly)) {
                        self.next(); // consume '}'
                        break;
                    }
                },
                Some(Token::RCurly) => {
                    self.next();
                    break;
                },
                _ => return Err(ParseError::ExpectedInObject),
            }
        }

        Ok(map)
    }

    pub fn parse_array(&mut self) -> Result<Vec<Value>, ParseError> {
        self.next(); // consume '['
        let mut vec = Vec::new();

        if matches!(self.peek(), Some(Token::RBrace)) {
            self.next();
            return Ok(vec);
        }

        loop {
            let value = self.parse_value()?;
            vec.push(value);

            match self.peek() {
                Some(Token::Comma) => {
                    self.next(); // consume ','
                    if matches!(self.peek(), Some(Token::RBrace)) {
                        self.next(); // consume ']'
                        break;
                    }
                },
                Some(Token::RBrace) => {
                    self.next();
                    break;
                },
                _ => return Err(ParseError::ExpectedInArray),
            }
        }

        Ok(vec)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use core::fmt::Write;

        fn proc_map(f: &mut fmt::Formatter<'_>, map: &Map) -> fmt::Result {
            f.write_char('{')?;

            for (i, (k, v)) in map.iter().enumerate() {
                write!(f, "\"{k}\": ")?;

                match v {
                    Value::Null => write!(f, "null")?,
                    Value::Number(n) => proc_num(f, *n)?,
                    Value::String(s) => proc_string(f, s)?,
                    Value::Bool(b) => proc_bool(f, *b)?,
                    Value::Dict(m) => proc_map(f, m)?,
                    Value::Array(a) => proc_array(f, a)?,
                }

                if i != map.len() - 1 {
                    f.write_str(", ")?;
                }
            }

            f.write_char('}')?;
            Ok(())
        }

        fn proc_array(f: &mut fmt::Formatter<'_>, array: &[Value]) -> fmt::Result {
            f.write_char('[')?;

            for (i, v) in array.iter().enumerate() {
                match v {
                    Value::Null => write!(f, "null")?,
                    Value::Number(n) => proc_num(f, *n)?,
                    Value::String(s) => proc_string(f, s)?,
                    Value::Bool(b) => proc_bool(f, *b)?,
                    Value::Dict(m) => proc_map(f, m)?,
                    Value::Array(a) => proc_array(f, a)?,
                }

                if i != array.len() - 1 {
                    f.write_str(", ")?;
                }
            }

            f.write_char(']')?;
            Ok(())
        }

        fn proc_num(f: &mut fmt::Formatter<'_>, num: f64) -> fmt::Result {
            write!(f, "{num}")
        }

        fn proc_string(f: &mut fmt::Formatter<'_>, string: &str) -> fmt::Result {
            write!(f, "{string:?}")
        }

        fn proc_bool(f: &mut fmt::Formatter<'_>, b: bool) -> fmt::Result {
            write!(f, "{b}")
        }

        proc_map(f, self)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::lexer::TokenStream;

    #[test]
    fn single_chars_test() {
        let source = r#"{
            "key": "value",
            "array": [80, 42.55, true, null, "string"],
            "dict": {
                "type": "title",
                "idx": 1
            }
        }"#;
        let stream = TokenStream::new(source);

        let mut parser = Parser::new(stream);
        let obj = parser.parse_object();
        assert!(obj.is_ok());
    }

    #[test]
    fn escape_test() {
        let source = r#"{ "key": "\n\nescape\tvalue\n" }"#;
        let stream = TokenStream::new(source);

        let mut parser = Parser::new(stream);
        let ret = parser.parse_object();

        let Ok(map) = ret else {
            panic!("Ret is Err()");
        };

        let value = map.get_string("key").unwrap();
        assert_eq!(value, "\n\nescape\tvalue\n");
    }
}
