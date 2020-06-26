
pub struct Sources{
    usings : String,
    root : String,
    structs : Vec<StructSource>,
}

impl Sources{
    pub fn new(usings : String, root : String, structs : Vec<StructSource>) -> Sources{
        Sources{ usings, root, structs }
    }
    pub fn usings(&self) -> &str{ &self.usings }
    pub fn root(&self) -> &str{ &self.root }
    pub fn structs(&self) -> &[StructSource]{ &self.structs }

    pub fn to_string(&self) -> String{
        let mut r = String::new();
        r.push_str(self.usings());
        r.push('\n');
        r.push_str(self.root());
        r.push('\n');
        for s in self.structs(){
            r.push_str(s.source());
            r.push('\n');
        }
        r
    }
}

pub struct StructSource{
    source : String,
    struct_name : String,
}

impl StructSource{
    pub(crate) fn new(source : String, struct_name : String) -> StructSource{ StructSource{ source, struct_name } }
    pub fn source(&self) -> &str{ &self.source }
    pub fn struct_name(&self) -> &str{ &self.struct_name }
}

pub struct SourceTree{
    pub(crate) item_source : StructSource,
    pub(crate) col_source : StructSource,
    pub(crate) children : Vec<SourceTree>
}


