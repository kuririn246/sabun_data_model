use crate::imp::to_member_source::MemberSource;

#[derive(Debug, PartialEq)]
pub struct ItemSource {
    members : Vec<MemberSouce>
}
impl ItemSource{
    pub fn new(members : Vec<MemberSource>) -> ItemSource{ ItemSource{ members } }
    pub fn to_string(&self) -> String{

    }
}