use crate::imp::structs::struct_desc::StructDesc;

pub struct StructDescTree{
    item : StructDesc,
    children : Vec<StructDesc>
}
impl StructDescTree{
    pub fn new(item : StructDesc, children : Vec<StructDesc>) -> StructDescTree{
        StructDescTree{ item, children }
    }
    pub fn item(&self) -> &StructDesc{ &self.item }
    pub fn children(&self) -> &[StructDesc]{ &self.children }
}
