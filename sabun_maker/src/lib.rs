pub mod read_json;
pub mod json_to_rust;
pub mod rust_struct;
pub mod convert_from_json_error;
pub mod json_name;

#[cfg(test)]
mod tests {
    use crate::json_to_rust::json_obj_to_rust;
    use crate::json_name::json_name;

    #[test]
    fn it_works() {
        json_name("aA0").unwrap();
        assert_eq!(true, json_name("A1").is_err());
        assert_eq!(true, json_name("01").is_err());
        println!("done")
        //let v = crate::read_json::untyped_example().unwrap();
        //let r = json_obj_to_rust(&v);
        //println!("{:?}", r);
    }
}
