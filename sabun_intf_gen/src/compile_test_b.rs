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

	pub fn refed(&self) -> RefedData{
		let ans = root::get_data(self.ptr, "refed").unwrap();
		RefedData::new(ans)
	}
	pub fn bu(&self) -> bool{
		let qv = root::get_bool(self.ptr, "bu").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_bu(&mut self, bu : bool){
		root::set_bool(self.ptr, "bu", Qv::Val(bu));
	}
	pub fn mutable_list(&self) -> MutListPtr<MutableListItem>{
		root::get_mut_list(self.ptr, "mutableList").unwrap()
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

#[derive(Debug, PartialEq)]
pub struct MutableListItem {
	ptr : MutListItemPtr,
}
impl MutableListItem {
	pub fn new(ptr : MutListItemPtr) -> MutableListItem{ MutableListItem{ ptr } } 
	pub fn nakabu(&self) -> bool{
		let qv = mut_list_item::get_bool(self.ptr, "nakabu").unwrap();
		qv.into_value().unwrap()
	}
}

impl From<MutListItemPtr> for MutableListItem{
	fn from(p : MutListItemPtr) -> Self {
		MutableListItem::new(p)
	}
}
