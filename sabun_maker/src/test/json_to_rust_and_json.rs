

#[cfg(test)]
mod tests {
    use crate::rust_to_json_new_default;
    use crate::imp::json_to_rust::json_to_rust;

    #[test]
    fn it_works() {
        let v = crate::old::json_siyou_old::untyped_example().unwrap();
        match json_to_rust(&v) {
            Ok(obj) => {
                match rust_to_json_new_default(&obj) {
                    Ok(val) => {
                        let pretty = val.to_string_pretty();
                        //println!("{}", pretty);
                        match json5_parser::from_str(&pretty) {
                            Ok(v) => {
                                match json_to_rust(&v) {
                                    Ok(obj) => {
                                        match rust_to_json_new_default(&obj) {
                                            Ok(val) => {
                                                let pretty2 = val.to_string_pretty();
                                                assert_eq!(pretty, pretty2);
                                            },
                                            Err(e) => { println!("{:?}", e); }
                                        }
                                    },
                                    Err(e) => { println!("{:?}", e); }
                                }
                            }
                            Err(e) => { println!("{:?}", e); }
                        }
                    },
                    Err(e) => { println!("{:?}", e) }
                }
            },
            Err(e) => println!("{:?}", e)
        }
        //let r = json_obj_to_rust(&v);
        //println!("{:?}", r);
    }
}
