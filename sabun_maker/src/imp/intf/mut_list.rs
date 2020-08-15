use crate::imp::structs::rust_list::{MutList, MutListItem};
use crate::imp::intf::mut_list_item::MutListItemPtr;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::linked_m::LinkedMap;

// pub fn get_member_desc(l : *const MutList) -> MemberDescs{
//     let l = unsafe{ l.as_ref().unwrap() };
//     get_list_def_desc(l.default())
// }
//
// pub fn get_ref_desc(l : *const MutList) -> RefDescs{
//     let l = unsafe{ l.as_ref().unwrap() };
//     get_ref_def_desc(l.default().refs())
// }
//
// #[repr(C)]
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct MutValue {
//     id : u64,
//     item : *mut MutListItem,
// }
//
// impl MutValue {
//     pub fn new(id : u64, item : *mut MutListItem) -> MutValue { MutValue { id, item }}
//     pub fn id(&self) -> u64{ self.id }
//     pub fn item(&self) -> *const MutListItem{ self.item }
// }
//
// #[derive(Debug, PartialEq, Clone)]
// pub struct MutValues {
//     items : Vec<MutValue>,
//     list_def : *const ListDefObj,
// }
//
// impl MutValues {
//     pub(crate) fn new(items : Vec<MutValue>, list_def : *const ListDefObj) -> MutValues { MutValues { items, list_def }}
// }
//
// pub fn get_values(l : *mut MutList) -> MutValues {
//     let l = unsafe{ l.as_ref().unwrap() };
//     get_values_impl(l.default(), l.list())
// }
//
// pub fn get_values_impl(list_def : &ListDefObj, data : &LinkedHashMap<u64, MutListItem>) -> MutValues {
//     MutValues::new(data.iter().map(|(k,v)|
//         MutValue::new(*k, v as *const MutListItem as *mut MutListItem)).collect(), list_def)
// // }
//
// #[repr(C)]
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct MutListPtr{
//     ptr : *mut MutList,
//     root : *mut RootObject,
// }
// impl MutListPtr {
//     pub fn new(ptr: *mut MutList, root: *mut RootObject) -> MutListPtr { MutListPtr { ptr, root } }
// }
//
// pub fn get_value(lp : MutListPtr, id : u64) -> Option<MutListItemPtr>{
//     let l = unsafe{ lp.ptr.as_mut().unwrap() };
//     let (default, list, _) = l.distribute_mut();
//     get_value_impl(list, default, id, lp.root)
// }
//
// pub fn get_value_impl(data : &mut LinkedMap<MutListItem>, list_def : &ListDefObj, id : u64, root : *mut RootObject) -> Option<MutListItemPtr>{
//
//     data.from_id_mut(id).map(|i| MutListItemPtr::new(i as *mut MutListItem, list_def, root))
// }
//
// pub fn append_item(lp : MutListPtr) -> Option<MutListItemPtr>{
//     let l = unsafe{ lp.ptr.as_mut().unwrap() };
//     let id = l.append_new_item();
//     get_value(lp, id)
// }

// pub fn arrange(l : *mut MutList, ids : &[u64]) -> bool{
//     let l = unsafe{l.as_mut().unwrap() };
//     l.arrange(ids)
// }