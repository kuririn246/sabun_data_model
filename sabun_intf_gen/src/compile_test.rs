use sabun_maker::intf::*;
use sabun_maker::structs::*;

pub struct RootIntf{
    root : Box<RootObject>,
    item : Box<RootItem>,
}
pub struct RootItem{
    ptr : RootObjectPtr
}
impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{
        let root = Box::new(obj);
        let item = Box::new(RootItem{ ptr : RootObjectPtr::new(root.as_ref()) });
        RootIntf{ root, item }
    }
    pub fn ptr(&self) -> RootObjectPtr{ self.item.as_ref().ptr }

	pub fn bu(&self) -> bool{
		let qv = root::get_bool(self.ptr(), "bu").unwrap();
		qv.into_value().unwrap()
	}
	
	pub fn set_bu(&mut self, bu : bool){
		root::set_bool(self.ptr(), "bu", Qv::Val(bu));
	}
	
}
