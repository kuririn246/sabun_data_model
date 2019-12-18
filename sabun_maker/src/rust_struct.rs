use std::collections::{HashMap, BTreeMap};

//There's no NullableNullableNumber in this system
pub enum RustArrayValueType{
    Number,
    String,
}

pub enum RustValue{
    Bool(bool),
    NullableBool(Option<bool>),
    Number(f64),
    NullableNumber(Option<f64>),
    String(String),
    NullableString(Option<String>),
    //Although Array can store any value in this implementation, it must declare it's type of the value.
    Array(RustArray),
    //There's no Array's Array in this system. That's too hard to differentiate efficiently.
    NullableArray(Option<RustArray>),
    Object(RustObject),
    NullableObject(Option<RustObject>)
}

pub struct RustArray{
    pub vec : Vec<RustValue>,
    pub val_type : RustArrayValueType,
}

pub type RustObject = BTreeMap<String, RustValue>;