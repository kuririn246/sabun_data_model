use crate::imp::structs::rust_list::{ConstItem};
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
//use crate::imp::intf::inner_data::InnerDataPtr;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::qv::Qv;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::intf::RootObjectPtr;
use crate::imp::intf::const_inner_list::CilPtr;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CItemPtr {
    item : *const ConstItem,
    list_def : *const ListDefObj,
    root : *mut RootObject,
}

impl CItemPtr {
    pub fn new(item : *const ConstItem, list_def : *const ListDefObj, root : *mut RootObject) -> CItemPtr { CItemPtr { item, list_def, root }}
    pub fn item(&self) -> *const ConstItem { self.item }
    pub fn list_def(&self) -> *const ListDefObj{ self.list_def }
}

pub fn get_cil(ps : CItemPtr, name : &str) -> Option<CilPtr>{
    let (item, list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(ListDefValue::CilDef(def)) = list_def.default().get(name){
        if let Some(ListSabValue::Cil(data)) = item.values().get(name){
            return Some(CilPtr::new(data, def, ps.root))
        }
    }
    None
}

pub fn get_bool(ps : CItemPtr, name : &str) -> Option<Qv<bool>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::Bool(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_float(ps : CItemPtr, name : &str) -> Option<Qv<f64>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::Float(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_int(ps : CItemPtr, name : &str) -> Option<Qv<i64>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::Int(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_str(ps : CItemPtr, name : &str) -> Option<Qv<String>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::String(b)) = get_param(item, list_def, name){
        Some(b.map(|s| s.str().to_string()))
    } else{ None }
}

pub fn get_param<'a>(item : &'a ConstItem, def : &'a ListDefObj, name : &str) -> Option<&'a RustParam>{
    if let Some(ListSabValue::Param(p)) = item.values().get(name){
        Some(p)
    } else if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
        Some(p)
    } else{
        None
    }
}

pub fn get_ref(ps : CItemPtr, list_name : &str) -> Option<Qv<CItemPtr>>{
    let (item, list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    let qv = if let Some(sab) = item.refs().get(list_name){
        sab.value()
    } else{
        if let Some(d) = list_def.refs().refs().get(list_name){
            d.value()
        } else{ return None; }
    };
    qv.opt_map(|id|{
        let data = super::root::get_data(RootObjectPtr::new(ps.root), list_name).unwrap();
        super::table::get_value(data, id)
    })
}