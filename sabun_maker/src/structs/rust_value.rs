use crate::structs::value_type::ValueType;
use crate::structs::qv::{QvType, Qv};
use crate::structs::rust_list::{ConstData, ConstList, InitialList, MutList, MutData};
use crate::structs::array_type::ArrayType;
use crate::structs::root_object::ListDefObj;
use std::rc::Rc;

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
    ConstData(ConstData),
    ConstList(ConstList),
    IniList(InitialList),
    MutList(MutList),
    MutData(MutData),
    InnerConstListDef(Rc<ListDefObj>),
    InnerConstDataDef(Rc<ListDefObj>),
    InnerIniListDef(Rc<ListDefObj>),
    InnerMutListDef(Rc<ListDefObj>),
    InnerMutDataDef(Rc<ListDefObj>),
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
    // pub fn value_type(&self) -> ValueType {
    //     let vt = match self{
    //         RustValue::Param(_, vt) => vt.clone(),
    //         RustValue::Data(_) | RustValue::List(_) | RustValue::Mut(_) => ValueType::Normal,
    //     };
    // }

    pub(crate) fn type_num(&self) -> usize{
        match self{
            RustValue::Param(param, _) => match param{
                RustParam::Bool(_) => 0,
                RustParam::Number(_) => 1,
                RustParam::String(_) => 2,
                RustParam::Array(_, _) => 3,
            },
            RustValue::ConstData(_) | RustValue::InnerConstDataDef(_)=> 4,
            RustValue::ConstList(_) | RustValue::InnerConstListDef(_)=> 5,
            RustValue::IniList(_) | RustValue::InnerIniListDef(_) => 6,
            RustValue::MutList(_) | RustValue::InnerMutListDef(_) => 7,
            RustValue::MutData(_) | RustValue::InnerMutDataDef(_) => 8,
        }
    }



}




