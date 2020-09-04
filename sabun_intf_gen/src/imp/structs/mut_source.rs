use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_snake_name, to_item_type_name};
use crate::imp::util::with_old::with_old;
use sabun_maker::intf::member_desc::{MemberDesc};
use crate::imp::structs::mut_item_source::MutItemSource;

#[derive(Debug, PartialEq)]
pub struct MutSource {
    stem : String,
    is_old : bool,
    item_source : MutItemSource,
}
impl MutSource {
    pub fn new(stem : String, is_old : bool, item_source : MutItemSource) -> MutSource {
        MutSource { stem, is_old, item_source }
    }
    pub fn from(desc : &MemberDesc) -> MutSource {
        let cs = desc.child_descs().unwrap();

        MutSource::new(
            desc.name().to_string(),
            desc.is_old(),
            MutItemSource::from(desc.name().to_string(), cs.items(), cs.refs())
        )
    }

    pub fn stem(&self) -> &str{ &self.stem }
    pub fn is_old(&self) -> bool{ self.is_old }
    pub fn item_source(&self) -> &MutItemSource{ &self.item_source }

    pub fn get(&self, mod_name : &str, ptr_exp : &str) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let item_type_name = to_item_type_name(id);
        sb.push(0,&format!("pub fn {}(&self) -> {}{{", with_old(&snake_name, is_old), &item_type_name));
        sb.push(1,&format!("let ans = {}::get_mut_list({}, \"{}\").unwrap();", &mod_name, ptr_exp, id));
        sb.push(1,&format!("MutListPtr::new(ans)"));
        sb.push(0,"}");
        sb.to_string()
    }
    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();
        sb.push_without_newline(0, &self.item_source.to_string());
        sb.to_string()
    }
}