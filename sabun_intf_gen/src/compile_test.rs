use sabun_maker::intf::*;
use sabun_maker::structs::*;

pub struct RootIntf{
    root : Box<RootObject>,
}

impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{ RootIntf{ root : Box::new(obj) } }
    pub(crate) fn ptr(&self) -> RootObjectPtr{ RootObjectPtr::new(self.root.as_ref()) }

	pub fn refed(&self) -> RefedData{
		let ans = root::get_data(self.ptr(), "refed").unwrap();
		RefedData::new(ans)
	}
	
	pub fn bu(&self) -> bool{
		let qv = root::get_bool(self.ptr(), "bu").unwrap();
		qv.into_value().unwrap()
	}
	
	pub fn set_bu(&mut self, bu : bool){
		root::set_bool(self.ptr(), "bu", Qv::Val(bu));
	}
	
	pub fn str(&self) -> String{
		let qv = root::get_str(self.ptr(), "str").unwrap();
		qv.into_value().unwrap()
	}
	
	pub fn set_str(&mut self, str : String){
		root::set_str(self.ptr(), "str", Qv::Val(str));
	}
	
}
#[derive(Debug, PartialEq)]
pub struct RefedData {
	ptr : ConstDataPtr,
}
impl RefedData {
	pub fn new(ptr : ConstDataPtr) -> RefedData{ RefedData{ ptr } } 
	pub fn first(&self) -> RefedItem {
		let ptr = data::get_value(self.ptr, "first").unwrap();
		RefedItem::new(ptr)
	}
	pub fn second(&self) -> RefedItem {
		let ptr = data::get_value(self.ptr, "second").unwrap();
		RefedItem::new(ptr)
	}
	pub fn from_id(&self, id : &str) -> Option<RefedItem>{
		match id{
			"first" => Some(self.first()),
			"second" => Some(self.second()),
			_ =>{ None },
		}
	}
}
#[derive(Debug, PartialEq)]
pub struct RefedItem {
	ptr : ListItemPtr,
}
impl RefedItem {
	pub fn new(ptr : ListItemPtr) -> RefedItem{ RefedItem{ ptr } } 
	pub fn mem(&self) -> i64{
		let qv = list_item::get_int(self.ptr, "mem").unwrap();
		qv.into_value().unwrap()
	}
	
}


