use crate::imp::to_member_source::MemberSource;
use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::to_item_type_name;
use crate::imp::structs::ref_source::RefSource;

#[derive(Debug, PartialEq)]
pub struct ItemSource {
    stem : String,
    members : Vec<MemberSource>,
    refs : Vec<RefSource>,
}
impl ItemSource{
    pub fn new(members : Vec<MemberSource>, refs : Vec<RefSource>, stem : String) -> ItemSource{ ItemSource{ members, refs, stem } }
    pub fn stem(&self) -> &str{ &self.stem }
    pub fn members(&self) -> &[MemberSource]{ &self.members }
    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let item_type_name = to_item_type_name(id);

        sb.push(0,&format!("#[derive(Debug, PartialEq)]"));
        sb.push(0,&format!("pub struct {} {{", &item_type_name));
        sb.push(1,"ptr : ListItemPtr,");
        sb.push(1,"root : *mut RootItem,");
        sb.push(0,"}");
        sb.push(0, &format!("impl {} {{", &data_type_name));
        sb.push(1, &format!("pub fn new(ptr : ListItemPtr, root : *mut RootItem) -> {}{{ {}{{ ptr, root }} }} ",
                            &item_type_name, &item_type_name));
        for mem in self.members() {
            match mem{
                MemberSource::Param(param) =>{
                    sb.push(1, &param.get("list_item", "self.ptr"));
                }
            }
        }
        sb.push(0, "}");

        sb.to_string()
    }
}