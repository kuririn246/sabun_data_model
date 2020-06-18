use crate::imp::structs::rust_list::{ConstData, ListItem};
use crate::imp::intf::member_desc::{ get_list_def_desc, MemberDescs};
use crate::imp::structs::def_obj::{ListDefObj};
use crate::imp::intf::ref_desc::{get_ref_def_desc, RefDescs};
use std::collections::{HashMap, HashSet};
use crate::imp::intf::list_item::ListItemPtrs;
use std::ffi::CString;

pub fn get_member_desc(root : *const ConstData) -> MemberDescs{
    let root = unsafe{ root.as_ref().unwrap() };
    get_list_def_desc(root.default())
}

pub fn get_ref_desc(root : *const ConstData) -> RefDescs{
    let root = unsafe{ root.as_ref().unwrap() };
    get_ref_def_desc(root.default().refs())
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataValue {
    is_old : bool,
    id : String,
    item : *const ListItem,
}

impl DataValue {
    ///utf-8とcstringの相互変換はcheckなしで安全に可能だよね・・・？
    pub(crate) fn new(is_old : bool, id : String, item : *const ListItem) -> DataValue { DataValue { is_old, id, item }}
    pub fn is_old(&self) -> bool { self.is_old }
    pub fn id(&self) -> &str{ self.id.as_str() }
    pub fn item(&self) -> *const ListItem{ self.item }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataValues {
    items : Vec<DataValue>,
    list_def : *const ListDefObj,
}

impl DataValues {
    pub(crate) fn new(items : Vec<DataValue>, list_def : *const ListDefObj) -> DataValues { DataValues { items, list_def }}
}

pub fn get_values(data : *const ConstData) -> DataValues {
    let data = unsafe{ data.as_ref().unwrap() };
    get_values_impl(data.default(), data.list(), data.old())
}

pub fn get_values_impl(list_def : &ListDefObj, data : &HashMap<String, ListItem>, old : &HashSet<String>) -> DataValues {
    DataValues::new(data.iter().map(|(k,v)|
        DataValue::new(old.contains(k), k.to_string(), v)).collect(),
                    list_def)
}

pub fn get_value(data : *const ConstData, id : &str) -> Option<ListItemPtrs>{
    let data = unsafe{data.as_ref().unwrap()};
    get_value_impl(data.list(), data.default(), id)
}

pub fn get_value_impl(data : &HashMap<String, ListItem>, list_def : &ListDefObj, id : &str) -> Option<ListItemPtrs>{
    data.get(id).map(|i| ListItemPtrs::new(i, list_def))
}

