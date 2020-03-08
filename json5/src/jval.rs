use std::collections::BTreeMap;
use std::rc::Rc;

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
    pub text : Rc<String>,
}

impl JVal{
    pub fn as_object(&self) -> Option<&BTreeMap<String, JVal>>{
        return match self {
            JVal::Map(map, _span) => { Some(map) }
            _ => { None }
        }
    }
}