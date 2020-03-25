#[cfg(test)]
mod tests {
    use crate::imp::json_to_rust::json_dir_to_rust::json_dir_to_rust;
    use crate::{rust_to_json_new_default, deconstruct_include};
    use crate::imp::json_to_rust::json_item_to_rust::json_item_to_rust;
    use crate::imp::json_to_rust::json_root_to_rust;


    #[test]
    fn it_works() {
        match json_dir_to_rust("src/json_dir/json_siyou", true){
            Ok(a) => {
                match rust_to_json_new_default(&a) {
                    Ok(a_v) => {
                        let av_s = a_v.to_string_pretty();
                        match json_root_to_rust(&av_s){
                            Ok(b) =>{
                                match rust_to_json_new_default(&b){
                                    Ok(b_v) =>{
                                        let bv_s = b_v.to_string_pretty();

                                        match json_root_to_rust(&bv_s){
                                            Ok(c) =>{
                                                match rust_to_json_new_default(&c){
                                                    Ok(c_v) =>{
                                                        let cv_s = c_v.to_string_pretty();
                                                        assert_eq!(bv_s, cv_s);
                                                        println!("{}", cv_s);
                                                    },
                                                    Err(e) =>println!("6 ban {}", e.message),
                                                }

                                            },
                                            Err(e) =>println!("5 ban {}", e.message),
                                        }
                                    },
                                    Err(e) => println!("4 ban {}", e.message),
                                }
                            }
                            Err(e) => println!("3 ban {}", e.message)
                        }
                    },
                    Err(e) => { println!("2 ban {}", e.message); }
                }
            },
            Err(e) => println!("1 ban {}", e.message),
        }
    }
}