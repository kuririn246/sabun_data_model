use sabun_maker::intf::member_desc::MemberDesc;

pub struct StructDesc{
    pub(crate) self_mod_name : String,
    pub(crate) struct_name : String,
    pub(crate) ptr_type : String,
    pub(crate) mem_descs : Vec<MemberDesc>,
    pub(crate) is_mut : bool,
}