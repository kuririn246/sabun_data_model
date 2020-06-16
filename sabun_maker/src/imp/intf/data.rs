use crate::imp::structs::rust_list::ConstData;
use crate::imp::intf::member_desc::{MemberDesc, get_list_def_desc, get_ref_def_desc, RefDesc};

pub fn get_member_desc(root : *const ConstData) -> Vec<MemberDesc>{
    let root = unsafe{ root.as_ref().unwrap() };
    get_list_def_desc(root.default())
}

pub fn get_ref_desc(root : *const ConstData) -> Vec<RefDesc>{
    let root = unsafe{ root.as_ref().unwrap() };
    get_ref_def_desc(root.default().refs())
}

pub fn get_ids(root : *const ConstData) -> Vec<String>{
    let root = unsafe{ root.as_ref().unwrap() };
    root.list().keys().map(|s| s.to_string()).collect()
}

