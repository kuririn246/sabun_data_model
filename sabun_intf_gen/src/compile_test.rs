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

	pub fn test_list(&self) -> CListPtr<TestListCItem>{
		root::get_clist(self.ptr, "testList").unwrap()
	}
	pub fn refed(&self) -> RefedTable{
		let ans = root::get_table(self.ptr, "refed").unwrap();
		RefedTable::new(ans)
	}
	pub fn bu(&self) -> bool{
		let qv = root::get_bool(self.ptr, "bu").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_bu(&mut self, bu : bool){
		root::set_bool(self.ptr, "bu", Qv::Val(bu));
	}
	pub fn str(&self) -> String{
		let qv = root::get_str(self.ptr, "str").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_str(&mut self, str : String){
		root::set_str(self.ptr, "str", Qv::Val(str));
	}
}
#[derive(Debug, PartialEq)]
pub struct TestListCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for TestListCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl TestListCItem {
	pub fn inner_test_list(&self) -> CListPtr<InnerTestListCItem>{
		citem::get_cil(self.ptr, "innerTestList").unwrap()
	}
	pub fn nakabu(&self) -> bool{
		let qv = citem::get_bool(self.ptr, "nakabu").unwrap();
		qv.into_value().unwrap()
	}
	pub fn ref_refed(&self) -> RefedCItem{
		let qv = citem::get_ref(self.ptr, "refed").unwrap();
		if let Qv::Val(v) = qv{ RefedCItem::from(v) } else { unreachable!() }
	}
}
#[derive(Debug, PartialEq)]
pub struct InnerTestListCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for InnerTestListCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl InnerTestListCItem {
	pub fn inner_mem(&self) -> i64{
		let qv = citem::get_int(self.ptr, "innerMem").unwrap();
		qv.into_value().unwrap()
	}
}

#[derive(Debug, PartialEq)]
pub struct RefedTable {
	ptr : TablePtr,
}
impl RefedTable {
	pub fn new(ptr : TablePtr) -> RefedTable{ RefedTable{ ptr } } 
	pub fn first(&self) -> RefedCItem {
		let ptr = table::get_value(self.ptr, "first").unwrap();
		RefedCItem::from(ptr)
	}
	pub fn second(&self) -> RefedCItem {
		let ptr = table::get_value(self.ptr, "second").unwrap();
		RefedCItem::from(ptr)
	}
	pub fn from_id(&self, id : RefedTableID) -> RefedCItem{
		match id{
			RefedTableID::First => self.first(),
			RefedTableID::Second => self.second(),
		}
	}
}
#[repr(u64)] pub enum RefedTableID{ First, Second, }
impl RefedTableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"first" => Some(Self::First),
			"second" => Some(Self::Second),
			_ =>{ None }
		}
	}
	pub fn from_num(id : u64) -> Option<Self>{
		match id{
			0 => Some(Self::First),
			1 => Some(Self::Second),
			_ =>{ None }
		}
	}
	pub fn len() -> u64{ 2 }
}
#[derive(Debug, PartialEq)]
pub struct RefedCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for RefedCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl RefedCItem {
	pub fn mem(&self) -> i64{
		let qv = citem::get_int(self.ptr, "mem").unwrap();
		qv.into_value().unwrap()
	}
}

