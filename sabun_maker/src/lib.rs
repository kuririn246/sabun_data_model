mod json_siyou;
mod error;

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
