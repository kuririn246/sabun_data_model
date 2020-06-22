
use sabun_maker::structs::RootObject;
use sabun_maker::intf::*;
use sabun_maker::structs::RustMemberType;
use crate::imp::generate_members::generate_members;

pub fn generate_interface(root : &RootObject) -> String{
    let mem_descs = root::get_member_desc(root);
    generate_members(&mem_descs, true)
}