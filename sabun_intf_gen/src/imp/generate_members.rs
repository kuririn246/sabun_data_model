use sabun_maker::intf::member_desc::{MemberDescs, MemberDesc};
use sabun_maker::structs::RustMemberType;
use crate::imp::str_and_tab::StrAndTab;
use crate::imp::fun::{Fun, Contents, GetC};

pub fn generate_members(mems : &[MemberDesc], is_mut : bool) -> Vec<Fun> {
    let mut funs : Vec<Fun> = vec![];
    for mem in mems {
        match mem_desc.member_type() {
            RustMemberType::Bool => {
                let fun = Fun::new2(mem.name(), vec![], "bool", false, Contents::Get(GetC::new("bool")));
                funs.push(fun);
                if is_mut{
                    
                }
            }
        }
    }

    funs
}