use std::ffi::CStr;
use crate::imp::c_ifc::c_ifc::{CifcSt, CifcType};
use crate::imp::c_ifc::c_enumerator::CEnumerator;

#[repr(C)]
pub struct CMemberDesc{
    pub name : Box<CStr>,
    pub ifc_t : CifcType,
}

pub struct MemberDesc{
    pub name : String,
    pub ifc_t : CifcType,
}

pub struct CMemberEnumerator{
    enu : CEnumerator<CMemberDesc>
}

pub extern "C" fn c_get_members(p : *const CifcSt) -> *mut CMemberEnumerator{
    todo!()
}

pub fn get_members(st : &CifcSt) -> Vec<MemberDesc>{

}