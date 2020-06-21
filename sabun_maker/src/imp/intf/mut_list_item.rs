use crate::imp::structs::rust_list::MutListItem;
use crate::imp::intf::inner_data::InnerDataPtrs;
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::list_def_obj::ListDefObj;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MutListItemPtrs {
    item : *mut MutListItem,
    list_def : *const ListDefObj,
}

impl MutListItemPtrs {
    pub fn new(item : *mut MutListItem, list_def : *const ListDefObj) -> MutListItemPtrs { MutListItemPtrs { item, list_def }}
    pub fn item(&self) -> *const MutListItem{ self.item }
    pub fn list_def(&self) -> *const ListDefObj{ self.list_def }
}

pub fn get_inner_data(ps : MutListItemPtrs, name : &str) -> Option<InnerDataPtrs>{
    let (item, list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(ListDefValue::InnerDataDef(def)) = list_def.default().get(name){
        if let Some(ListSabValue::InnerData(data)) = item.values().get(name){
            return Some(InnerDataPtrs::new(data, def))
        }
    }
    None
}

pub fn get_bool(ps : MutListItemPtrs, name : &str) -> Option<Qv<bool>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::Bool(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_param<'a>(item : &'a MutListItem, def : &'a ListDefObj, name : &str) -> Option<&'a RustParam>{
    if let Some(ListSabValue::Param(p)) = item.values().get(name){
        Some(p)
    } else if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
        Some(p)
    } else{
        None
    }
}

pub fn set_bool(ps : MutListItemPtrs, name : &str, val : Qv<bool>) -> bool{
    let (item,def) = unsafe{ (ps.item.as_mut().unwrap(),ps.list_def.as_ref().unwrap()) };
    match item.set_sabun(def, name.to_string(), RustParam::Bool(val)) {
        Ok(_) => true,
        Err(_) => false,
    }
}