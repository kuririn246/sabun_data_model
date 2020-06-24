use crate::imp::intf::data::{DataKVs, get_kvs_impl, get_value_impl};
use crate::imp::structs::rust_list::InnerData;
use crate::imp::intf::list_item::ListItemPtrs;
use crate::imp::structs::list_def_obj::ListDefObj;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InnerDataPtrs{
    data : *const InnerData,
    list_def : *const ListDefObj,
}

impl InnerDataPtrs{
    pub fn new(data : *const InnerData, list_def : *const ListDefObj) -> InnerDataPtrs{ InnerDataPtrs{ data, list_def }}
    pub fn data(&self) -> *const InnerData{ self.data }
    pub fn list_def(&self) -> *const ListDefObj{ self.list_def }
}

pub fn get_values(ps: InnerDataPtrs) -> DataKVs {
    let (data,list_def) = unsafe{ (ps.data.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    get_kvs_impl(list_def, data.list(), data.old())
}

pub fn get_value(ps : InnerDataPtrs, id : &str) -> Option<ListItemPtrs>{
    let (data, list_def) = unsafe{ (ps.data.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    get_value_impl(data.list(), list_def, id)
}

// pub fn get_member_desc(ps : InnerDataPtrs) -> MemberDescs{
//     let list_def = unsafe{ ps.list_def.as_ref().unwrap() };
//     get_list_def_desc(list_def)
// }
//
// pub fn get_ref_desc(ps : InnerDataPtrs) -> RefDescs{
//     let ref_def = unsafe{ ps.list_def.as_ref().unwrap().refs() };
//     get_ref_def_desc(ref_def)
// }