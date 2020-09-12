use crate::imp::structs::rust_list::{ConstTable, ConstItem};
use crate::{HashM, HashS};
use crate::imp::intf::const_item::CItemPtr;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;

// pub fn get_member_desc(root : *const ConstData) -> MemberDescs{
//     let root = unsafe{ root.as_ref().unwrap() };
//     get_list_def_desc(root.default())
// }

// pub fn get_ref_desc(root : *const ConstData) -> RefDescs{
//     let root = unsafe{ root.as_ref().unwrap() };
//     get_ref_def_desc(root.default().refs())
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ConstTablePtr {
    ptr : *const ConstTable,
    root : *mut RootObject,
}
impl ConstTablePtr {
    pub fn new(ptr : *const ConstTable, root : *mut RootObject) -> ConstTablePtr { ConstTablePtr { ptr, root } }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataKV {
    is_old : bool,
    id : String,
    item : *const ConstItem,
}

impl DataKV {
    pub(crate) fn new(is_old : bool, id : String, item : *const ConstItem) -> DataKV { DataKV { is_old, id, item }}
    pub fn is_old(&self) -> bool { self.is_old }
    pub fn id(&self) -> &str{ self.id.as_str() }
    pub fn item(&self) -> *const ConstItem { self.item }
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

pub fn get_kvs(data : ConstTablePtr) -> DataKVs {
    let data = unsafe{ data.ptr.as_ref().unwrap() };
    get_kvs_impl(data.default(), data.list(), data.old())
}

pub fn get_kvs_impl(list_def : &ListDefObj, data : &HashM<String, ConstItem>, old : &HashS<String>) -> DataKVs {
    DataKVs::new(data.iter().map(|(k,v)|
        DataKV::new(old.contains(k), k.to_string(), v)).collect(),
                 list_def)
}

pub fn get_value(data : ConstTablePtr, id : &str) -> Option<CItemPtr>{
    let d = unsafe{data.ptr.as_ref().unwrap()};
    get_value_impl(d.list(), d.default(), id, data.root)
}

pub fn get_value_impl(data : &HashM<String, ConstItem>, list_def : &ListDefObj, id : &str, root : *mut RootObject) -> Option<CItemPtr>{
    data.get(id).map(|i| CItemPtr::new(i, list_def, root))
}

