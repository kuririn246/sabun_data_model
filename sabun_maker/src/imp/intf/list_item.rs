use crate::imp::structs::def_obj::{ListDefObj};
use crate::imp::structs::rust_list::{ListItem};
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
use crate::imp::intf::inner_data::InnerDataPtrs;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ListItemPtrs{
    item : *const ListItem,
    list_def : *const ListDefObj,
}

impl ListItemPtrs{
    pub(crate) fn new(item : *const ListItem, list_def : *const ListDefObj) -> ListItemPtrs{ ListItemPtrs{ item, list_def }}
    pub fn item(&self) -> *const ListItem{ self.item }
    pub fn list_def(&self) -> *const ListDefObj{ self.list_def }
}

pub fn get_inner_data(ps : ListItemPtrs, name : &str) -> Option<InnerDataPtrs>{
    let (item, list_def) = unsafe{ (ps.item.as_ref().unwrap(), ps.list_def.as_ref().unwrap()) };
    if let Some(ListDefValue::InnerDataDef(def)) = list_def.default().get(name){
        if let Some(ListSabValue::InnerData(data)) = item.values().get(name){
            return Some(InnerDataPtrs::new(data, def))
        }
    }
    None
}