use sabun_maker::intf::*;
use sabun_maker::structs::RootObject;

use crate::imp::fun::Impl;
use crate::imp::create_funs::create_funs;
use crate::imp::generate_struct::generate_struct;
use crate::imp::str_and_tab::str_and_tabs_to_string;

pub fn generate_interface(root : &RootObject) -> String{
    let mem_descs = root::get_member_desc(root);
    let funs = create_funs(&mem_descs, true);
    let vec = generate_struct(&Impl{
        self_mod_name : "root".to_string(),
        funs,
        struct_name : "Root".to_string(),
        ptr_type : "RootObject".to_string(),
    });
    str_and_tabs_to_string(&vec)
}