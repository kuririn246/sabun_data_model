// #[cfg(test)]
// mod tests {
//     use crate::imp::json_to_rust::json_dir_to_rust::json_dir_to_rust;
//     use crate::{rust_to_json_new_default, deconstruct_include};
//
//
//     #[test]
//     fn it_works() {
//         match json_dir_to_rust("src/json_dir/json_siyou", true){
//             Ok(a) => {
//                 match rust_to_json_new_default(&a) {
//                     Ok(v) => {
//                         let dec = deconstruct_include(v);
//                         println!("{}", dec.to_string());
//                     },
//                     Err(e) => { println!("{}", e.message); }
//                 }
//             },
//             Err(e) => println!("{}", e.message),
//         }
//     }
// }