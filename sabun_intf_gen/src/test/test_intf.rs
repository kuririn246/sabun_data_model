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

                // let ba = root.col();
                // let huga = ba.huga();
                // let refed = huga.ref_refed();
                // let mem = refed.mem();
                // let atdesu = root.at_at_desu();
                // println!("{}", atdesu);
                // root.set_at_at_desu("hogehoge".to_string());
                //

            },
            Err(e) => { println!("val 1 {}", e.message) }
        }
    }
}