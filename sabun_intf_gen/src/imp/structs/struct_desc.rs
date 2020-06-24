use sabun_maker::intf::member_desc::{MemberDesc, KeyItem};

pub struct StructDesc{
    pub(crate) self_mod_name : String,
    pub(crate) item_struct_name : String,
    pub(crate) col_struct_name : String,
    pub(crate) ptr_type : String,
    pub(crate) mem_descs : Vec<MemberDesc>,
    pub(crate) is_mut : bool,
    pub(crate) keys : Vec<KeyItem>,
}