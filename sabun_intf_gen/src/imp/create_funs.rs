use sabun_maker::intf::member_desc::{MemberDesc};
use sabun_maker::structs::{RustMemberType, VarType};

use crate::imp::fun::{Arg, Contents, Fun, GetC, SetC};

pub fn create_funs(mems : &[MemberDesc], is_mut : bool) -> Vec<Fun> {
    let mut funs : Vec<Fun> = vec![];
    for mem in mems {
        match mem.member_type() {
            RustMemberType::Bool => {
                funs.push(fun_get(mem.name(), "bool", "bool", mem.var_type()));
                if is_mut{
                    funs.push(fun_set(mem.name(), "bool", "bool", mem.var_type()))
                }
            },
            _=>{},
        }
    }

    funs
}

fn fun_get(mem_name : &str, result_type : &str, type_name_small : &str, vt : &VarType) -> Fun{
    Fun::new(mem_name, vec![], &with_var(result_type, vt), false, Contents::Get(GetC::new(type_name_small, vt.clone())))
}

fn fun_set(mem_name : &str, arg_type : &str, type_name_small : &str, vt : &VarType) -> Fun{
    Fun::new(
        &format!("set_{}", mem_name),
        vec![Arg::new(mem_name, &with_var(arg_type, vt))],
        "", true, Contents::Set(
            SetC::new(type_name_small, mem_name, vt.clone())))
}

fn with_var(t : &str, v : &VarType) -> String{
    match v{
        VarType::Normal => t.to_string(),
        VarType::Nullable => format!("NullOr<{}>", t),
        VarType::Undefiable => format!("UndefOr<{}>", t),
        VarType::UndefNullable => format!("Qv<{}>", t),
    }
}