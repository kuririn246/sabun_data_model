mod old;
pub mod error;
mod imp;
pub mod rust_struct;
pub mod my_json;
mod test;

pub use imp::rust_to_json::rust_to_json::rust_to_json_new_default as rust_to_json_new_default;
pub use imp::json_to_rust::json_dir_to_rust::json_dir_to_rust as json_dir_to_rust;
#[cfg(test)]
mod tests {
    use crate::imp::json_to_rust::json_dir_to_rust::json_dir_to_rust;

    #[test]
    fn it_works() {
        match json_dir_to_rust("src/json_dir/json_siyou", true){
            Ok(a) =>{ println!("{:?}", a)}
            Err(e) => println!("{}", e.message),
        }
    }
}