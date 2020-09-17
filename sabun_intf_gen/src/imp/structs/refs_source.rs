use crate::imp::structs::ref_source::RefSource;
use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_enum_type_name, to_citem_type_name};

#[repr(C)] #[derive(Debug, PartialEq)]
pub struct RefsSource{
    refs : Vec<RefSource>,
    is_enum : bool,
}

impl RefsSource{
    pub fn new(refs : Vec<RefSource>, is_enum : bool) -> RefsSource{
        RefsSource{ refs, is_enum }
    }
    pub fn is_enum(&self) -> bool{ self.is_enum }
    pub fn refs(&self) -> &[RefSource]{ &self.refs }

    pub fn get(&self) -> String{
        let mut sb = SourceBuilder::new();
        if self.is_enum {

        } else {
            for r in &self.refs {
                sb.push_without_newline(0, &r.get(true))
            }
        }
        sb.to_string()
    }

    pub fn to_string(&self, stem : &str) -> Option<String>{
        if self.is_enum{ Some(self.get_enum_soutce()) } else{ None }
    }
    fn get_enum_soutce(&self, stem : &str) -> String{
        let mut sb = SourceBuilder::new();

        sb.append(&format!("pub enum {}{{ ", to_enum_type_name(stem)));
        for r in self.refs{
            let item_type = to_citem_type_name(r.name());
            sb.append(&format!("{}({}), ", &item_type, &item_type));
        }
        sb.append("}\n");



        sb.to_string()
    }
}

