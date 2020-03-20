mod json_siyou;
pub mod error;
mod imp;
pub mod rust_struct;
pub mod my_json;

pub use imp::json_to_rust::json_to_rust as json_to_rust;
pub use imp::rust_to_json::rust_to_json::rust_to_json_new_default as rust_to_json_new_default;

#[cfg(test)]
mod tests {
    use crate::{json_to_rust, rust_to_json_new_default};

    #[test]
    fn it_works() {

        let v = crate::json_siyou::untyped_example().unwrap();
        match json_to_rust(&v){
            Ok(obj) =>{
                match rust_to_json_new_default(&obj){
                    Ok(val) =>{
                        let pretty = val.to_string_pretty();
                        println!("{}", pretty);
                        match json5_parser::from_str(&pretty){
                            Ok(v) =>{
                                match json_to_rust(&v){
                                    Ok(obj) =>{
                                        match rust_to_json_new_default(&obj){
                                            Ok(val) =>{
                                                let pretty2 = val.to_string_pretty();
                                                assert_eq!(pretty, pretty2);
                                            },
                                            Err(e) =>{ println!("{:?}", e); }
                                        }
                                    },
                                    Err(e) =>{ println!("{:?}", e); }
                                }
                            }
                            Err(e) =>{ println!("{:?}", e); }
                        }
                    },
                    Err(e) =>{ println!("{:?}", e) }
                }
            },
            Err(e) => println!("{:?}", e)
        }
        //let r = json_obj_to_rust(&v);
        //println!("{:?}", r);
    }
}
