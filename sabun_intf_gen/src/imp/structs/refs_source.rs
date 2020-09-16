use crate::imp::structs::ref_source::RefSource;

#[repr(C)] #[derive(Debug, PartialEq)]
pub struct RefsSource{
    refs : Vec<RefSource>,
    is_enum : bool,
}