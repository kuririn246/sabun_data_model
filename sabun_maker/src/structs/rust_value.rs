use crate::structs::value_type::ValueType;
use crate::structs::qv::{QvType, Qv};
use crate::structs::rust_list::{ConstData, ConstList, MutList, InnerList };
use crate::structs::array_type::ArrayType;
use crate::structs::root_object::ListDefObj;

#[derive(Debug, PartialEq)]
pub enum RustParam{
    Bool(Qv<bool>),
    Number(Qv<f64>),
    String(Qv<String>),
    Array(Qv<RustArray>, ArrayType),
}

#[derive(Debug, PartialEq)]
pub enum RustValue{
    Param(RustParam, ValueType),
    Data(ConstData),
    List(ConstList),
    Mut(MutList),
    Inner(InnerList),
    InnerDef(ListDefObj),
}

#[derive(Debug, PartialEq)]
pub struct RustArray{
    pub vec : Vec<RustParam>,
}

impl RustParam{
    pub fn qv_type(&self) -> QvType {
        match self {
            RustParam::Bool(b) => b.qv_type(),
            RustParam::Number(n) => n.qv_type(),
            RustParam::String(s) => s.qv_type(),
            RustParam::Array(a, _) => a.qv_type(),
        }
    }
}

impl RustValue{

    ///この数値は仮
    pub(crate) fn type_num(&self) -> usize{
        match self{
            RustValue::Param(param, _) => match param{
                RustParam::Bool(_) => 0,
                RustParam::Number(_) => 1,
                RustParam::String(_) => 2,
                RustParam::Array(_, _) => 3,
            },
            RustValue::Data(_) => 4,
            RustValue::List(_) => 5,
            RustValue::Mut(_) => 6,
            RustValue::Inner(_) | RustValue::InnerDef(_) => 7,
        }
    }




}




