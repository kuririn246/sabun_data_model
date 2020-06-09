use std::ffi::{CStr, CString};
use crate::imp::c_ifc::c_ifc::{CifcSt};
use crate::imp::c_ifc::c_enumerator::CEnumerator;
use crate::imp::c_ifc::c_ifc_value::{CifcValue};
use crate::imp::c_ifc::c_ifc_type::CifcType;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::value_type::ValueType;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CMemberDesc{
    name : CString,
    ifc_t : CifcType,
    value_t : ValueType,
}

pub extern "C" fn c_get_name_from_mem_desc(p : *const CMemberDesc) -> *const u8{
    p.name.as_c_str().to_bytes_with_nul().as_ptr()
}

pub extern "C" fn c_get_ifc_type_from_mem_desc(p : *const CMemberDesc) -> CifcType{
    p.ifc_t
}

pub extern "C" fn c_get_value_type_from_mem_desc(p : *const CMemberDesc) -> ValueType{
    p.value_t
}


#[derive(Debug, Clone)]
pub struct MemberDesc{
    pub name : String,
    pub ifc_t : CifcType,
    pub value_t : ValueType,
}

///this enumerator must be destroyed, items must not be destroyed
#[derive(Debug)]
pub struct CMemberEnumerator{
    enu : CEnumerator<CMemberDesc>
}

impl CMemberEnumerator{
    pub fn new(vec : Vec<MemberDesc>) -> Box<CMemberEnumerator>{
        Box::new(CMemberEnumerator{ enu : CEnumerator::new(vec.iter().map(|a|
            CMemberDesc{ name : CString::new(a.name.as_bytes()).unwrap(), ifc_t : a.ifc_t, value_t : a.value_t }).collect())})
    }
}

pub extern "C" fn c_member_enu_move_next(m : *mut CMemberEnumerator) -> bool{
    unsafe{ (*m).enu.move_next() }
}
pub extern "C" fn c_member_enu_current(m : *mut CMemberEnumerator) -> *mut CMemberDesc{
    unsafe{ (*m).enu.current() }
}
pub extern "C" fn c_member_enu_destroy(m : *mut CMemberEnumerator){
    unsafe{ Box::from_raw(m) };
}

pub extern "C" fn c_get_members(p : *const CifcSt) -> *mut CMemberEnumerator{
    todo!()
    // match unsafe{ p.as_ref() }{
    //     Some(p) =>{
    //         let vec = get_members(p.as_ref());
    //         let b = CMemberEnumerator::new(vec);
    //         Box::into_raw(b)
    //     },
    //     None =>{ 0 as *mut CMemberEnumerator }
    // }

}

pub fn get_members(st : &CifcSt) -> Vec<MemberDesc>{
    // match st.value(){
    //     CifcValue::Root(r) => get_members_root(r),
    // }
    todo!()
}

fn get_members_root(r : &RootObject) -> Vec<MemberDesc>{
    todo!()
    // let mut result : Vec<MemberDesc> = Vec::with_capacity(r.default().len());
    // for (k,v) in r.default(){
    //     let t = match v{
    //         RootValue::Param(_,_) => CifcType::Param,
    //         RootValue::Data(_) => CifcType::Data,
    //         RootValue::List(_) => CifcType::List,
    //         RootValue::Mut(_) => CifcType::Mut,
    //     };
    //     result.push(MemberDesc{ name : k.to_string(), ifc_t : t })
    // }
    // result
}