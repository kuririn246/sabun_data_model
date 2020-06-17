use crate::imp::structs::rust_list::ConstData;
use crate::imp::intf::member_desc::{MemberDesc, get_list_def_desc, get_ref_def_desc, RefDesc, RefDescs};

pub fn get_member_desc(root : *const ConstData) -> Vec<MemberDesc>{
    let root = unsafe{ root.as_ref().unwrap() };
    get_list_def_desc(root.default())
}

pub fn get_ref_desc(root : *const ConstData) -> RefDescs{
    let root = unsafe{ root.as_ref().unwrap() };
    get_ref_def_desc(root.default().refs())
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdItem{
    pub is_old : bool,
    pub id : String,
}

impl IdItem{
    pub fn new()
}

pub fn get_ids(root : *const ConstData) -> Vec<IdItem>{
    let root = unsafe{ root.as_ref().unwrap() };
    let old = root.old();
    root.list().keys().map(|s| IdItem{ is_old : old.contains(s), id : s.to_string()}).collect()
}

