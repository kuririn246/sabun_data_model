use sabun_maker::intf::member_desc::KeyItem;
use crate::imp::structs::struct_desc::{ColType};

pub struct StructTemp {
    pub(crate) funs : Vec<String>,
    pub(crate) proxies : Vec<String>,
    pub(crate) self_mod_name : String,
    pub(crate) struct_name : String,
    pub(crate) ptr_type : String,
}

pub struct ColTemp{
    pub(crate) self_mod_name : String,
    pub(crate) struct_name : String,
    pub(crate) ptr_type : String,
    pub(crate) keys : Vec<KeyItem>,
    pub(crate) col_type : ColType,
    pub(crate) item_struct_name: String,
}