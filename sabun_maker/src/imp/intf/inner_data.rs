use crate::imp::intf::data::{DataItems, get_values_impl};
use crate::imp::intf::member_desc::{MemberDescs, get_list_def_desc};
use crate::imp::intf::ref_desc::{get_ref_def_desc, RefDescs};
use crate::imp::structs::rust_list::InnerData;
use crate::imp::structs::def_obj::ListDefObj;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InnerDataPtrs{
    data : *const InnerData,
    list_def : *const ListDefObj,
}

impl InnerDataPtrs{
    pub(crate) fn new(data : *const InnerData, list_def : *const ListDefObj) -> InnerDataPtrs{ InnerDataPtrs{ data, list_def }}
    pub fn data(&self) -> *const InnerData{ self.data }
    pub fn list_def(&self) -> *const ListDefObj{ self.list_def }
}

pub fn get_values_inner(data : InnerDataPtrs) -> DataItems{
    let (data,list_def) = unsafe{ (data.data.as_ref().unwrap(), data.list_def.as_ref().unwrap()) };
    get_values_impl(list_def, data.list(), data.old())
}

pub fn get_member_desc(ps : InnerDataPtrs) -> MemberDescs{
    let list_def = unsafe{ ps.list_def.as_ref().unwrap() };
    get_list_def_desc(list_def)
}

pub fn get_ref_desc(ps : InnerDataPtrs) -> RefDescs{
    let ref_def = unsafe{ ps.list_def.as_ref().unwrap().refs() };
    get_ref_def_desc(ref_def)
}