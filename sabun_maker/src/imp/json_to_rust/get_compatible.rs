// use json5_parser::{JVal};
// use crate::imp::json_to_rust::names::Names;
// use crate::error::Result;
// use crate::imp::json_to_rust::json_name::{is_dot_chained_name};
// use crate::{HashS, HashSt};
//
// pub fn get_compatible(array : &[JVal], names : &Names) -> Result<HashS<String>>{
//     let mut result : HashS<String> = HashSt::with_capacity(array.len());
//
//     for item in array{
//         match item{
//             JVal::String(s, span) =>{
//                 if is_dot_chained_name(s) {
//                     result.insert(s.to_string());
//                 }else {
//                     Err(format!("{} {} is not a valid dot-chained name {}", span.line_str(), s, names))?;
//                 }
//             },
//             _ =>{
//                 let span = item.span();
//                 Err(format!(r#"{} {} old must be strings {}"#, span.line_str(), span.slice(), names))?
//             }
//         }
//     }
//     return Ok(result);
// }