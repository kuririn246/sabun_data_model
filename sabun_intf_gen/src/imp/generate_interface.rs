use sabun_maker::intf::*;
use sabun_maker::structs::RootObject;

use crate::imp::create_funs::create_funs;
use crate::imp::generate_struct::generate_struct;
use crate::imp::structs::fun::Impl;
use crate::imp::structs::sources::{Sources, StructSource};
use crate::imp::structs::struct_desc::StructDesc;

pub fn generate_interface(root : &RootObject) -> Sources{
    let mem_descs = root::get_member_desc(root);
    let (funs, proxies, descs) = create_funs(&mem_descs, true);
    let root : StructSource = generate_struct(&Impl{
        self_mod_name : "root".to_string(),
        funs,
        proxies,
        struct_name : "RootPtr".to_string(),
        ptr_type : "RootObject".to_string(),
    });

    let mut vec : Vec<StructSource> = vec![];
    for desc in &descs {
        vec.append(&mut generate_str_source(desc));
    }

    let usings = "use sabun_maker::intf::*;\nuse sabun_maker::structs::*;".to_string();
    Sources::new(usings, root.source, vec)
}

fn generate_str_source(desc : &StructDesc) -> Vec<StructSource>{
    let (funs, proxies, descs) =
        create_funs(&desc.mem_descs, desc.is_mut);

    let s : StructSource = generate_struct(&Impl{
        self_mod_name : desc.self_mod_name.to_string(),
        funs,
        proxies,
        struct_name : desc.struct_name.to_string(),
        ptr_type : desc.ptr_type.to_string(),
    });
    let mut result = vec![s];

    for desc in &descs{
        result.append(&mut generate_str_source(desc));
    }

    result
}