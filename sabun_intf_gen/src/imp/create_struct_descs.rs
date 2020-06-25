use sabun_maker::intf::member_desc::{MemberDesc};
use sabun_maker::structs::{RustMemberType};
use crate::imp::structs::struct_desc::{StructDesc, ParamItem, ColType, ParamType};
use crate::imp::util::to_type_name::to_type_name;
use crate::imp::create_ref_items::create_ref_items;

pub fn create_struct_desc_root(mems : &[MemberDesc]) -> StructDesc{
    let (descs, params) = create_struct_descs(mems);
    StructDesc{
        refs : vec![],
        children : descs,
        keys : vec![],
        col_struct_name : String::new(),
        is_mut : true,
        item_ptr_type : format!("*mut RootObject"),
        col_mod_name : String::new(),
        col_type : ColType::Root,
        col_undefiable : false,
        item_mod_name : "root".to_string(),
        params,
        ref_is_enum : false,
        item_struct_name : "RootItem".to_string(),
        col_ptr_type: "".to_string(),
    }
}

pub fn create_struct_descs(mems : &[MemberDesc]) -> (Vec<StructDesc>, Vec<ParamItem>) {
    let mut result:Vec<StructDesc> = vec![];
    let mut params : Vec<ParamItem> = vec![];

    for mem in mems {
        match mem.member_type() {
            RustMemberType::Bool =>{
                params.push(ParamItem{
                    is_old: mem.is_old(),
                    name: mem.name().to_string(),
                    var_type: mem.var_type().clone(),
                    param_type: ParamType::Bool,
                });
            },
            RustMemberType::Data =>{
                let type_name = to_type_name(mem.name());
                let children = mem.child_descs().unwrap();
                let (descs,params) = create_struct_descs(children.items());
                let refs = create_ref_items(children.refs().items());
                result.push(StructDesc{
                    children : descs,
                    refs,
                    params,
                    keys : vec![],
                    item_mod_name : "list_item".to_string(),
                    col_mod_name : "data".to_string(),
                    item_struct_name : format!("{}Item", type_name),
                    col_struct_name : format!("{}Data", type_name),
                    col_ptr_type : "*const ConstData".to_string(),
                    item_ptr_type: "*const ListItem".to_string(),
                    col_undefiable : false,
                    col_type: ColType::Data,
                    ref_is_enum : children.refs().is_enum(),
                    is_mut : false,

                });
            }
            _=>{},
        }
    }

    (result, params)
}
