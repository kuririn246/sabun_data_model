use crate::imp::structs::struct_desc::{StructDesc, RefItem, ParamItem, ParamType};
use crate::imp::structs::struct_temp::{StructTemp, Proxy};
use crate::imp::structs::fun::{Fun, Arg, Contents, GetC, SetC};

pub fn to_struct_temp_from_struct_desc(d : &StructDesc) -> StructTemp{
    let mut ref_funs = refs_to_funs(&d.refs, d.ref_is_enum);
    let mut param_funs = params_to_funs(&d.params, d.is_mut);
    let mut ref_proxies = refs_to_proxies(&d.refs, d.ref_is_enum);
    let mut param_proxies = params_to_proxies(&d.params);
    param_funs.append(&mut ref_funs);
    param_proxies.append(&mut ref_proxies);

    StructTemp{
        funs: param_funs,
        self_mod_name: d.item_mod_name.to_string(),
        struct_name: d.item_struct_name.to_string(),
        ptr_type: d.item_ptr_type.to_string(),
        proxies: param_proxies,
    }
}

fn refs_to_funs(items : &[RefItem], ref_is_enum : bool) -> Vec<Fun>{
    //unimplemented!()
}

fn params_to_funs(items : &[ParamItem], is_mut : bool) -> Vec<Fun>{
    let mut vec : Vec<Fun> = Vec::with_capacity(items.len());
    for item in items{
        vec.push(param_to_fun_get(item));
        if is_mut{
            vec.push(param_to_fun_set(item))
        }
    }
    vec
}

fn proxy_name(name : &str) -> String{
    format!("p_{}", name)
}

fn with_old(name : &str, is_old : bool) -> String {
    if is_old {
        format!("{}_old", name)
    } else {
        name.to_string()
    }
}

fn param_to_fun_get(item : &ParamItem) -> Fun{
    match item.param_type{
        ParamType::Bool =>{
            Fun::new(&with_old(&item.name, item.is_old),
                     Contents::Get(GetC::new("bool", item.var_type, &proxy_name(&item.name))))
        },
        _ { unimplemented!() }
    }
}

fn param_to_fun_set(item : &ParamItem) -> Fun {
    match item.param_type {
        ParamType::Bool => {
            Fun::new(&format!("set_{}", with_old(&item.name, item.is_old)),
                     Contents::Set(SetC::new("bool", item.var_type, &proxy_name(&item.name))))
        },
        _ { unimplemented!() }
    }
}

fn params_to_proxies(items : &[ParamItem]) -> Vec<Proxy>{
    unimplemented!()
}

fn refs_to_proxies(items : &[RefItem], ref_is_enum : bool) -> Vec<Proxy>{
    //unimplemented!()
}