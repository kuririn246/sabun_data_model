use sabun_maker::intf::*;
use sabun_maker::structs::*;
unsafe impl Send for RootIntf{}
#[derive(Debug, PartialEq)]
pub struct RootIntf{
    root : Box<RootObject>,
}
impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{ RootIntf{ root : Box::new(obj) } }
    pub(crate) fn ptr(&self) -> RootObjectPtr{ RootObjectPtr::new(self.root.as_ref()) }

	pub fn list(&self) -> ListList{
		let ans = root::get_list(self.ptr(), "list").unwrap();
		ListList::new(ans)
	}
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
pub struct ListList {
	ptr : ConstListPtr,
}
impl ListList {
	pub fn new(ptr : ConstListPtr) -> ListList{ ListList{ ptr } } 
	pub fn len(&self) -> usize{ list::get_len(self.ptr) }
	pub fn index(&self, idx : usize) -> ListItem{
		let val = list::get_value(self.ptr, idx).unwrap();
		ListItem::new(val)
	}
	pub fn iter(&self) -> GeneralIter<ListList, ListItem>{
		GeneralIter::new(self.len(), self, ListList::index)
	}
}
#[derive(Debug, PartialEq)]
pub struct ListItem {
	ptr : ListItemPtr,
}
impl ListItem {
	pub fn new(ptr : ListItemPtr) -> ListItem{ ListItem{ ptr } } 
	pub fn inner_list(&self) -> InnerListList{
		let ans = list_item::get_inner_list(self.ptr, "innerList").unwrap();
		InnerListList::new(ans)
	}
	pub fn nakabu(&self) -> bool{
		let qv = list_item::get_bool(self.ptr, "nakabu").unwrap();
		qv.into_value().unwrap()
	}
	pub fn ref_refed(&self) -> RefedItem{
		let qv = list_item::get_ref(self.ptr, "refed").unwrap();
		if let Qv::Val(v) = qv{ RefedItem::new(v) } else { unreachable!() }
	}
}
#[derive(Debug, PartialEq)]
pub struct InnerListList {
	ptr : InnerListPtr,
}
impl InnerListList {
	pub fn new(ptr : InnerListPtr) -> InnerListList{ InnerListList{ ptr } } 
	pub fn len(&self) -> usize{ inner_list::get_len(self.ptr) }
	pub fn index(&self, idx : usize) -> InnerListItem{
		let val = inner_list::get_value(self.ptr, idx).unwrap();
		InnerListItem::new(val)
	}
	pub fn iter(&self) -> GeneralIter<InnerListList, InnerListItem>{
		GeneralIter::new(self.len(), self, InnerListList::index)
	}
}
#[derive(Debug, PartialEq)]
pub struct InnerListItem {
	ptr : ListItemPtr,
}
impl InnerListItem {
	pub fn new(ptr : ListItemPtr) -> InnerListItem{ InnerListItem{ ptr } } 
	pub fn inner_mem(&self) -> i64{
		let qv = list_item::get_int(self.ptr, "innerMem").unwrap();
		qv.into_value().unwrap()
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

