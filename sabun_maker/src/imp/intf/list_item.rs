use crate::imp::structs::def_obj::{ListDefObj, RefDefObj};
use crate::imp::structs::rust_list::{InnerData, ListItem};
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InnerDataPtrs{
    pub data : *const InnerData,
    pub list_def : *const ListDefObj,
    pub ref_def : *const RefDefObj,
}

pub fn get_inner_data(item : *const ListItem, list_def : *const ListDefObj, name : &str) -> Option<InnerDataPtrs>{
    let (item, list_def) = unsafe{ (item.as_ref().unwrap(), list_def.as_ref().unwrap()) };
    if let Some(ListDefValue::InnerDataDef(def)) = unsafe{ list_def.default().get(name) }{
        if let Some(ListSabValue::InnerData(data)) = item.values().get(name){
            return Some(InnerDataPtrs{ data, list_def, ref_def : list_def.refs() })
        }
    }
    None
}