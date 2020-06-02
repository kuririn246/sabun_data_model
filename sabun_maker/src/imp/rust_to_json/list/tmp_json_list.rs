use crate::structs::rust_list::ConstList;
use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;
use std::collections::HashSet;
use crate::structs::root_object::ListDefObj;

pub struct TmpJsonList{
    pub vec : Vec<TmpObj>,
    pub old : Option<HashSet<String>>,
    pub default : Option<ListDefObj>,
    pub compatible : Option<HashSet<String>>,
    pub next_id : Option<u64>,
}

impl TmpJsonList{
    pub fn from_const_list(l : &ConstList) -> TmpJsonList{

        TmpJsonList{ vec: l.list.map(), compatible : None, next_id: None, old : None, default : Some(l.default.as_ref().clone()) }
    }
}