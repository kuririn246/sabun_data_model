use crate::imp::c_ifc::root_ifc::RootIfc;
use crate::imp::c_ifc::c_ifc::CifcSt;

pub extern "C" fn c_get_value_from_root(root : *const RootIfc, name : *const u8){

}

pub fn get_value_from_root(root : &RootIfc, name : &str) -> CifcSt{

}