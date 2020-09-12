use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_snake_name, to_citem_type_name};
use crate::imp::util::with_old::with_old;
use crate::imp::structs::citem_source::CItemSource;
use sabun_maker::intf::member_desc::{MemberDesc};

#[derive(Debug, PartialEq)]
pub struct CilSource {
    stem : String,
    is_old : bool,
    item_source : CItemSource,
}
impl CilSource {
    pub fn new(stem : String, is_old : bool, item_source : CItemSource) -> CilSource {
        CilSource { stem, is_old, item_source }
    }
    pub fn from(desc : &MemberDesc) -> CilSource {
        let cs = desc.child_descs().unwrap();

        CilSource::new(
            desc.name().to_string(),
            desc.is_old(),
            CItemSource::from(desc.name().to_string(), cs.items(), cs.refs())
        )
    }

    pub fn stem(&self) -> &str{ &self.stem }
    pub fn is_old(&self) -> bool{ self.is_old }
    pub fn item_source(&self) -> &CItemSource { &self.item_source }

    pub fn get(&self) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let item_type_name = to_citem_type_name(id);
        sb.push(0,&format!("pub fn {}(&self) -> CListPtr<{}>{{", with_old(&snake_name, is_old), &item_type_name));
        sb.push(1,&format!("citem::get_cil(self.ptr, \"{}\").unwrap();", id));
        sb.push(0,"}");
        sb.to_string()
    }
    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();
        sb.push_without_newline(0, &self.item_source.to_string());
        sb.to_string()
    }
}