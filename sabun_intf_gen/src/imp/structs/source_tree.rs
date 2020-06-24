use crate::imp::structs::sources::StructSource;

pub struct SourceTree{
    pub(crate) source : StructSource,
    pub(crate) children : Vec<SourceTree>,
}