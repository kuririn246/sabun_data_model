pub mod json_siyou;
pub mod error;
pub mod json_to_rust;
pub mod rust_struct;

#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {

        let v = crate::json_siyou::untyped_example().unwrap();
        println!("{:?}", v);
        //let r = json_obj_to_rust(&v);
        //println!("{:?}", r);
    }
}
