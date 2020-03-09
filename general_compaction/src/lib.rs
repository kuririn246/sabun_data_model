pub mod kihon_enum;
pub mod encode;
pub mod decode;
mod tag_storage;
pub mod signed_bytes;
mod vec_reader;
mod tag_reader;
mod test_enc_dec;
pub mod var_int;
pub mod string_compaction;
pub mod decimal_lib;
mod test_string_compaction;
pub mod b8s_and_b6s;
pub mod char_and_b6;
pub mod url_string;

//#[allow(dead_code)]
#[cfg(test)]
pub mod tests {

    #[test]
    pub fn it_works() {
        crate::test_enc_dec::_test_enc_dec();
        crate::test_string_compaction::_test_string_compaction();
        //test_be_bytes();
    }
}
