mod old;
pub mod error;
mod imp;
pub mod structs;
pub mod my_json;
mod test;

pub use imp::rust_to_json::rust_to_json::rust_to_json_new_default as rust_to_json_new_default;
pub use imp::json_to_rust::json_dir_to_rust::json_dir_to_rust as json_dir_to_rust;
pub use crate::imp::rust_to_json::deconstruct_include::deconstruct_include as deconstruct_include;

#[cfg(test)]
mod tests {
    use crate::imp::json_to_rust::json_dir_to_rust::json_dir_to_rust;
    use crate::{rust_to_json_new_default, deconstruct_include};
    use crate::test::test_version_adjuster::test_version_adjuster;


    #[test]
    fn it_works() {
        match test_version_adjuster(){
            Ok(_) =>{},
            Err(e) => println!("{}", e.message),
        }
    }
}