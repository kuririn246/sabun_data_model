use crate::imp::to_member_source::{MemberSource, to_member_source};
use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::to_item_type_name;
use crate::imp::structs::ref_source::RefSource;
use sabun_maker::intf::member_desc::MemberDesc;
use sabun_maker::intf::ref_desc::RefDescs;

#[derive(Debug, PartialEq)]
pub struct MutItemSource {
    stem : String,
    members : Vec<MemberSource>,
    refs : Vec<RefSource>,
}
impl MutItemSource {
    pub fn new(members : Vec<MemberSource>, refs : Vec<RefSource>, stem : String) -> MutItemSource { MutItemSource { members, refs, stem } }
    pub fn from(stem : String, mems : &[MemberDesc], refs : &RefDescs) -> MutItemSource {
        MutItemSource {
            stem,
            members : mems.iter().map(to_member_source).collect(),
            refs : refs.items().iter().map(RefSource::from).collect()
        }
    }

    pub fn stem(&self) -> &str{ &self.stem }
    pub fn members(&self) -> &[MemberSource]{ &self.members }
    pub fn refs(&self) -> &[RefSource]{ &self.refs }

    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let item_type_name = to_item_type_name(id);

        sb.push(0,&format!("#[derive(Debug, PartialEq)]"));
        sb.push(0,&format!("pub struct {} {{", &item_type_name));
        sb.push(1,"ptr : MutListItemPtr,");
        sb.push(0,"}");
        sb.push(0, &format!("impl {} {{", &item_type_name));
        sb.push(1, &format!("pub fn new(ptr : MutListItemPtr) -> {}{{ {}{{ ptr }} }} ",
                            &item_type_name, &item_type_name));
        for mem in self.members() {
            match mem{
                MemberSource::Param(param) =>{
                    sb.push_without_newline(1, &param.get("mut_list_item", "self.ptr"));
                },
                MemberSource::Data(_) =>{},
                MemberSource::List(_) =>{},
                MemberSource::InnerList(l) =>{
                    sb.push_without_newline(1, &l.get("mut_list_item", "self.ptr"));
                }
            }
        }
        for r in self.refs() {
            sb.push_without_newline(1, &r.get("mut_list_item"))
        }
        sb.push(0, "}");

        for mem in &self.members{
            match mem{
                MemberSource::InnerList(l) =>{
                    sb.push_without_newline(0, &l.to_string());
                },
                MemberSource::List(_) =>{},
                MemberSource::Data(_) =>{},
                MemberSource::Param(_) =>{},
            }
        }

        sb.to_string()
    }
}