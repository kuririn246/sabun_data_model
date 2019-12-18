use std::collections::{HashMap, BTreeMap};

pub enum ArrayType{
    Num,
    String,
    Num2, //two dimensional num array
}

pub enum RustValue{
    Bool(bool),
    NullableBool(Option<bool>),
    Number(f64),
    NullableNumber(Option<f64>),
    String(String),
    NullableString(Option<String>),
    Array(RustArray),
    NullableArray(Option<RustArray>),
    List(RustList),
    NullableList(Option<RustList>),
    Object(RustObject),
    NullableObject(Option<RustObject>)
}

pub struct RustArray{
    pub vec : Vec<RustValue>,
    pub array_type : ArrayType,
}

pub struct RustList{

}

pub type RustObject = BTreeMap<String, RustValue>;