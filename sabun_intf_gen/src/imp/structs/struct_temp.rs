use crate::imp::structs::fun::Fun;
use sabun_maker::intf::member_desc::KeyItem;
use crate::imp::structs::struct_desc::ColType;

pub struct StructTemp {
    pub(crate) funs : Vec<Fun>,
    pub(crate) self_mod_name : String,
    pub(crate) struct_name : String,
    pub(crate) ptr_type : String,
    pub(crate) proxies : Vec<Proxy>,
}

pub struct Proxy{
    pub(crate) name : String,
    pub(crate) value_type : String,
}
impl Proxy{
    pub fn new(name : &str, value_type : &str) -> Proxy{
        Proxy{ name : name.to_string(), value_type : value_type.to_string() }
    }
}

pub struct ColTemp{
    pub(crate) self_mod_name : String,
    pub(crate) struct_name : String,
    pub(crate) ptr_type : String,
    pub(crate) keys : Vec<KeyItem>,
    pub(crate) col_type : ColType,
    pub(crate) item_struct_name: String,
}