use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::array_type::ArrayType;

#[derive(Debug, PartialEq, Clone)]
pub struct RustArray{
    array : Box<RustArrayInternal>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct RustArrayInternal{
    qv : Qv<Vec<RustParam>>,
    at : ArrayType,
}

impl RustArray{
    pub(crate) fn new(qv : Qv<Vec<RustParam>>, at : ArrayType) -> RustArray{
        RustArray{ array : Box::new(RustArrayInternal{
            qv, at
        })}
    }
    pub(crate) fn null(at : ArrayType) -> RustArray{
        RustArray{ array : Box::new(RustArrayInternal{
            qv : Qv::Null, at
        })}
    }
    pub(crate) fn undefined(at : ArrayType) -> RustArray{
        RustArray{ array : Box::new(RustArrayInternal{
            qv : Qv::Undefined, at
        })}
    }
    pub(crate) fn qv(&self) -> &Qv<Vec<RustParam>>{ &self.array.qv }
    pub(crate) fn array_type(&self) -> ArrayType{ self.array.at.clone() }
}