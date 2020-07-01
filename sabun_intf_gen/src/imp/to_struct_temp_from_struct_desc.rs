use crate::imp::structs::struct_desc::{StructDesc, RefItem, ParamItem, ParamType};
use crate::imp::structs::struct_temp::{StructTemp};
use sabun_maker::structs::VarType;
use crate::imp::util::to_type_name::{to_item_name, to_type_name, to_item_type_name};
use crate::imp::fun_get::param::get_fun_string;
use crate::imp::fun_get::col::get_col_fun_string;
use crate::imp::fun_set::param_fun_set::fun_set;
use crate::imp::fun_get::refs::get_ref_fun_string;

pub fn to_struct_temp_from_struct_desc(d : &StructDesc) -> StructTemp{
    let mut ref_funs = refs_to_funs(&d.refs, d.ref_is_enum, &d.item_mod_name, d.is_mut);
    let mut param_funs = params_to_funs(&d.params, &d.item_mod_name, &d.item_struct_name, d.is_mut);
    let mut col_funs = cols_to_funs(d);
    param_funs.append(&mut col_funs);
    param_funs.append(&mut ref_funs);
    let (funs, proxies) = separate(param_funs.clone());

    StructTemp{
        new : new(&d.item_ptr_type, &d.item_struct_name, &param_funs),
        funs,
        self_mod_name: d.item_mod_name.to_string(),
        struct_name: d.item_struct_name.to_string(),
        ptr_type: d.item_ptr_type.to_string(),
        proxies,
    }
}

fn new(ptr_type_name : &str, result_type_name : &str, proxies : &[Ret]) -> String{
    let mut s = String::new();
    push(&mut s, 0, &format!("pub fn new(ptr : {}) -> {}{{\n", ptr_type_name, result_type_name));
    push(&mut s, 1, &format!("{}{{ ptr, ", result_type_name));
    for p in proxies {
        if let Some(n) = &p.proxy {
            s.push_str(&format!("{} : None, ", n.name))
        }
    }
    s.push_str("}\n");
    push(&mut s, 0, "}");
    s
}

#[derive(Debug, PartialEq, Clone)]
struct Ret{
    fun : String,
    proxy : Option<Proxy>,
}
#[derive(Debug, PartialEq, Clone)]
struct Proxy{
    name : String,
    type_without_option : String,
}
fn separate(v : Vec<Ret>) -> (Vec<String>, Vec<String>){
    let mut funs :Vec<String> = vec![];
    let mut proxies :Vec<String> = vec![];
    for item in v{
        funs.push(item.fun);
        if let Some(p) = item.proxy {
            proxies.push(format!("{} : Option<{}>,",p.name, p.type_without_option));
        }
    }
    (funs, proxies)
}

fn params_to_funs(items : &[ParamItem], self_mod_name : &str, self_type_name : &str, is_mut : bool) -> Vec<Ret>{
    let mut vec : Vec<Ret> = Vec::with_capacity(items.len());
    for item in items{
        vec.push(param_to_fun_get(item, self_mod_name, self_type_name));
        if is_mut{
            vec.push(param_to_fun_set(item, self_mod_name))
        }
    }
    vec
}

fn proxy_name(name : &str) -> String{
    format!("p_{}", name)
}

pub fn with_old(name : &str, is_old : bool) -> String {
    if is_old {
        format!("{}_old", name)
    } else {
        name.to_string()
    }
}

pub fn with_var(t : &str, vt : VarType) -> String{
    match vt{
        VarType::Normal => t.to_string(),
        VarType::Nullable => format!("NullOr<{}>", t),
        VarType::Undefiable => format!("UndefOr<{}>", t),
        VarType::UndefNullable => format!("Qv<{}>", t),
    }
}

pub fn push(s : &mut String, tabs : usize, text : &str) {
    for _ in 0..tabs {
        s.push('\t');
    }
    s.push_str(text);
}


fn param_to_fun_get(item : &ParamItem, self_mod_name : &str, self_type_name : &str) -> Ret{
    let p = proxy_name(&item.name);

    match item.param_type{
        ParamType::Bool =>{
            //let proxy = format!("{} : Option<{}>,", p, with_var("bool", item.var_type));
            let s = get_fun_string(&item.name, item.is_old, item.var_type,
                                   self_mod_name, self_type_name, "bool", &p, "bool");
            Ret{ proxy : Some(Proxy{ name : p, type_without_option : with_var("bool", item.var_type) }), fun : s }
        },
        ParamType::Num =>{
            let s = get_fun_string(&item.name, item.is_old, item.var_type,
                                   self_mod_name, self_type_name, "num", &p, "f64");
            Ret{ proxy : Some(Proxy{ name : p, type_without_option : with_var("f64", item.var_type) }), fun : s }
        },
        _ =>{ unimplemented!() }
    }
}

fn param_to_fun_set(item : &ParamItem, item_mod_name : &str) -> Ret {
    let p = proxy_name(&item.name);

    let fun = match item.param_type {
        ParamType::Bool => fun_set(item, item_mod_name, &p),
        _ =>{ unimplemented!() }
    };
    Ret{ proxy : None, fun }
}

fn cols_to_funs(d : &StructDesc) -> Vec<Ret>{
    let mut vec : Vec<Ret> = Vec::with_capacity(d.children.len());
    for child in &d.children{
        if child.col_struct_name.is_empty() == false{
            let item_name = to_item_name(&child.col_struct_name);
            let p = proxy_name(&item_name);
            //let proxy = format!("{} : Option<{}>,", &p, &child.col_ptr_type);
            let s = get_col_fun_string(&item_name, child.col_is_old,
                                   &d.item_mod_name, &child.col_mod_name,
                                   &p, &child.col_struct_name);
            vec.push(Ret{ proxy : Some(Proxy{
                name : p,
                type_without_option : child.col_struct_name.to_string(),
            }), fun : s });
        }
    }
    vec
}



fn refs_to_funs(items : &[RefItem], ref_is_enum : bool, self_mod_name : &str, is_mut : bool) -> Vec<Ret>{
    let mut vec : Vec<Ret> = Vec::with_capacity(items.len());
    for item in items{
        vec.push(ref_to_fun_get(item, self_mod_name));
        if is_mut{
            //vec.push(ref_to_fun_set(item, self_mod_name))
        }
    }
    vec
}

fn ref_proxy_name(s : &str) -> String{
    format!("ref_{}", s)
}

fn ref_to_fun_get(item : &RefItem, item_mod_name : &str) -> Ret {
    let p = ref_proxy_name(&item.name);
    let item_type = to_item_type_name(&item.name);

    let s = get_ref_fun_string(&item.name, item.is_old, item.var_type,
                           item_mod_name,  &p, &item_type);
    Ret { proxy: Some(Proxy { name: p, type_without_option: with_var(&item_type, item.var_type) }), fun: s }
}
