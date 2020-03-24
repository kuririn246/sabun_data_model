// #[cfg(test)]
// mod tests {
//     use crate::imp::json_to_rust::json_dir_to_rust::json_dir_to_rust;
//     use crate::{rust_to_json_new_default, deconstruct_include};
//     use crate::imp::json_to_rust::json_item_to_rust::json_item_to_rust;
//     use crate::imp::json_to_rust::json_root_to_rust;
//
//
//     #[test]
//     fn it_works() {
//         match json_dir_to_rust("src/json_dir/json_siyou", true){
//             Ok(a) => {
//                 match rust_to_json_new_default(&a) {
//                     Ok(a_v) => {
//                         let av_s = a_v.to_string_pretty();
//                         match json_root_to_rust(&av_s){
//                             Ok(b) =>{
//                                 match rust_to_json_new_default(&b){
//                                     Ok(b_v) =>{
//                                         assert_eq!(av_s,b_v.to_string_pretty());
//                                     },
//                                     Err(e) => println!("{}", e.message),
//                                 }
//                             }
//                             Err(e) => println!("{}", e.message)
//                         }
//                     },
//                     Err(e) => { println!("{}", e.message); }
//                 }
//             },
//             Err(e) => println!("{}", e.message),
//         }
//     }
// }