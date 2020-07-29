// use sabun_maker::intf::member_desc::{MemberDesc};
// use sabun_maker::structs::{RustMemberType};
// use crate::imp::structs::struct_desc::{StructDesc, ParamItem, ColType};
// use crate::imp::util::to_type_name::{to_type_name, to_item_type_name};
// use crate::imp::create_ref_items::create_ref_items;
//
// pub fn create_struct_desc_root(mems : &[MemberDesc]) -> StructDesc{
//     let (descs, params) = create_struct_descs(mems);
//     StructDesc{
//         refs : vec![],
//         children : descs,
//         keys : vec![],
//         col_struct_name : String::new(),
//         col_id : String::new(),
//         is_mut : true,
//         item_ptr_type : format!("RootObjectPtr"),
//         col_mod_name : String::new(),
//         col_type : ColType::Root,
//         col_undefiable : false,
//         col_is_old : false,
//         item_mod_name : "root".to_string(),
//         params,
//         ref_is_enum : false,
//         item_struct_name : "RootItem".to_string(),
//         col_ptr_type: "".to_string(),
//     }
// }
//
// pub fn create_struct_descs(mems : &[MemberDesc]) -> (Vec<StructDesc>, Vec<ParamItem>) {
//     let mut result:Vec<StructDesc> = vec![];
//     let mut params : Vec<ParamItem> = vec![];
//
//     for mem in mems {
//         match mem.member_type() {
//             RustMemberType::Bool =>{
//                 params.push(ParamItem{
//                     is_old: mem.is_old(),
//                     id: mem.name().to_string(),
//                     var_type: mem.var_type().clone(),
//                     value_type_name : "bool".to_string(),
//                     value_type_nickname : "bool".to_string(),
//                     is_ref : false,
//                 });
//             },
//             RustMemberType::Float =>{
//                 params.push(ParamItem{
//                     is_old: mem.is_old(),
//                     id: mem.name().to_string(),
//                     var_type: mem.var_type().clone(),
//                     value_type_name : "f64".to_string(),
//                     value_type_nickname : "float".to_string(),
//                     is_ref : false,
//                 });
//             },
//             RustMemberType::Int =>{
//                 params.push(ParamItem{
//                     is_old: mem.is_old(),
//                     id: mem.name().to_string(),
//                     var_type: mem.var_type().clone(),
//                     value_type_name : "i64".to_string(),
//                     value_type_nickname : "int".to_string(),
//                     is_ref : false,
//                 });
//             },
//
//             RustMemberType::Str =>{
//                 params.push(ParamItem{
//                     is_old: mem.is_old(),
//                     id: mem.name().to_string(),
//                     var_type: mem.var_type().clone(),
//                     value_type_name : "String".to_string(),
//                     value_type_nickname : "str".to_string(),
//                     is_ref : true,
//                 });
//             },
//             RustMemberType::Data =>{
//                 let type_name = to_type_name(mem.name());
//                 let children = mem.child_descs().unwrap();
//                 let (descs,params) = create_struct_descs(children.items());
//                 let refs = create_ref_items(children.refs().items());
//                 result.push(StructDesc{
//                     col_id : mem.name().to_string(),
//                     children : descs,
//                     refs,
//                     params,
//                     keys : children.keys().iter().map(|k| k.clone()).collect(),
//                     item_mod_name : "list_item".to_string(),
//                     col_mod_name : "data".to_string(),
//                     item_struct_name : to_item_type_name(mem.name()),
//                     col_struct_name : format!("{}Data", type_name),
//                     col_ptr_type : "ConstDataPtr".to_string(),
//                     item_ptr_type: "ListItemPtr".to_string(),
//                     col_undefiable : false,
//                     col_type: ColType::Data,
//                     col_is_old : mem.is_old(),
//                     ref_is_enum : children.refs().is_enum(),
//                     is_mut : false,
//                 });
//             },
//             RustMemberType::List =>{
//                 let type_name = to_type_name(mem.name());
//                 let children = mem.child_descs().unwrap();
//                 let (descs,params) = create_struct_descs(children.items());
//                 let refs = create_ref_items(children.refs().items());
//                 result.push(StructDesc{
//                     col_id : mem.name().to_string(),
//                     children : descs,
//                     refs,
//                     params,
//                     keys : vec![],
//                     item_mod_name : "list_item".to_string(),
//                     col_mod_name : "list".to_string(),
//                     item_struct_name : to_item_type_name(mem.name()),
//                     col_struct_name : format!("{}List", type_name),
//                     col_ptr_type : "ConstListPtr".to_string(),
//                     item_ptr_type: "ListItemPtr".to_string(),
//                     col_undefiable : false,
//                     col_type: ColType::List,
//                     col_is_old : mem.is_old(),
//                     ref_is_enum : children.refs().is_enum(),
//                     is_mut : false,
//                 });
//             },
//             _=>{},
//         }
//     }
//
//     (result, params)
// }
