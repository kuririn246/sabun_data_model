use sabun_maker::intf::member_desc::{MemberDesc};
use sabun_maker::structs::{RustMemberType, VarType};
use crate::imp::structs::fun::{Fun, Proxy, Contents, GetC, Arg, SetC};
use crate::imp::structs::struct_desc::StructDesc;
use crate::imp::structs::source_tree::SourceTree;


pub fn create_source_tree(mems : &StructDesc) -> SourceTree {
    let mut funs : Vec<Fun> = vec![];
    let mut proxies : Vec<Proxy> = vec![];
    let mut str_descs : Vec<StructDesc> = vec![];

    for mem in mems {
        match mem.member_type() {
            RustMemberType::Bool => {
                let proxy_name = format!("p_{}", mem.name());
                proxies.push(Proxy::new(&proxy_name, &with_var("bool", mem.var_type())));
                funs.push(fun_get(mem.name(), "bool", "bool", mem.var_type(), &proxy_name));
                if is_mut{
                    funs.push(fun_set(mem.name(), "bool", "bool", mem.var_type(), &proxy_name))
                }
            },
            RustMemberType::Data =>{
                let proxy_name = format!("p_{}", mem.name());
                let type_name = to_type_name(mem.name());
                let children = mem.child_descs().unwrap();
                str_descs.push(StructDesc{
                    item_struct_name : format!("{}Item", type_name),
                    col_struct_name : format!("{}Data", type_name),
                    ptr_type : "*const ConstData".to_string(),
                    self_mod_name : "data".to_string(),
                    is_mut : false,
                    mem_descs : children.items().iter().map(|m|m.clone()).collect(),
                    keys: children.keys().unwrap().iter().map(|k| k.clone()).collect(),
                });
                proxies.push(Proxy::new(&proxy_name, &type_name));
            }
            _=>{},
        }
    }

    (funs,proxies, str_descs)
}

fn to_type_name(s : &str) -> String{
    let mut r = String::with_capacity(s.len());
    for (i,c) in s.chars().enumerate(){
        if i == 0{
            r.push(c.to_ascii_uppercase());
        } else{
            r.push(c)
        }
    }
    r
}

fn fun_get(mem_name : &str, result_type : &str, type_name_small : &str, vt : &VarType, proxy_name : &str) -> Fun{
    Fun::new(mem_name, vec![], &with_var(result_type, vt),
             Contents::Get(GetC::new(type_name_small, vt.clone(), proxy_name)))
}

fn fun_set(mem_name : &str, arg_type : &str, type_name_small : &str, vt : &VarType, proxy_name : &str) -> Fun{
    Fun::new(
        &format!("set_{}", mem_name),
        vec![Arg::new(mem_name, &with_var(arg_type, vt))], "",
        Contents::Set(SetC::new(type_name_small, mem_name, vt.clone(), proxy_name)))
}

fn with_var(t : &str, v : &VarType) -> String{
    match v{
        VarType::Normal => t.to_string(),
        VarType::Nullable => format!("NullOr<{}>", t),
        VarType::Undefiable => format!("UndefOr<{}>", t),
        VarType::UndefNullable => format!("Qv<{}>", t),
    }
}