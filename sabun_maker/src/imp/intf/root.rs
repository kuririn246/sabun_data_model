use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::qv::Qv;
use crate::HashM;
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_list::{ MutListItem};
use crate::imp::intf::ConstDataPtr;
use crate::imp::structs::rust_string::RustString;
use crate::imp::intf::list::ConstListPtr;
use crate::imp::intf::mut_list_ptr::MutListPtr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RootObjectPtr{
    ptr : *mut RootObject
}
impl RootObjectPtr {
    pub fn new(ptr: *mut RootObject) -> RootObjectPtr { RootObjectPtr { ptr } }
}

pub fn get_bool(root : RootObjectPtr, name : &str) -> Option<Qv<bool>>{
    let root = unsafe{ root.ptr.as_ref().unwrap() };
    if let Some(RustParam::Bool(b)) = get_param(root.default(), root.sabun(), name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_float(root : RootObjectPtr, name : &str) -> Option<Qv<f64>>{
    let root = unsafe{ root.ptr.as_ref().unwrap() };
    if let Some(RustParam::Float(b)) = get_param(root.default(), root.sabun(), name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_int(root : RootObjectPtr, name : &str) -> Option<Qv<i64>>{
    let root = unsafe{ root.ptr.as_ref().unwrap() };
    if let Some(RustParam::Int(b)) = get_param(root.default(), root.sabun(), name){
        Some(b.clone())
    } else{
        None
    }
}


pub fn get_str(root : RootObjectPtr, name : &str) -> Option<Qv<String>>{
    let root = unsafe{ root.ptr.as_ref().unwrap() };
    if let Some(RustParam::String(b)) = get_param(root.default(), root.sabun(), name){
        Some(b.map(|s| s.str().to_string()))
    } else{
        None
    }
}

pub fn get_data(root_ptr : RootObjectPtr, name : &str) -> Option<ConstDataPtr>{
    let root = unsafe{ root_ptr.ptr.as_ref().unwrap() };
    if let Some(RootValue::Data(d)) = root.default().get(name){
        Some(ConstDataPtr::new(d, root_ptr.ptr))
    } else{ None }
}
// pub fn get_data2(root : *const RootObject, name : &str) -> Option<ConstDataPtr>{
//     get_data(RootObjectPtr::new(root as *mut RootObject), name)
// }

pub fn get_list(root_ptr : RootObjectPtr, name : &str) -> Option<ConstListPtr>{
    let root = unsafe{ root_ptr.ptr.as_ref().unwrap() };
    if let Some(RootValue::List(l)) = root.default().get(name){
        Some(ConstListPtr::new(l, root_ptr.ptr))
    } else{ None }
}

pub fn get_mut<T : From<*mut MutListItem>>(root : *mut RootObject, name : &str) -> Option<MutListPtr<T>>{
    let root = unsafe{ root.as_mut().unwrap() };
    if let Some(RootValue::Mut(l)) = root.default_mut().get_mut(name){
        Some(MutListPtr::new(l.list_mut()))
    } else{ None }
}

pub fn get_param<'a>(def : &'a HashM<String, RootValue>, sab : &'a HashM<String, RustParam>, name : &str) -> Option<&'a RustParam>{
    if let Some(RootValue::Param(p,_v)) = def.get(name){
        if let Some(p) = sab.get(name){
            Some(p)
        } else{
            Some(p)
        }
    } else { None }
}

pub fn set_bool(root : RootObjectPtr, name : &str, val : Qv<bool>) -> bool{
    let root = unsafe{ root.ptr.as_mut().unwrap() };
    match root.set_sabun(name.to_string(), RustParam::Bool(val)){
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn set_float(root : RootObjectPtr, name : &str, val : Qv<f64>) -> bool{
    let root = unsafe{ root.ptr.as_mut().unwrap() };
    match root.set_sabun(name.to_string(), RustParam::Float(val)){
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn set_int(root : RootObjectPtr, name : &str, val : Qv<i64>) -> bool{
    let root = unsafe{ root.ptr.as_mut().unwrap() };
    match root.set_sabun(name.to_string(), RustParam::Int(val)){
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn set_str(root : RootObjectPtr, name : &str, val : Qv<String>) -> bool{
    let root = unsafe{ root.ptr.as_mut().unwrap() };
    match root.set_sabun(name.to_string(), RustParam::String(val.map(|s| RustString::new(s.to_string())))){
        Ok(_) => true,
        Err(_) => false,
    }
}