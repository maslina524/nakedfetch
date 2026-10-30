use alloc::{string::String, vec::Vec};

use crate::Map;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<Self>),
    Dict(Map),
    Null,
}

impl Value {
    pub const fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub const fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(s) => Some(*s),
            _ => None,
        }
    }

    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(s) => Some(*s),
            _ => None,
        }
    }

    pub const fn as_array(&self) -> Option<&Vec<Self>> {
        match self {
            Self::Array(s) => Some(s),
            _ => None,
        }
    }

    pub const fn as_object(&self) -> Option<&Map> {
        match self {
            Self::Dict(s) => Some(s),
            _ => None,
        }
    }

    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub const fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub const fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Dict(_))
    }

    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}
