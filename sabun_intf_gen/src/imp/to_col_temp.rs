use crate::imp::structs::struct_desc::StructDesc;
use crate::imp::structs::struct_temp::ColTemp;

pub fn from_struct_desc(d : &StructDesc) -> ColTemp{
    ColTemp{
        self_mod_name: d.col_mod_name.to_string(),
        struct_name: d.col_struct_name.to_string(),
        ptr_type: d.col_ptr_type.to_string(),
        keys: d.keys.to_vec(),
        col_type: d.col_type.clone(),
        item_struct_name: d.item_struct_name.to_string(),
    }
}