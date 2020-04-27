use crate::structs::value_type::ValueType;
use crate::structs::qv::{QvType, Qv};
use crate::structs::rust_object::RustObject;
use crate::structs::rust_list::RustList;
use crate::structs::array_type::ArrayType;

#[derive(Debug, PartialEq)]
pub enum RustValue{
    Bool(Qv<bool>, ValueType),
    Number(Qv<f64>, ValueType),
    String(Qv<String>, ValueType),
    Array(Qv<RustArray>, ArrayType, ValueType),
    ///Listは定義上nullやundefinedにならない
    List(RustList),
    //Objectは定義上nullやundefinedにならない
    //Object(RustObject),
}

impl RustValue{
    pub fn value_type(&self) -> ValueType {
        let vt = match self{
            RustValue::Bool(_,vt) => vt,
            RustValue::Number(_, vt) => vt,
            RustValue::String(_, vt) => vt,
            RustValue::Array(_, _at, vt) => vt,
            RustValue::List(_) => &ValueType::Normal,
            //RustValue::Object(_) => &ValueType::Normal,
        };
        vt.clone()
    }

    pub(crate) fn type_num(&self) -> usize{
        match self{
            RustValue::Bool(_, _) => 0,
            RustValue::Number(_, _) => 1,
            RustValue::String(_, _) => 2,
            RustValue::Array(_, _, _) => 3,
            RustValue::List(_) => 4,
            //RustValue::Object(_) => 5,
        }
    }

    pub fn qv_type(&self) -> QvType{
        match self{
            RustValue::Bool(b, _) => b.qv_type(),
            RustValue::Number(n, _) => n.qv_type(),
            RustValue::String(s, _) => s.qv_type(),
            RustValue::Array(a, _, _) => a.qv_type(),
            RustValue::List(_) => QvType::Val,
            //RustValue::Object(_) => QvType::Val,
        }
    }

}



#[derive(Debug, PartialEq)]
pub struct RustArray{
    pub vec : Vec<RustValue>,

}
