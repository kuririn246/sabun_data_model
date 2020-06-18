use crate::imp::structs::rust_list::{MutList, MutListItem};
use crate::imp::intf::member_desc::{MemberDescs, get_list_def_desc};
use crate::imp::intf::ref_desc::{RefDescs, get_ref_def_desc};
use crate::imp::structs::def_obj::ListDefObj;
use linked_hash_map::LinkedHashMap;
use crate::imp::intf::mut_list_item::MutListItemPtrs;
use std::collections::HashMap;

pub fn get_member_desc(l : *const MutList) -> MemberDescs{
    let l = unsafe{ l.as_ref().unwrap() };
    get_list_def_desc(l.default())
}

pub fn get_ref_desc(l : *const MutList) -> RefDescs{
    let l = unsafe{ l.as_ref().unwrap() };
    get_ref_def_desc(l.default().refs())
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MutValue {
    id : u64,
    item : *mut MutListItem,
}

impl MutValue {
    pub fn new(id : u64, item : *mut MutListItem) -> MutValue { MutValue { id, item }}
    pub fn id(&self) -> u64{ self.id }
    pub fn item(&self) -> *const MutListItem{ self.item }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MutValues {
    items : Vec<MutValue>,
    list_def : *const ListDefObj,
}

impl MutValues {
    pub(crate) fn new(items : Vec<MutValue>, list_def : *const ListDefObj) -> MutValues { MutValues { items, list_def }}
}

pub fn get_values(l : *mut MutList) -> MutValues {
    let l = unsafe{ l.as_ref().unwrap() };
    get_values_impl(l.default(), l.list())
}

pub fn get_values_impl(list_def : &ListDefObj, data : &LinkedHashMap<u64, MutListItem>) -> MutValues {
    MutValues::new(data.iter().map(|(k,v)|
        MutValue::new(*k, v as *const MutListItem as *mut MutListItem)).collect(), list_def)
}

pub fn get_value(l : *mut MutList, id : u64) -> Option<MutListItemPtrs>{
    let l = unsafe{ l.as_ref().unwrap() };
    get_value_impl(l.list(), l.default(), id)
}

pub fn get_value_impl(data : &LinkedHashMap<u64, MutListItem>, list_def : &ListDefObj, id : u64) -> Option<MutListItemPtrs>{
    data.get(&id).map(|i| MutListItemPtrs::new(i as *const MutListItem as *mut MutListItem, list_def))
}

pub fn append_item(l : *mut MutList) -> Option<MutListItemPtrs>{
    let l = unsafe{ l.as_mut().unwrap() };
    let id = l.append_new_item();
    get_value(l, id)
}

pub fn arrange(l : *mut MutList, ids : &[u64]) -> bool{
    let l = unsafe{l.as_mut().unwrap() };
    l.arrange(ids)
}