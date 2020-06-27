use sabun_maker::intf::member_desc::KeyItem;
use crate::imp::structs::struct_desc::{ColType};

pub struct StructTemp {
    pub(crate) funs : Vec<String>,
    pub(crate) proxies : Vec<String>,
    pub(crate) self_mod_name : String,
    pub(crate) struct_name : String,
    pub(crate) ptr_type : String,
}

