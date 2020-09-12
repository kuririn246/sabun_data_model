use crate::imp::to_member_source::{MemberSource, to_member_source};
use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::structs::ref_source::RefSource;
use sabun_maker::intf::member_desc::MemberDesc;
use sabun_maker::intf::ref_desc::RefDescs;
use crate::imp::util::to_type_name::to_citem_type_name;

#[derive(Debug, PartialEq)]
pub struct CItemSource {
    stem : String,
    members : Vec<MemberSource>,
    refs : Vec<RefSource>,
}
impl CItemSource {
    pub fn new(members : Vec<MemberSource>, refs : Vec<RefSource>, stem : String) -> CItemSource { CItemSource { members, refs, stem } }
    pub fn from(stem : String, mems : &[MemberDesc], refs : &RefDescs) -> CItemSource {
        CItemSource {
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
        let item_type_name = to_citem_type_name(id);

        sb.push(0,&format!("#[derive(Debug, PartialEq)]"));
        sb.push(0,&format!("pub struct {} {{", &item_type_name));
        sb.push(1,"ptr : ListItemPtr,");
        sb.push(0,"}");
        sb.push(0, &format!("impl {} {{", &item_type_name));
        sb.push(1, &format!("pub fn new(ptr : ListItemPtr) -> {}{{ {}{{ ptr }} }} ",
                            &item_type_name, &item_type_name));
        for mem in self.members() {
            match mem{
                MemberSource::Param(param) =>{
                    sb.push_without_newline(1, &param.get("citem"));
                },
                MemberSource::Table(_) =>{},
                MemberSource::CList(_) =>{},
                MemberSource::MList(_) =>{},
                MemberSource::Cil(l) =>{
                    sb.push_without_newline(1, &l.get());
                },
                MemberSource::Mil(_) =>{},
            }
        }
        for r in self.refs() {
            sb.push_without_newline(1, &r.get(true))
        }
        sb.push(0, "}");

        for mem in &self.members{
            match mem{
                MemberSource::Cil(l) =>{
                    sb.push_without_newline(0, &l.to_string());
                },
                MemberSource::CList(_) =>{},
                MemberSource::Table(_) =>{},
                MemberSource::MList(_) =>{},
                MemberSource::Param(_) =>{},
                MemberSource::Mil(_) =>{},
            }
        }

        sb.to_string()
    }
}