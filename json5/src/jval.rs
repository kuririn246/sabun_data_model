use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum JVal{
    Null(Span),
    Bool(bool, Span),
    String(String, Span),
    Int(i64, Span),
    Double(f64, Span),
    Array(Vec<JVal>, Span),
    Map(BTreeMap<String, JVal>, Span)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span{
    pub start : usize,
    pub end : usize,
}