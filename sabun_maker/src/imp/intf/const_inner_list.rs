use crate::imp::structs::rust_list::{ConstInnerList};
use crate::imp::intf::const_item::CItemPtr;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::intf::temp::{get_len_impl, get_value_impl};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CilPtr {
    ptr : *const ConstInnerList,
    def : *const ListDefObj,
    root : *mut RootObject,
}
impl CilPtr {
    pub fn new(ptr : *const ConstInnerList, def : *const ListDefObj, root : *mut RootObject) -> CilPtr { CilPtr { ptr, def, root } }
}

pub fn get_len(list: CilPtr) -> usize{
    let d = unsafe{ list.ptr.as_ref().unwrap()};
    get_len_impl(d.list())
}

pub fn get_value(list: CilPtr, idx : usize) -> Option<CItemPtr>{
    let d = unsafe{ (list.ptr.as_ref().unwrap())};
    get_value_impl(d.list(), list.def, idx, list.root)
}

