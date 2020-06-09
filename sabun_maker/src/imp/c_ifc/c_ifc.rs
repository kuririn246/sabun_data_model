use crate::imp::structs::root_obj::RootObject;
use crate::imp::c_ifc::c_ifc_type::CifcType;
use crate::imp::c_ifc::c_ifc_value::CifcValue;

#[repr(C)]
pub struct CifcSt{
    pub cifc_type : CifcType,
    value : Box<CifcValue>,
}

impl CifcSt{
    pub(crate) fn value(&self) -> &CifcValue { self.value.as_ref() }
}

pub extern "C" fn get_type(p : *const CifcSt) -> CifcType{
    p.cifc_type
}


