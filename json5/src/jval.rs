use std::collections::BTreeMap;

pub enum JVal{
    Null(Span),
    Bool(bool, Span),
    String(String, Span),
    Int(i64, Span),
    Double(f64, Span),
    Array(Vec<JVal>, Span),
    Map(BTreeMap<String, JVal>, Span)
}

pub struct Span{
    pub start : usize,
    pub end : usize,
}