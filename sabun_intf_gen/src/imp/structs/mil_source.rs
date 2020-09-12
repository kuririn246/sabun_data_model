use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_snake_name, to_item_type_name, to_list_type_name};
use crate::imp::util::with_old::with_old;
use sabun_maker::intf::member_desc::{MemberDesc};
use crate::imp::structs::mitem_source::MItemSource;

#[derive(Debug, PartialEq)]
pub struct MilSource {
    stem : String,
    is_old : bool,
    item_source : MItemSource,
}
impl MilSource {
    pub fn new(stem : String, is_old : bool, item_source : MItemSource) -> MilSource {
        MilSource { stem, is_old, item_source }
    }
    pub fn from(desc : &MemberDesc) -> MilSource {
        let cs = desc.child_descs().unwrap();

        MilSource::new(
            desc.name().to_string(),
            desc.is_old(),
            MItemSource::from(desc.name().to_string(), cs.items(), cs.refs())
        )
    }

    pub fn stem(&self) -> &str{ &self.stem }
    pub fn is_old(&self) -> bool{ self.is_old }
    pub fn item_source(&self) -> &MItemSource { &self.item_source }

    pub fn get(&self) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let list_type_name = to_list_type_name(id);
        sb.push(0,&format!("pub fn {}(&self) -> {}{{", with_old(&snake_name, is_old), &list_type_name));
        sb.push(1,&format!("let ans = mut_item::get_inner_mut_list(self.ptr, \"{}\").unwrap();", id));
        sb.push(1,&format!("{}::new(ans)", &list_type_name));
        sb.push(0,"}");
        sb.to_string()
    }
    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();
        let id = self.stem();
        let list_type_name = to_list_type_name(id);
        let item_type_name = to_item_type_name(id);

        sb.push(0,&format!("#[derive(Debug, PartialEq)]"));
        sb.push(0,&format!("pub struct {} {{", &list_type_name));
        sb.push(1,"ptr : InnerListPtr,");
        sb.push(0,"}");
        sb.push(0, &format!("impl {} {{", &list_type_name));
        sb.push(1, &format!("pub fn new(ptr : InnerListPtr) -> {}{{ {}{{ ptr }} }} ",
                           &list_type_name, &list_type_name));

        sb.push(1, &format!("pub fn len(&self) -> usize{{ inner_list::get_len(self.ptr) }}"));
        sb.push(1, &format!("pub fn index(&self, idx : usize) -> {}{{", item_type_name));
        sb.push(2, &format!("let val = inner_list::get_value(self.ptr, idx).unwrap();"));
        sb.push(2, &format!("{}::new(val)", item_type_name));
        sb.push(1, "}");
        sb.push(1, &format!("pub fn iter(&self) -> GeneralIter<{}, {}>{{", &list_type_name, &item_type_name));
        sb.push(2, &format!("GeneralIter::new(self.len(), self, {}::index)", &list_type_name));
        sb.push(1, "}");
        sb.push(0, "}");

        sb.push_without_newline(0, &self.item_source.to_string());
        sb.to_string()
    }
}