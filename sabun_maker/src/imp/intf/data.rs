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
    is_old : bool,
    id : String,
}

impl IdItem{
    pub fn new(is_old : bool, id : String) -> IdItem{ IdItem{ is_old, id }}
    pub fn is_old(&self) -> bool { self.is_old }
    pub fn id(&self) -> &str{ &self.id }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdItems{
    items : Vec<IdItem>
}

impl IdItems{
    pub fn new(items : Vec<IdItem>) -> IdItems{ IdItems{ items }}
}

pub fn get_ids(root : *const ConstData) -> IdItems{
    let root = unsafe{ root.as_ref().unwrap() };
    let old = root.old();
    IdItems::new(root.list().keys().map(|s| IdItem::new(old.contains(s), s.to_string())).collect())
}

