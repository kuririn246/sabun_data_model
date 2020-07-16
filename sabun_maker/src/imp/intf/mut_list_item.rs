use crate::imp::structs::rust_list::MutListItem;
//use crate::imp::intf::inner_data::InnerDataPtr;
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MutListItemPtr {
    item : *mut MutListItem,
    list_def : *const ListDefObj,
    root : *mut RootObject,
}

impl MutListItemPtr {
    pub fn new(item : *mut MutListItem, list_def : *const ListDefObj, root : *mut RootObject) -> MutListItemPtr {
        MutListItemPtr { item, list_def, root }
    }
    pub fn item(&self) -> *const MutListItem{ self.item }
    pub fn list_def(&self) -> *const ListDefObj{ self.list_def }
}

// pub fn get_inner_data(ps : MutListItemPtrs, name : &str) -> Option<InnerDataPtr>{
//     let (item, list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
//     if let Some(ListDefValue::InnerDataDef(def)) = list_def.default().get(name){
//         if let Some(ListSabValue::InnerData(data)) = item.values().get(name){
//             return Some(InnerDataPtr::new(data, def, ps.root))
//         }
//     }
//     None
// }

pub fn get_bool(ps : MutListItemPtr, name : &str) -> Option<Qv<bool>>{
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

pub fn set_bool(ps : MutListItemPtr, name : &str, val : Qv<bool>) -> bool{
    let (item,def) = unsafe{ (ps.item.as_mut().unwrap(),ps.list_def.as_ref().unwrap()) };
    match item.set_sabun(def, name.to_string(), RustParam::Bool(val)) {
        Ok(_) => true,
        Err(_) => false,
    }
}