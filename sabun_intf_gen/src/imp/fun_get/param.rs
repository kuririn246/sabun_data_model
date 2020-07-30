// use sabun_maker::structs::VarType;
// use crate::imp::structs::source_builder::SourceBuilder;
// use sabun_maker::intf::member_desc::MemberDesc;
// use crate::imp::util::to_type_name::to_snake_name;
// use crate::imp::util::with_old::with_old;
// use crate::imp::util::with_var::with_var;
//
//
// //pub fn param1(&self) -> NullOr<String>{
// //    let item = unsafe{ &*self.item };
// //    let qv = item.get("param1").unwrap().clone()
// //    NullOr::from_qv(qv).unwrap()
// //}
//
// pub fn generate_param_fun_get(
//     desc : &MemberDesc, self_mod_name : &str, value_nickname: &str, value_type_name : &str, is_ref : bool) -> String{
//     let mut sb = SourceBuilder::new();
//
//     let id = desc.name();
//     let snake_name = to_snake_name(id);
//     let is_old = desc.is_old();
//     let var_type = desc.var_type();
//     //let and = if is_ref{ "&" } else{ "" };
//     sb.push(0,&format!("pub fn {}(&self) -> {}{{", with_old(&snake_name, is_old), with_var(value_type_name, var_type)));
//     sb.push(1,&format!("let qv = {}::get_{}(self.ptr, \"{}\").unwrap();", self_mod_name, value_nickname, id));
//     match &var_type {
//         VarType::Normal => {
//             sb.push(1,&format!("qv.into_value().unwrap();"));
//         },
//         VarType::Undefiable => {
//             sb.push(1,&format!("UndefOr::from_qv(qv).unwrap()"));
//         },
//         VarType::Nullable => {
//             sb.push(1,&format!("NullOr::from_qv(qv).unwrap()"));
//         },
//         VarType::UndefNullable => {
//             sb.push(1,&format!("qv"));
//         },
//     }
//     sb.push(0,&format!("}}"));
//     sb.to_string()
// }
//
