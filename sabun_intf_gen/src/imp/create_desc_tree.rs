use sabun_maker::intf::member_desc::{MemberDesc};
use sabun_maker::structs::{RustMemberType, VarType};
use crate::imp::structs::fun::{Fun, Proxy, Contents, GetC, Arg, SetC};
use crate::imp::structs::struct_desc::StructDesc;
use crate::imp::structs::source_tree::SourceTree;
use crate::imp::util::to_type_name::to_type_name;

pub fn create_desc_tree(mems : &[MemberDesc]) -> Vec<StructDesc> {
    let mut r :Vec<StructDesc> = vec![];

    for mem in mems {
        match mem.member_type() {
            RustMemberType::Data =>{
                let type_name = to_type_name(mem.name());
                let children = mem.child_descs().unwrap();
                r.push(StructDesc{
                    item_struct_name : format!("{}Item", type_name),
                    col_struct_name : format!("{}Data", type_name),
                    ptr_type : "*const ConstData".to_string(),
                    self_mod_name : "data".to_string(),
                    is_mut : false,
                    mem_descs : children.items().iter().map(|m|m.clone()).collect(),
                    keys: children.keys().unwrap().iter().map(|k| k.clone()).collect(),
                });
            }
            _=>{},
        }
    }

    r
}
