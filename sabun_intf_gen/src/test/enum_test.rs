#[cfg(test)]
mod tests {
    use sabun_maker::json_dir_to_rust;
    use crate::generate_interface;
    use sabun_maker::intf::RootObjectPtr;

    #[test]
    fn it_works() {

        match json_dir_to_rust("src/json_dir/test", true) {
            Ok(mut a) => {
                let mut root = crate::compile_test::RootIntf::new(a);
                println!("bu {} ", root.bu());
                root.set_bu(true);
                println!("bu {} ", root.bu());
                let mut enum_list = root.enum_test_list();
                let f = enum_list.first().unwrap();

            },
            Err(e) => { println!("val 1 {}", e.message) }
        }
    }
}