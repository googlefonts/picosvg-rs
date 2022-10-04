use std::collections::HashMap;
use std::string::String;

pub trait ValueDescriptor {
    fn into_value(self) -> Value;
    fn try_deref_value(value: &Value) -> Option<&Self>;
}

macro_rules! value_desc {
    ($name:ident) => {
        impl ValueDescriptor for $name {
            fn into_value(self) -> Value {
                Value::$name(self)
            }
            fn try_deref_value(value: &Value) -> Option<&Self> {
                match value {
                    Value::$name(value) => Some(value),
                    _ => None,
                }
            }
        }
    };
}

pub trait AttributeDescriptor {
    const ID: &'static str;
    type Value: ValueDescriptor;

    fn id(&self) -> &'static str {
        Self::ID
    }

    fn validate(value: &Value) -> bool {
        Self::Value::try_deref_value(value).is_some()
    }
}

macro_rules! attr_desc {
    ($name:ident, $key:expr, $ty:ty) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $name;
        impl AttributeDescriptor for $name {
            const ID: &'static str = $key;
            type Value = $ty;
        }
    };
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Unit {
    None,
    Em,
    Ex,
    Px,
    In,
    Cm,
    Mm,
    Pt,
    Pc,
    Percent,
}

#[derive(Copy, Clone, Debug)]
pub struct Length {
    pub value: f32,
    pub unit: Unit,
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Debug)]
pub enum Value {
    Length(Length),
    Color(Color),
    String(String),
}

#[derive(Default, Debug)]
pub struct AttributeMap {
    items: HashMap<&'static str, Value>,
}

impl AttributeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<A: AttributeDescriptor>(&mut self, attr: A, value: A::Value) -> Option<Value> {
        self.items.insert(attr.id(), value.into_value())
    }

    pub fn get<A: AttributeDescriptor>(&self, attr: A) -> Option<&A::Value> {
        self.items
            .get(attr.id())
            .map(|value| A::Value::try_deref_value(value))
            .flatten()
    }

    pub fn insert_by_id(&mut self, id: &str, value: Value) -> Result<Option<Value>, InsertError> {
        let (id, validator) = match ATTRIBUTE_VALIDATORS.binary_search_by(|probe| probe.0.cmp(&id))
        {
            Ok(idx) => ATTRIBUTE_VALIDATORS[idx],
            _ => return Err(InsertError::InvalidAttribute),
        };
        if !validator(&value) {
            return Err(InsertError::InvalidValue);
        }
        Ok(self.items.insert(id, value))
    }

    pub fn get_by_id(&self, id: &str) -> Option<&Value> {
        self.items.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &Value)> + Clone {
        self.items.iter().map(|(a, b)| (*a, b))
    }
}

#[derive(Debug)]
pub enum InsertError {
    InvalidAttribute,
    InvalidValue,
}

include!("../generated/attribute.rs");
