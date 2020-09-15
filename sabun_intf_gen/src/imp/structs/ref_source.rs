use sabun_maker::structs::{VarType};
use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_snake_name, to_citem_type_name, to_mitem_type_name};
use crate::imp::util::with_old::with_old;
use crate::imp::util::with_var::with_var;
use crate::imp::util::var_type_name::var_type_name;
use sabun_maker::intf::ref_desc::RefDesc;

#[repr(C)] #[derive(Debug, PartialEq)]
pub struct RefSource{
    name : String,
    var_type : VarType,
    is_old : bool,
}
impl RefSource{
    pub fn new(name : String, var_type : VarType, is_old : bool) -> RefSource{
        RefSource{
            name, var_type, is_old
        }
    }
    pub fn from(desc : &RefDesc) -> RefSource{
        RefSource::new(
        desc.data_name().to_string(),
        desc.var_type(),
        desc.is_old())
    }

    pub fn name(&self) -> &str{
        &self.name
    }
    pub fn var_type(&self) -> VarType{
        self.var_type
    }
    pub fn is_old(&self) -> bool{ self.is_old }

    pub fn get(&self, from_citem : bool) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.name();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let var_type = self.var_type();
        let item_type_name = if from_citem{ to_citem_type_name(id) } else{ to_mitem_type_name(id) };
        let mod_name = if from_citem{ "citem" } else{ "mitem" };
        let var_type_name = var_type_name(var_type);
        sb.push(0, &format!("pub fn ref_{}(&self) -> {}{{", with_old(&snake_name, is_old), with_var(&item_type_name, var_type)));
        sb.push(1,&format!("let qv = {}::get_ref(self.ptr, \"{}\").unwrap();", mod_name, id));
        if var_type != VarType::Normal {
            sb.push(1, &format!("let ptr = match qv{{"));
            sb.push(2, &format!("Qv::None =>{{ return {}::Null; }},", &var_type_name));
            sb.push(2, &format!("Qv::Undefined =>{{ return {}::Undefined; }},", &var_type_name));
            sb.push(2, &format!("Qv::Val(id) => {}::Val(id)", &var_type_name));
            sb.push(1, "}");
        } else {
            sb.push(1, &format!("if let Qv::Val(v) = qv{{ {}::from(v) }} else {{ unreachable!() }}", &item_type_name));
        }
        sb.push(0, "}");
        sb.to_string()
    }
    pub fn set(&self, mod_name : &str) -> String {
        let mut sb = SourceBuilder::new();
        let id = self.name();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        sb.push(0, &format!("pub fn set_ref_{}(&self, ref_id : Qv<String>){{\n", with_old(&snake_name, is_old)));
        sb.push(1,&format!("{}::set_ref(self.ptr, ref_id);", mod_name));
        sb.to_string()
    }
    pub fn c_get(&self) -> Option<&str>{
        unimplemented!()
    }
    pub fn c_set(&self) -> Option<&str>{
        unimplemented!()
    }
}
