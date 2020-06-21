
use sabun_maker::structs::RootObject;
use sabun_maker::intf::*;
use sabun_maker::structs::RustMemberType;

pub fn generate_interface(root : &RootObject) -> String{
    let mem_descs = root::get_member_desc(root);
    for mem_desc in &mem_descs{
        match mem_desc.member_type(){
            RustMemberType::
        }
    }
}