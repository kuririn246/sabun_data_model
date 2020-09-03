use crate::imp::structs::rust_list::{ListItem, ConstList};
use crate::imp::intf::list_item::ListItemPtr;
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
pub struct ConstListPtr{
    ptr : *const ConstList,
    root : *mut RootObject,
}
impl ConstListPtr{
    pub fn new(ptr : *const ConstList, root : *mut RootObject) -> ConstListPtr{ ConstListPtr{ ptr, root } }
}

pub fn get_len(list: ConstListPtr) -> usize{
    let d = unsafe{ list.ptr.as_ref().unwrap()};
    get_len_impl(d.list())
}

pub fn get_len_impl(list: *const Vec<ListItem>) -> usize{
    let d = unsafe{ list.as_ref().unwrap()};
    d.len()
}


pub fn get_value(list: ConstListPtr, idx : usize) -> Option<ListItemPtr>{
    let d = unsafe{ list.ptr.as_ref().unwrap()};
    get_value_impl(d.list(), d.default(), idx, list.root)
}

pub fn get_value_impl(vec: *const Vec<ListItem>, list_def : *const ListDefObj, idx : usize, root : *mut RootObject) -> Option<ListItemPtr>{
    let vec = unsafe{ vec.as_ref().unwrap() };
    vec.get(idx).map(|item| ListItemPtr::new(item, list_def, root))
}
