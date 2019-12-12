use std::collections::{HashMap, BTreeMap};

//There's no NullableNullableNumber in this system
pub enum RustValueType{
    Bool,
    NullableBool,
    Number,
    NullableNumber,
    String,
    NullableString
}

pub enum RustValue{
    Bool(bool),
    NullableBool(Option<bool>),
    Number(f64),
    NullableNumber(Option<f64>),
    String(String),
    NullableString(Option<String>),
    //Although Array can store any value in this implementation, it must declare it's type of the value.
    Array(Vec<RustValue>, RustValueType),
    //There's no Array's Array in this system. That's too hard to differentiate efficiently.
    NullableArray(Option<(Vec<RustValue>, RustValueType)>),
    Object(HashMap<String, RustValue>),
    NullableObject(Option<HashMap<String, RustValue>>)
}

pub struct RustArray{
    pub vec : Vec<RustValue>,
    pub val_type : RustValueType,
}

pub type RustObject = BTreeMap<String, RustValue>;