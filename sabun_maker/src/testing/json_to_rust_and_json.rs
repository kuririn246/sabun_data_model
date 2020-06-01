#[cfg(test)]
mod tests {
    use crate::imp::json_to_rust::json_dir_to_rust::json_dir_to_rust;
    use crate::structs::value_type::ValueType;

    #[test]
    fn it_works() {
        match json_dir_to_rust("src/json_dir/json_siyou", true) {
            Ok(a) => { println!("ok ok {:?}", a.default) },
            Err(e) => { println!("{}", e.message) }
        }
    }

    #[test]
    fn check_size(){
        println!("size {} ",std::mem::size_of::<ValueType>());
    }


    // #[test]
    // fn it_works() {
    //     match json_dir_to_rust("src/json_dir/json_siyou", true){
    //         Ok(a) => {
    //             match rust_to_json_new_default(&a, None, &a) {
    //                 Ok(a_v) => {
    //                     let av_s = a_v.to_string_pretty();
    //                     match json_root_to_rust(&av_s){
    //                         Ok(b) =>{
    //                             match rust_to_json_new_default(&b, None, &b){
    //                                 Ok(b_v) =>{
    //                                     let bv_s = b_v.to_string_pretty();
    //
    //                                     match json_root_to_rust(&bv_s){
    //                                         Ok(c) =>{
    //                                             match rust_to_json_new_default(&c, None, &c){
    //                                                 Ok(c_v) =>{
    //                                                     let cv_s = c_v.to_string_pretty();
    //                                                     assert_eq!(bv_s, cv_s);
    //                                                     println!("{}", cv_s);
    //                                                 },
    //                                                 Err(e) =>println!("type6 {}", e.message),
    //                                             }
    //
    //                                         },
    //                                         Err(e) =>println!("type5 {}", e.message),
    //                                     }
    //                                 },
    //                                 Err(e) => println!("type4 {}", e.message),
    //                             }
    //                         }
    //                         Err(e) => println!("type3 {}", e.message)
    //                     }
    //                 },
    //                 Err(e) => { println!("type2 {}", e.message); }
    //             }
    //         },
    //         Err(e) => println!("type1 {}", e.message),
    //     }
    // }
}