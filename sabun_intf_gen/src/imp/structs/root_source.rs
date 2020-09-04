use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::to_member_source::MemberSource;

pub struct RootSource{
    members : Vec<MemberSource>,
}
impl RootSource{
    pub fn new(members : Vec<MemberSource>) -> RootSource{
        RootSource{ members }
    }
    pub fn members(&self) -> &[MemberSource]{
        &self.members
    }

    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();

        sb.push(0,     "\
use sabun_maker::intf::*;
use sabun_maker::structs::*;
unsafe impl Send for RootIntf{}
#[derive(Debug, PartialEq)]
pub struct RootIntf{
    root : Box<RootObject>,
    ptr : RootObjectPtr,
}
impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{
		let mut root = Box::new(obj);
		let ptr = RootObjectPtr::new(root.as_mut());
		RootIntf { root, ptr }
	}
");
        for mem in self.members() {
            match mem{
                MemberSource::Param(param) =>{
                    sb.push_without_newline(1, &param.get("root", "self.ptr"));
                    sb.push_without_newline(1, &param.set("root", "self.ptr"));
                },
                MemberSource::Data(data) =>{
                    sb.push_without_newline(1, &data.get("root", "self.ptr", ));
                },
                MemberSource::List(l) =>{
                    sb.push_without_newline(1, &l.get("root", "self.ptr"));
                },
                MemberSource::Mut(m) =>{
                    sb.push_without_newline(1, &m.get("root", "self.ptr"));
                },
                MemberSource::InnerList(_) =>{},
                MemberSource::InnerMut(_) =>{},
            }
        }
        sb.push(0, "}");

        for mem in self.members(){
            match mem{
                MemberSource::Data(data) =>{
                    sb.push(0, &data.to_string())
                },
                MemberSource::List(l) =>{
                    sb.push(0, &l.to_string())
                },
                MemberSource::Mut(m) =>{
                    sb.push(0, &m.to_string())
                },
                MemberSource::Param(_) =>{},
                MemberSource::InnerList(_) =>{},
                MemberSource::InnerMut(_)=>{},
            }
        }

        sb.to_string()
    }

    pub fn to_string_with_cfg_test(&self) -> String{
        let mut sb = SourceBuilder::new();
        sb.push(0, &format!("#[cfg(test)] pub mod test{{"));
        sb.push(1, &self.to_string());
        sb.push(0, "}");
        sb.to_string()
    }
}