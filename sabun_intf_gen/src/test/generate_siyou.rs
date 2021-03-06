#[cfg(test)]
mod tests {
    use sabun_maker::json_dir_to_rust;
    use crate::generate_interface;
    use crate::test::write_file::test::write_file;
    //use crate::rust_to_json_new_default;
    //use crate::imp::json_to_rust::json_root_to_rust;
    //use crate::imp::rust_to_json::root_to_json::root_to_json_new_default;

    #[test]
    fn it_works() {
        match json_dir_to_rust("src/json_dir/test/siyou", true) {
            Ok(mut a) => {
                //println!("{:?}", a);
                let ans = generate_interface(&mut a);
                let source = ans.to_string();
                write_file("src/test_generated/siyou.rs", &source);
            },
            Err(e) => { println!("val 1 {}", e.message) }
        }
    }
}