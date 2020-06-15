use crate::imp::structs::qv::{Qv, QvType};
use crate::imp::structs::rust_string::RustString;
use crate::imp::structs::rust_array::RustArray;
use crate::imp::structs::array_type::ArrayType;
use crate::imp::structs::rust_value::RustValueType;

#[derive(Debug, PartialEq, Clone)]
pub enum RustParam{
    Bool(Qv<bool>),
    Number(Qv<f64>),
    String(Qv<RustString>),
    ///ArrayTypeはただのフラグなのでRustParamのenumに畳み込まれるようだ
    Array(RustArray, ArrayType),
}

impl RustParam{
    pub(crate) fn qv_type(&self) -> QvType {
        match self {
            RustParam::Bool(b) => b.qv_type(),
            RustParam::Number(n) => n.qv_type(),
            RustParam::String(s) => s.qv_type(),
            RustParam::Array(a, _) => a.qv().qv_type(),
        }
    }

    pub(crate) fn type_num(&self) -> RustValueType{
        use RustValueType::*;
        match self{
            RustParam::Bool(_) => Bool,
            RustParam::Number(_) => Num,
            RustParam::String(_) => Str,
            RustParam::Array(_,ArrayType::Num) => NumArray,
            RustParam::Array(_,ArrayType::String) => StrArray,
            RustParam::Array(_,ArrayType::Num2) => Num2Array,
        }
    }

    pub(crate) fn acceptable(&self, other : &Self) -> bool {
        if self.type_num() != other.type_num() {
            return false;
        }
        return true;
    }

    ///型情報を維持したままundefinedに変える
    pub(crate) fn to_undefined(&self) -> Self{
        match self{
            RustParam::Bool(_) => RustParam::Bool(Qv::Undefined),
            RustParam::Number(_) => RustParam::Number(Qv::Undefined),
            RustParam::String(_) => RustParam::String(Qv::Undefined),
            RustParam::Array(_a, at) => RustParam::Array(RustArray::new(Qv::Undefined), at.clone())

        }
    }
}