use crate::imp::structs::struct_desc::StructDesc;
use crate::imp::structs::struct_temp::StructTemp;

pub fn from_struct_desc(d : &StructDesc) -> StructTemp{
    StructTemp{
        funs: vec![],
        self_mod_name: "".to_string(),
        struct_name: "".to_string(),
        ptr_type: "".to_string(),
        proxies: vec![]
    }
}