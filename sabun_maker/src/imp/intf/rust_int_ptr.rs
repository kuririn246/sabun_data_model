use crate::imp::structs::qv::Qv;
use std::ptr::null;


#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct QvInt{
    val : i64,
    is_null : u32,
    is_undefined : u32,
}
impl QvInt{
    pub fn new(s : Qv<i64>)-> QvInt{
        QvIntPtr{ s }
    }
}
#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct OptInt{
    val : i64,
    is_valid : u32,
}
impl OptInt{
    pub fn new(val : i64, is_valid : u32) -> OptInt{ OptInt{ val, is_valid } }
}

#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvInt_NullOr(p : QvInt) -> OptInt{
    OptInt::new(p.val, p.is_null)
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvInt_UndefOr(p : QvInt) -> OptInt{
    OptInt::new(p.val, p.is_undefined)
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvInt_Value(p : QvInt) -> i64{
    p.val
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_IsNull(p : QvInt) -> i8{
    p.is_null as i8
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_IsUndefined(p : QvInt) -> i8{
    p.is_undefined as i8
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn OptInt_Value(p : OptInt) -> i64{
    p.val
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn OptInt_IsValid(p : OptInt) -> i8{
    p.is_valid as i8
}