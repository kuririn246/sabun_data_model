pub mod json_siyou;
pub mod error;
pub mod json_to_rust;
pub mod rust_struct;
pub mod json_name;
pub mod json_item_to_rust;
pub mod json_array_to_rust;
pub mod json_list_to_rust;

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
