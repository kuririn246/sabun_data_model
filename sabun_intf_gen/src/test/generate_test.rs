// #[cfg(test)]
// mod tests {
//     use sabun_maker::json_dir_to_rust;
//     use crate::generate_interface;
//     //use crate::rust_to_json_new_default;
//     //use crate::imp::json_to_rust::json_root_to_rust;
//     //use crate::imp::rust_to_json::root_to_json::root_to_json_new_default;
//
//     #[test]
//     fn it_works() {
//         match json_dir_to_rust("src/json_dir/test", true) {
//             Ok(a) => {
//                 //println!("{:?}", a);
//                 let ans = generate_interface(&a);
//                 println!("{}", ans.to_string());
//             },
//             Err(e) => { println!("val 1 {}", e.message) }
//         }
//     }
// }