use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;
use std::collections::HashSet;
use crate::structs::root_object::ListDefObj;
use crate::structs::rust_list::{ConstList, ListItem};

pub struct TmpList{
    pub vec : Vec<TmpObj>,
    pub old : HashSet<String>,
    pub default : ListDefObj,
    pub compatible : HashSet<String>,
}

impl TmpList{
    pub fn new() -> TmpList{
        TmpList{ vec : vec![], old : HashSet::new(), default : ListDefObj::new(), compatible : HashSet::new() }
    }

    pub fn to_const_list(self) -> ConstList{
        ConstList{ default : self.default, compatible : self.compatible, list }
    }


}

fn to_list(vec : Vec<TmpObj>) -> Vec<ListItem>{
    vec.into_iter().map(|tmp| )
}