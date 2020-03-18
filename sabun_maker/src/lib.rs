mod json_siyou;
pub mod error;
mod imp;
pub mod rust_struct;

pub use imp::json_to_rust::json_to_rust as json_to_rust;
pub use imp::rust_to_json::rust_to_json::rust_to_json_new_default as rust_to_json_new_default;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

        let v = crate::json_siyou::untyped_example().unwrap();
        match json_to_rust(&v){
            Ok(obj) => println!("{:?}", obj),
            Err(e) => println!("{:?}", e)
        }
        //let r = json_obj_to_rust(&v);
        //println!("{:?}", r);
    }
}
