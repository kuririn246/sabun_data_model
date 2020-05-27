use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;
use crate::structs::root_object::RootObject;
use std::collections::HashMap;

pub fn tmp_to_root(tmp : TmpObj) -> RootObject{
    RootObject {
        include: tmp.include,
        default: tmp.default,
        sabun: HashMap::new(),
        old: tmp.old.iter().collect(),
    }
}