use indexmap::IndexMap;

pub enum Value{
    Bool(bool),
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Map(IndexMap<String, Value>),
    Null,
    Undefined,
}