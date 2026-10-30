use alloc::{string::String, vec::Vec};
use core::str::FromStr;

use crate::{
    Value,
    lexer::TokenStream,
    parser::{ParseError, Parser},
};

#[derive(Debug, Clone, Default)]
pub struct Map(Vec<(String, Value)>);

impl Map {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.0.push((key, value));
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.get(key).and_then(Value::as_string)
    }

    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(Value::as_number)
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(Value::as_bool)
    }

    pub fn get_array(&self, key: &str) -> Option<&Vec<Value>> {
        self.get(key).and_then(Value::as_array)
    }

    pub fn get_object(&self, key: &str) -> Option<&Self> {
        self.get(key).and_then(Value::as_object)
    }
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stream = TokenStream::new(s);
        let mut parser = Parser::new(stream);
        parser.parse_object()
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr as _;

    use crate::Map;

    #[test]
    fn single_chars_test() {
        let source = r#"{
            "key": "value",
            "array": [80, 42.55, true, null, "string"],
            "dict": {
                "type": "title",
                "idx": 1
            },
            "nullable": null,
            "boolean": true
        }"#;

        let json = Map::from_str(source);
        println!("{json:#?}");
    }
}
