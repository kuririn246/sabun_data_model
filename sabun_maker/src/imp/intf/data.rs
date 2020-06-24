use crate::imp::structs::rust_list::{ConstData, ListItem};
use crate::{HashM, HashS};
use crate::imp::intf::list_item::ListItemPtrs;
use crate::imp::structs::list_def_obj::ListDefObj;

// pub fn get_member_desc(root : *const ConstData) -> MemberDescs{
//     let root = unsafe{ root.as_ref().unwrap() };
//     get_list_def_desc(root.default())
// }

// pub fn get_ref_desc(root : *const ConstData) -> RefDescs{
//     let root = unsafe{ root.as_ref().unwrap() };
//     get_ref_def_desc(root.default().refs())
// }

#[derive(Debug, PartialEq, Clone)]
pub struct DataKV {
    is_old : bool,
    id : String,
    item : *const ListItem,
}

impl DataKV {
    ///utf-8とcstringの相互変換はcheckなしで安全に可能だよね・・・？
    pub(crate) fn new(is_old : bool, id : String, item : *const ListItem) -> DataKV { DataKV { is_old, id, item }}
    pub fn is_old(&self) -> bool { self.is_old }
    pub fn id(&self) -> &str{ self.id.as_str() }
    pub fn item(&self) -> *const ListItem{ self.item }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataKVs {
    items : Vec<DataKV>,
    list_def : *const ListDefObj,
}

impl DataKVs {
    pub(crate) fn new(items : Vec<DataKV>, list_def : *const ListDefObj) -> DataKVs { DataKVs { items, list_def }}
    pub fn items(&self) -> &[DataKV]{ &self.items }
    pub fn def(&self) -> *const ListDefObj{ self.list_def }
}

pub fn get_kvs(data : *const ConstData) -> DataKVs {
    let data = unsafe{ data.as_ref().unwrap() };
    get_kvs_impl(data.default(), data.list(), data.old())
}

pub fn get_kvs_impl(list_def : &ListDefObj, data : &HashM<String, ListItem>, old : &HashS<String>) -> DataKVs {
    DataKVs::new(data.iter().map(|(k,v)|
        DataKV::new(old.contains(k), k.to_string(), v)).collect(),
                 list_def)
}

pub fn get_value(data : *const ConstData, id : &str) -> Option<ListItemPtrs>{
    let data = unsafe{data.as_ref().unwrap()};
    get_value_impl(data.list(), data.default(), id)
}

pub fn get_value_impl(data : &HashM<String, ListItem>, list_def : &ListDefObj, id : &str) -> Option<ListItemPtrs>{
    data.get(id).map(|i| ListItemPtrs::new(i, list_def))
}

