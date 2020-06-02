use crate::structs::value_type::ValueType;
use crate::structs::qv::{QvType, Qv};
use crate::structs::rust_list::{ConstData, ConstList, MutList, InnerList, InnerData, InnerMutList};
use crate::structs::array_type::ArrayType;
use crate::structs::root_object::{ListDefObj, InnerMutDefObj};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum RustParam{
    Bool(Qv<bool>),
    Number(Qv<f64>),
    String(Qv<RustString>),
    Array(Qv<RustArray>, ArrayType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum RustValue{
    Param(RustParam, ValueType),
    Data(ConstData),
    List(ConstList),
    Mut(MutList),
    InnerData(InnerData),
    InnerList(InnerList),
    ///InnerMutListだけundefinedになりうる
    InnerMut(Option<InnerMutList>),
    InnerDataDef(ListDefObj),
    InnerListDef(ListDefObj),
    InnerMutDef(InnerMutDefObj),
}

pub enum ListType{
    Data, List, Mut, InnerData, InnerList, InnerMut, InnderDataDef, InnerListDef, InnerMutDef,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustArray{
    vec : Box<Vec<RustParam>>,
}

impl RustArray{
    pub fn new(v : Vec<RustParam>) -> RustArray{ RustArray{ vec : Box::new(v)} }
    pub fn vec(&self) -> &[RustParam]{ self.vec.as_ref().as_ref() }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustString{
    str : Box<String>,
}

impl RustString{
    pub fn new(s : String) -> RustString{ RustString{ str : Box::new(s) }}
    pub fn str(&self) -> &str{ self.str.as_ref().as_ref() }
}

impl Display for RustString{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.str().fmt(f)
    }
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
            RustValue::InnerMutDef(obj) => if obj.undefinable() { ValueType::Undefinable } else{ ValueType::Normal }
            _ => ValueType::Normal,
        }
    }

    pub fn list_type(&self) -> Option<ListType>{
        Some(match self{
            RustValue::Data(_) => ListType::Data,
            RustValue::List(_) => ListType::Data,
            RustValue::Mut(_) => ListType::Data,
            RustValue::InnerData(_) => ListType::InnerData,
            RustValue::InnerList(_) => ListType::InnerList,
            RustValue::InnerMut(_) => ListType::InnerMut,
            RustValue::InnerDataDef(_) => ListType::InnderDataDef,
            RustValue::InnerListDef(_) => ListType::InnerListDef,
            RustValue::InnerMutDef(_) => ListType::InnerMutDef,
            _ => return None,
        })
    }


}




