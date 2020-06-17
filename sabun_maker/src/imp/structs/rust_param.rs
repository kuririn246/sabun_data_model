use crate::imp::structs::qv::{Qv, QvType};
use crate::imp::structs::rust_string::{RustString };
use crate::imp::structs::rust_value::RustValueType;
use crate::imp::structs::rust_array::{RustArray, RustNumArray, RustStrArray, RustNum2Array};
use crate::imp::structs::array_type::ArrayType;

#[derive(Debug, PartialEq, Clone)]
pub enum RustParam{
    Bool(Qv<bool>),
    Number(Qv<f64>),
    String(Qv<RustString>),
    NumArray(Qv<RustNumArray>),
    StrArray(Qv<RustStrArray>),
    Num2Array(Qv<RustNum2Array>)
}

impl RustParam {
    pub(crate) fn qv_type(&self) -> QvType {
        match self {
            RustParam::Bool(b) => b.qv_type(),
            RustParam::Number(n) => n.qv_type(),
            RustParam::String(s) => s.qv_type(),
            RustParam::NumArray(a) => a.qv_type(),
            RustParam::StrArray(a) => a.qv_type(),
            RustParam::Num2Array(a) => a.qv_type(),
        }
    }

    pub(crate) fn type_num(&self) -> RustValueType {
        use RustValueType::*;
        match self {
            RustParam::Bool(_) => Bool,
            RustParam::Number(_) => Num,
            RustParam::String(_) => Str,
            RustParam::NumArray(_) => NumArray,
            RustParam::StrArray(_) => StrArray,
            RustParam::Num2Array(_) => Num2Array,
        }
    }

    pub(crate) fn acceptable(&self, other: &Self) -> bool {
        if self.type_num() != other.type_num() {
            return false;
        }
        return true;
    }

    ///型情報を維持したままundefinedに変える
    pub(crate) fn to_undefined(&self) -> Self {
        match self {
            RustParam::Bool(_) => RustParam::Bool(Qv::Undefined),
            RustParam::Number(_) => RustParam::Number(Qv::Undefined),
            RustParam::String(_) => RustParam::String(Qv::Undefined),
            RustParam::NumArray(_) => RustParam::NumArray(Qv::Undefined),
            RustParam::StrArray(_) => RustParam::StrArray(Qv::Undefined),
            RustParam::Num2Array(_) => RustParam::Num2Array(Qv::Undefined)
        }
    }

    pub(crate) fn to_rust_array(&self) -> Option<(RustArray, ArrayType)>{
        match self{
            RustParam::NumArray(a) => Some((RustArray::from_num_array(a), ArrayType::Num)),
            RustParam::StrArray(a) => Some((RustArray::from_str_array(a), ArrayType::String)),
            RustParam::Num2Array(a) => Some((RustArray::from_num2_array(a), ArrayType::Num2)),
            _ => None,
        }
    }

    pub(crate) fn to_num(&self) -> Option<f64>{
        if let RustParam::Number(Qv::Val(s)) = self { Some(*s) } else{ None }
    }

    pub(crate) fn to_string(&self) -> Option<String>{
        if let RustParam::String(Qv::Val(s)) = self { Some(s.to_string()) } else{ None }
    }

    pub(crate) fn to_num_array(&self) -> Option<Vec<f64>>{
        if let RustParam::NumArray(Qv::Val(a)) = self { Some(a.as_ref().clone()) } else{ None }
    }

    // pub(crate) fn from_num_array(v: Vec<RustParam>, qv_type: &QvType) -> Option<RustParam> {
    //     match qv_type {
    //         QvType::Val => {
    //             let mut r: Vec<f64> = Vec::with_capacity(v.len());
    //             for item in v {
    //                 if let RustParam::Number(Qv::Val(f)) = item {
    //                     r.push(f);
    //                 } else {
    //                     return None;
    //                 }
    //             }
    //             Some(RustParam::NumArray(Qv::Val(r)))
    //         },
    //         QvType::Null => Some(RustParam::NumArray(Qv::Null)),
    //         QvType::Undefined => Some(RustParam::NumArray(Qv::Undefined))
    //     }
    // }
    //
    // pub(crate) fn from_str_array(v: Vec<RustParam>, qv_type: &QvType) -> Option<RustParam> {
    //     match qv_type {
    //         QvType::Val => {
    //             let mut r: Vec<String> = Vec::with_capacity(v.len());
    //             for item in v {
    //                 if let RustParam::String(Qv::Val(f)) = item {
    //                     r.push(f.str().to_string());
    //                 } else {
    //                     return None;
    //                 }
    //             }
    //             Some(RustParam::NumArray(Qv::Val(r)))
    //         },
    //         QvType::Null => Some(RustParam::NumArray(Qv::Null)),
    //         QvType::Undefined => Some(RustParam::NumArray(Qv::Undefined))
    //     }
    // }
}