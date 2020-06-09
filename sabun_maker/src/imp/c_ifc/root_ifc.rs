use crate::imp::structs::root_obj::RootObject;
use std::ffi::CStr;
use crate::imp::c_ifc::c_ifc::CifcSt;
use crate::imp::structs::rust_param::RustParam;

pub struct RootIfc{
    root : RootObject
}

impl RootIfc{
    pub fn new(root : RootObject) -> RootIfc{ RootIfc{ root }}
    pub fn root(&self) -> &RootObject{ &self.root }
}

pub extern "C" fn c_create_root_from_dir(dir_path : &CStr, validation : bool) -> *mut RootIfc{
    match dir_path.to_str(){
        Ok(s) => {
            match crate::json_dir_to_rust(s, validation) {
                Ok(root) => {
                    let b = Box::new(RootIfc::new(root));
                    return Box::into_raw(b);
                }
                _ =>{}
            }
        },
        _ =>{}
    }
    return std::ptr::null_mut();
}

pub extern "C" fn c_destroy_root(p : *mut RootIfc){
    unsafe{ Box::from_raw(p); }
}

pub extern "C" fn c_get_value_from_root(p : *const RootIfc, name : *const u8) -> *mut CifcSt{

}

pub fn get_value_from_root(p : &RootIfc, name : &str) -> CifcSt{
    if let Some(param) = p.root.sabun().get(name){
        match param{
            RustParam::Bool(b) =>
        }
    }
}