use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::qv::Qv;
use std::collections::HashMap;
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_list::{ConstData, ConstList, MutList};
use crate::imp::intf::member_desc::MemberDesc;
use crate::imp::structs::rust_value::RustValueType;
use crate::imp::structs::value_type::ValueType;

pub fn get_bool(root : *const RootObject, name : &str) -> Option<Qv<bool>>{
    let root = unsafe{ root.as_ref().unwrap() };
    if let Some(RustParam::Bool(b)) = get_param(root.default(), root.sabun(), name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_data(root : *const RootObject, name : &str) -> Option<*const ConstData>{
    let root = unsafe{ root.as_ref().unwrap() };
    if let Some(RootValue::Data(d)) = root.default().get(name){
        Some(d as *const ConstData)
    } else{ None }
}

pub fn get_list(root : *const RootObject, name : &str) -> Option<*const ConstList>{
    let root = unsafe{ root.as_ref().unwrap() };
    if let Some(RootValue::List(l)) = root.default().get(name){
        Some(l as *const ConstList)
    } else{ None }
}

pub fn get_mut(root : *mut RootObject, name : &str) -> Option<*mut MutList>{
    let root = unsafe{ root.as_ref().unwrap() };
    if let Some(RootValue::Mut(l)) = root.default().get(name){
        Some(l as *const MutList as *mut MutList)
    } else{ None }
}

pub fn get_member_desc(root : *const RootObject) -> Vec<MemberDesc>{
    let root = unsafe{ root.as_ref().unwrap() };
    let mut vec : Vec<MemberDesc> = Vec::with_capacity(root.default().len());
    for (k,val) in root.default(){
        let mem = k.to_string();
        let is_old = root.old().contains(k);
        let mt = match val{
            RootValue::Param(p, vt) =>{
                vec.push(MemberDesc::new(mem, vt.clone(), p.type_num(), is_old));
                continue;
            },
            RootValue::Data(_) => RustValueType::Data,
            RootValue::List(_) => RustValueType::List,
            RootValue::Mut(_) => RustValueType::Mut,
        };
        vec.push(MemberDesc::new(mem, ValueType::Normal, mt, is_old));
    }
    vec
}

pub fn get_param<'a>(def : &'a HashMap<String, RootValue>, sab : &'a HashMap<String, RustParam>, name : &str) -> Option<&'a RustParam>{
    if let Some(RootValue::Param(p,_v)) = def.get(name){
        if let Some(p) = sab.get(name){
            Some(p)
        } else{
            Some(p)
        }
    } else { None }
}

pub fn set_bool(root : *mut RootObject, name : &str, val : Qv<bool>) -> bool{
    let root = unsafe{ root.as_mut().unwrap() };
    match root.set_sabun(name.to_string(), RustParam::Bool(val)){
        Ok(_) => true,
        Err(_) => false,
    }
}
