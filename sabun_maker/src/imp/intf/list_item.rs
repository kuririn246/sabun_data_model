use crate::imp::structs::def_obj::ListDefObj;
use crate::imp::structs::rust_list::{InnerData, ListItem};
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InnerDataPtrs{
    pub data : *const InnerData,
    pub def : *const ListDefObj,
}

pub fn get_inner_data(item : *const ListItem, def : *const ListDefObj, name : &str) -> Option<InnerDataPtrs>{
    let item = unsafe{ item.as_ref().unwrap() };
    if let Some(ListDefValue::InnerDataDef(def)) = unsafe{ (*def).default().get(name) }{
        if let Some(ListSabValue::InnerData(d)) = item.values().get(name){
            Some(InnerDataPtrs{ data : d as *const InnerData, def : def as *const ListDefObj })
        }
    }
    None
}