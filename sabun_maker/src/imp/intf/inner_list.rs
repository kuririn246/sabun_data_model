use crate::imp::structs::rust_list::{InnerList};
use crate::imp::intf::list_item::ListItemPtr;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::intf::list::{get_len_impl, get_value_impl};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InnerListPtr{
    ptr : *const InnerList,
    def : *const ListDefObj,
    root : *mut RootObject,
}
impl InnerListPtr{
    pub fn new(ptr : *const InnerList, def : *const ListDefObj, root : *mut RootObject) -> InnerListPtr{ InnerListPtr{ ptr, def, root } }
}

pub fn get_len(list: InnerListPtr) -> usize{
    let d = unsafe{ list.ptr.as_ref().unwrap()};
    get_len_impl(d.list())
}

pub fn get_value(list: InnerListPtr, idx : usize) -> Option<ListItemPtr>{
    let d = unsafe{ (list.ptr.as_ref().unwrap())};
    get_value_impl(d.list(), list.def, idx, list.root)
}

