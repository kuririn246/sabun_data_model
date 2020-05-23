use crate::structs::value_type::ValueType;
use crate::structs::qv::{QvType, Qv};
use crate::structs::rust_list::RustList;
use crate::structs::array_type::ArrayType;

#[derive(Debug, PartialEq)]
pub enum RustParam{
    Bool(Qv<bool>, ValueType),
    Number(Qv<f64>, ValueType),
    String(Qv<String>, ValueType),
    Array(Qv<RustArray>, ArrayType, ValueType),
    //Listは定義上nullやundefinedにならない
    //List(RustList),
    //Objectは定義上nullやundefinedにならない
    //Object(RustObject),
}

impl RustParam{
    pub fn value_type(&self) -> ValueType {
        let vt = match self{
            RustParam::Bool(_,vt) => vt,
            RustParam::Number(_, vt) => vt,
            RustParam::String(_, vt) => vt,
            RustParam::Array(_, _at, vt) => vt,
        };
        vt.clone()
    }
}

pub enum RustValue{
    Param(RustParam),
    List(RustList)
}

impl RustValue{
    pub fn value_type(&self) -> ValueType {
        let vt = match self{
            RustValue::Param(param) => param.value_type(),
            RustValue::List(_) => &ValueType::Normal,
        };
        vt.clone()
    }

    pub(crate) fn type_num(&self) -> usize{
        match self{
            RustValue::Param(param) => match param{
                RustParam::Bool(_, _) => 0,
                RustParam::Number(_, _) => 1,
                RustParam::String(_, _) => 2,
                RustParam::Array(_, _, _) => 3,
            },
            RustValue::List(_) => 4,
            //RustValue::Object(_) => 5,
        }
    }

    pub fn qv_type(&self) -> QvType{
        match self{
            RustValue::Param(param) => match param{
                RustParam::Bool(b, _) => b.qv_type(),
                RustParam::Number(n, _) => n.qv_type(),
                RustParam::String(s, _) => s.qv_type(),
                RustParam::Array(a, _, _) => a.qv_type(),
            }
            RustValue::List(_) => QvType::Val,
            //RustValue::Object(_) => QvType::Val,
        }
    }

}



#[derive(Debug, PartialEq)]
pub struct RustArray{
    pub vec : Vec<RustValue>,

}
