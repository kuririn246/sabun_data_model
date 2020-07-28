// use crate::imp::structs::sources::StructSource;
// use crate::imp::structs::str_and_tab::{str_and_tabs_to_string, StrAndTab};
// use crate::imp::structs::struct_temp::StructTemp;
// use crate::imp::structs::source_builder::SourceBuilder;
//
// pub fn generate_item_source(struct_name : &str, ptr_type : &str, new : &str, funs : &[String]) -> StructSource {
//     let mut sb = SourceBuilder::new();
//
//     sb.push(0, format!("#[repr(C)] #[derive(Debug, PartialEq, Clone, Copy)]"));
//     sb.push(0, format!("pub struct {} {{", struct_name));
//     sb.push(1, format!("ptr : {},", ptr_type));
//     sb.push(1, format!("root : *const RootItem,"));
//     sb.push(0, format!("}}"));
//     sb.push(0, format!("impl {} {{", struct_name));
//
//     for line in new.split('\n'){
//         sb.push(1, line.to_string());
//     }
//
//     for fun in funs {
//         for line in fun.split('\n') {
//             sb.push(1,line.to_string());
//         }
//     }
//     sb.push(0, format!("}}"));
//
//     StructSource::new(
//         str_and_tabs_to_string(&sb.vec),
//        struct_name.to_string())
// }