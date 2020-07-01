#[cfg(test)]
mod tests {
    use sabun_maker::json_dir_to_rust;
    use crate::generate_interface;
    use sabun_maker::intf::RootObjectPtr;
    //use crate::rust_to_json_new_default;
    //use crate::imp::json_to_rust::json_root_to_rust;
    //use crate::imp::rust_to_json::root_to_json::root_to_json_new_default;

    #[test]
    fn it_works() {
        match json_dir_to_rust("src/json_dir/test", true) {
            Ok(mut a) => {
                let mut root = crate::compile_test::RootItem::new(RootObjectPtr::new(&mut a));
                let b = root.bu();
                let ba = root.col_data();
            },
            Err(e) => { println!("val 1 {}", e.message) }
        }
    }
}