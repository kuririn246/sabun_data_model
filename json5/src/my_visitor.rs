use crate::error::Result;
use std::collections::BTreeMap;

pub struct MyVisitor{

}

impl MyVisitor{
    pub fn visit_unit(&mut self) -> Result<MyVisitorValue>{ todo!{} }
    pub fn visit_bool(&mut self, b : bool) -> Result<MyVisitorValue>{ todo!{} }
    pub fn visit_string(&mut self, s : String)-> Result<MyVisitorValue>{ todo!{} }
    pub fn visit_i64(&mut self, i : i64)-> Result<MyVisitorValue>{ todo!{} }
    pub fn visit_f64(&mut self, f : f64)-> Result<MyVisitorValue>{ todo!{} }
    pub fn visit_seq(&mut self, s : crate::de::Seq)-> Result<MyVisitorValue>{ todo!{} }
    pub fn visit_map(&mut self, m : crate::de::Map)-> Result<MyVisitorValue>{
         for p in m.pairs{
             println!("s {}", p.as_str());
             println!("span {:?}", p.as_span());
             println!("rule {:?}", p.as_rule());
         }
        Ok(MyVisitorValue{})
    }
    pub fn new() -> MyVisitor{ MyVisitor{} }
}

pub enum JVal{
    Null,
    Bool(bool),
    String(String),
    Int(i64),
    Double(f64),
    Array(Vec<JVal>),
    Map(BTreeMap<String, JVal>)
}

pub struct MyVisitorValue{




}