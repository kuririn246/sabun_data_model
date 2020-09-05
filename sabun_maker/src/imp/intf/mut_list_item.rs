use crate::imp::structs::rust_list::MutListItem;
//use crate::imp::intf::inner_data::InnerDataPtr;
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::intf::mut_list_ptr::MutListPtr;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MutListItemPtr {
    item : *mut MutListItem,
    list_def : *const ListDefObj,
    root : *mut RootObject,
}

impl MutListItemPtr {
    ///getだけなら &MutListItemからのポインタでもOKである。その場合setするとundefined behaviorなので、&mut からのポインタを得る必要がある
    pub fn new(item : *mut MutListItem, list_def : *const ListDefObj, root : *mut RootObject) -> MutListItemPtr {
        MutListItemPtr { item, list_def, root }
    }
    pub fn item(&self) -> *mut MutListItem{ self.item }
    pub fn list_def(&self) -> *const ListDefObj{ self.list_def }
}

pub fn get_inner_mut<T : From<MutListItemPtr>>(ps : MutListItemPtr, name : &str) -> Option<Option<MutListPtr<T>>> {
    let item = unsafe { ps.item.as_mut().unwrap() };
    if let Some(ListSabValue::InnerMut(data)) = item.values_mut().get_mut(name) {
        if let Some(inner) = data {
            return Some(Some(MutListPtr::new(inner.list_mut(), ps.list_def, ps.root)))
        } else {
            return Some(None)
        }
    }
    return None
}

pub fn get_bool(ps : MutListItemPtr, name : &str) -> Option<Qv<bool>>{
    let (item,list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(RustParam::Bool(b)) = get_param(item, list_def, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_param<'a>(item : &'a MutListItem, def : &'a ListDefObj, name : &str) -> Option<&'a RustParam>{
    if let Some(ListSabValue::Param(p)) = item.values().get(name){
        Some(p)
    } else if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
        Some(p)
    } else{
        None
    }
}

pub fn set_bool(ps : MutListItemPtr, name : &str, val : Qv<bool>) -> bool{
    let (item,def) = unsafe{ (ps.item.as_mut().unwrap(),ps.list_def.as_ref().unwrap()) };
    match item.set_sabun(def, name.to_string(), RustParam::Bool(val)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// pub fn set_ref(ps : MutListItemPtr, list_name : &str, id : &str){
//     let item= unsafe{ ps.item.as_mut().unwrap() };
//     item.refs()
//     let qv = if let Some(sab) = item.refs().get(list_name){
//         sab.value()
//     } else{
//         if let Some(d) = list_def.refs().refs().get(list_name){
//             d.value()
//         } else{ return None; }
//     };
//     Some(qv)
// }