use crate::imp::structs::rust_list::{ListItem};
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
use crate::imp::intf::inner_data::InnerDataPtrs;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::qv::Qv;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ListItemPtr {
    item : *const ListItem,
    list_def : *const ListDefObj,
    root : *const RootObject,
}

impl ListItemPtr {
    pub fn new(item : *const ListItem, list_def : *const ListDefObj, root : *const RootObject) -> ListItemPtr { ListItemPtr { item, list_def, root }}
    pub fn item(&self) -> *const ListItem{ self.item }
    pub fn list_def(&self) -> *const ListDefObj{ self.list_def }
}

pub fn get_inner_data(ps : ListItemPtr, name : &str) -> Option<InnerDataPtrs>{
    let (item, list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(ListDefValue::InnerDataDef(def)) = list_def.default().get(name){
        if let Some(ListSabValue::InnerData(data)) = item.values().get(name){
            return Some(InnerDataPtrs::new(data, def, ps.root))
        }
    }
    None
}

pub fn get_bool(ps : ListItemPtr, name : &str) -> Option<Qv<bool>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::Bool(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_num(ps : ListItemPtr, name : &str) -> Option<Qv<f64>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::Number(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_param<'a>(item : &'a ListItem, def : &'a ListDefObj, name : &str) -> Option<&'a RustParam>{
    if let Some(ListSabValue::Param(p)) = item.values().get(name){
        Some(p)
    } else if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
        Some(p)
    } else{
        None
    }
}

pub fn get_ref(ps : ListItemPtr, list_name : &str) -> Option<&Qv<String>>{
    let (item, list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    let qv = if let Some(sab) = item.refs().get(list_name){
        sab.value()
    } else{
        if let Some(d) = list_def.refs().refs().get(list_name){
            d.value()
        } else{ return None; }
    };
    Some(qv)
}