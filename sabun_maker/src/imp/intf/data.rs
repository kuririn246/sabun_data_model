use crate::imp::structs::rust_list::{ConstData, ListItem, InnerData};
use crate::imp::intf::member_desc::{MemberDesc, get_list_def_desc, get_ref_def_desc, RefDesc, RefDescs, MemberDescs};
use crate::imp::structs::def_obj::ListDefObj;
use crate::imp::intf::list_item::InnerDataPtrs;

pub fn get_member_desc(root : *const ConstData) -> MemberDescs{
    let root = unsafe{ root.as_ref().unwrap() };
    get_list_def_desc(root.default())
}

pub fn get_ref_desc(root : *const ConstData) -> RefDescs{
    let root = unsafe{ root.as_ref().unwrap() };
    get_ref_def_desc(root.default().refs())
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataItem {
    is_old : bool,
    id : String,
    item : *const ListItem,
}

impl DataItem {
    pub fn new(is_old : bool, id : String, item : *const ListItem) -> DataItem { DataItem { is_old, id, item }}
    pub fn is_old(&self) -> bool { self.is_old }
    pub fn id(&self) -> &str{ &self.id }
    pub fn item(&self) -> *const ListItem{ self.item }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataItems {
    items : Vec<DataItem>
}

impl DataItems {
    pub fn new(items : Vec<DataItem>) -> DataItems { DataItems { items }}
}

pub fn get_values(data : *const ConstData) -> DataItems {
    let data = unsafe{ data.as_ref().unwrap() };
    let old = data.old();
    DataItems::new(data.list().iter().map(|(k,v)|
        DataItem::new(old.contains(k), k.to_string(), v as *const ListItem)).collect())
}

pub fn get_values_inner(data : InnerDataPtrs) -> DataItems{
    let data = unsafe{ data.data.as_ref().unwrap() };
    let old = data.old();
    DataItems::new(data.list().iter().map(|(k,v)|
        DataItem::new(old.contains(k), k.to_string(), v as *const ListItem)).collect())
}

