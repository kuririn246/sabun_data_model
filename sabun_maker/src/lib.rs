#![feature(test)]


mod old;
pub mod error;
mod imp;
pub mod structs;
//pub mod my_json;
//mod indexmap;
mod testing;


extern crate test;

pub use imp::rust_to_json::root_to_json::root_to_json_new_default as rust_to_json_new_default;
pub use imp::json_to_rust::json_dir_to_rust::json_dir_to_rust as json_dir_to_rust;
pub use imp::version_adjuster::version_adjuster::adjust_versions as adjust_versions;
pub use imp::intf as intf;

pub(crate) use imp::structs::util::hash_m::HashMt as HashMt;
pub(crate) use imp::structs::util::hash_m::HashM as HashM;
pub(crate) use imp::structs::util::hash_m::HashS as HashS;
pub(crate) use imp::structs::util::hash_m::HashSt as HashSt;

//pub use crate::imp::rust_to_json::deconstruct_include::deconstruct_include as deconstruct_include;
