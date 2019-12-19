mod read_json;
mod json_to_rust;
mod rust_struct;

#[cfg(test)]
mod tests {
    use crate::json_to_rust::json_obj_to_rust;

    #[test]
    fn it_works() {
        let v = crate::read_json::untyped_example().unwrap();
        let r = json_obj_to_rust(&v);
        println!("{:?}", r);
    }
}
