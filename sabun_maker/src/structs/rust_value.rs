use crate::structs::value_type::ValueType;
use crate::structs::qv::{QvType, Qv};
use crate::structs::rust_list::{ConstData, ConstList, MutList, InnerList, InnerData, InnerMutList};
use crate::structs::array_type::ArrayType;
use crate::structs::root_object::{ListDefObj, InnerMutDefObj};

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
    InnerList(InnerList),
    InnerData(InnerData),
    InnerMut(InnerMutObj),
    InnerListDef(ListDefObj),
    InnerDataDef(ListDefObj),
    InnerMutDef(InnerMutDefObj),
}

#[derive(Debug, PartialEq)]
pub struct InnerMutObj{
    pub list : Option<InnerMutList>,
    pub undefinable : bool,
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
            RustValue::List(_) => 4,
            RustValue::Data(_) => 5,
            RustValue::Mut(_) => 6,
            RustValue::InnerList(_) | RustValue::InnerListDef(_) => 7,
            RustValue::InnerData(_) | RustValue::InnerDataDef(_) => 8,
            RustValue::InnerMut(_) | RustValue::InnerMutDef(_) => 9,
        }
    }

    pub fn value_type(&self) -> ValueType{
        match self{
            RustValue::Param(_param, vt) => vt.clone(),
            _ => ValueType::Normal,
        }
    }



}




