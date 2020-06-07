use crate::imp::structs::qv::{Qv, QvType};
use crate::imp::structs::rust_string::RustString;
use crate::imp::structs::rust_array::RustArray;
use crate::imp::structs::array_type::ArrayType;

#[derive(Debug, PartialEq, Clone)]
pub enum RustParam{
    Bool(Qv<bool>),
    Number(Qv<f64>),
    String(Qv<RustString>),
    Array(RustArray),
}

impl RustParam{
    pub(crate) fn qv_type(&self) -> QvType {
        match self {
            RustParam::Bool(b) => b.qv_type(),
            RustParam::Number(n) => n.qv_type(),
            RustParam::String(s) => s.qv_type(),
            RustParam::Array(a) => a.qv().qv_type(),
        }
    }

    pub(crate) fn type_num(&self) -> usize{
        match self{
            RustParam::Bool(_) => 0,
            RustParam::Number(_) => 1,
            RustParam::String(_) => 2,
            RustParam::Array(a) => match a.array_type(){
                ArrayType::Num => 3,
                ArrayType::String => 4,
                ArrayType::Num2 => 5,
            },
        }
    }

    pub(crate) fn acceptable(&self, other : &Self) -> bool {
        if self.type_num() != other.type_num() {
            return false;
        }
        if let RustParam::Array(s) = self {
            if let RustParam::Array(o) = other {
                //array_typeが一致してるかはここまでしないと調べられないだろうか・・・？
                if s.array_type().type_num() != o.array_type().type_num() {
                    return false;
                }
            } else { unreachable!() }
        }
        return true;
    }

    ///型情報を維持したままundefinedに変える
    pub(crate) fn to_undefined(&self) -> Self{
        match self{
            RustParam::Bool(_) => RustParam::Bool(Qv::Undefined),
            RustParam::Number(_) => RustParam::Number(Qv::Undefined),
            RustParam::String(_) => RustParam::String(Qv::Undefined),
            RustParam::Array(a) => RustParam::Array(RustArray::undefined(a.array_type()))

        }
    }
}