use crate::imp::structs::rust_list::{InnerTemplate};
use crate::imp::intf::const_item::ConstItemPtr;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::intf::temp::{get_len_impl, get_value_impl};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InnerTempPtr {
    ptr : *const InnerTemplate,
    def : *const ListDefObj,
    root : *mut RootObject,
}
impl InnerTempPtr {
    pub fn new(ptr : *const InnerTemplate, def : *const ListDefObj, root : *mut RootObject) -> InnerTempPtr { InnerTempPtr { ptr, def, root } }
}

pub fn get_len(list: InnerTempPtr) -> usize{
    let d = unsafe{ list.ptr.as_ref().unwrap()};
    get_len_impl(d.list())
}

pub fn get_value(list: InnerTempPtr, idx : usize) -> Option<ConstItemPtr>{
    let d = unsafe{ (list.ptr.as_ref().unwrap())};
    get_value_impl(d.list(), list.def, idx, list.root)
}

