pub mod json_siyou;
pub mod error;
mod json_to_rust;
pub mod rust_struct;

#[cfg(test)]
mod tests {
    use crate::json_to_rust::json_to_rust;

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
