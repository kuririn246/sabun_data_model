pub mod kihon_enum;
pub mod enc_dec;
pub mod url_string;
pub mod basic_compaction;
mod test;
pub mod string_compaction;

pub use enc_dec::encode::encode;
pub use enc_dec::decode::decode;


//#[allow(dead_code)]
#[cfg(test)]
pub mod tests {

    #[test]
    pub fn it_works() {
        //assert!(0.5f64 as f32 as f64 == 0.5f64);
        //crate::test_enc_dec::_test_enc_dec();
        //crate::test_string_compaction::_test_string_compaction();
        //test_be_bytes();
    }
}
