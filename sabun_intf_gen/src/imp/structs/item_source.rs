use crate::imp::to_member_source::MemberSource;
use crate::imp::structs::source_builder::SourceBuilder;

#[derive(Debug, PartialEq)]
pub struct ItemSource {
    members : Vec<MemberSource>,
    refs : Vec<RefSource>,
}
impl ItemSource{
    pub fn new(members : Vec<MemberSource>, refs : Vec<RefSource>) -> ItemSource{ ItemSource{ members, refs } }
    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();

        sb.to_string()
    }
}