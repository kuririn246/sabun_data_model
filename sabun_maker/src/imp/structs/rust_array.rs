use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;

#[derive(Debug, PartialEq, Clone)]
pub struct RustArray{
    array : Box<Qv<Vec<RustParam>>>,
}

// #[derive(Debug, PartialEq, Clone)]
// pub(crate) struct RustArrayInternal{
//     qv : ,
//
// }

impl RustArray{
    pub(crate) fn new(qv : Qv<Vec<RustParam>>) -> RustArray{
        RustArray{ array : Box::new(qv) }
    }

    // pub(crate) fn null(at : ArrayType) -> RustArray{
    //     RustArray{ array : Box::new(RustArrayInternal{
    //         qv : Qv::Null, at
    //     })}
    // }
    // pub(crate) fn undefined(at : ArrayType) -> RustArray{
    //     RustArray{ array : Box::new(RustArrayInternal{
    //         qv : Qv::Undefined, at
    //     })}
    // }
    pub(crate) fn qv(&self) -> &Qv<Vec<RustParam>>{ self.array.as_ref() }
    //pub(crate) fn array_type(&self) -> ArrayType{ self.array.at.clone() }

    pub(crate) fn to_num_array(&self) -> Qv<Vec<f64>>{

    }
}