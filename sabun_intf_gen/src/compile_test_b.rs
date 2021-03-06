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
	pub fn mutable_list(&self) -> MListPtr<MutableListMItem>{
		root::get_mlist(self.ptr, "mutableList").unwrap()
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
	pub fn to_str(&self) -> &str{
		match self{
			RefedTableID::First => "first",
			RefedTableID::Second => "second",
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

#[derive(Debug, PartialEq)]
pub struct MutableListMItem {
	ptr : MItemPtr,
}
impl From<MItemPtr> for MutableListMItem {
	fn from(ptr : MItemPtr) -> Self {
		Self{ ptr }
	}
}
impl MutableListMItem {
	pub fn nakabu(&self) -> bool{
		let qv = mitem::get_bool(self.ptr, "nakabu").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_nakabu(&mut self, nakabu : bool){
		mitem::set_bool(self.ptr, "nakabu", Qv::Val(nakabu));
	}
	pub fn ref_refed(&self) -> RefedCItem{
		let qv = mitem::get_ref(self.ptr, "refed").unwrap();
		if let Qv::Val(v) = qv{ RefedCItem::from(v) } else { unreachable!() }
	}
	pub fn set_ref_refed(&self, id : RefedTableID) {
		mitem::set_ref(self.ptr, id.to_str());
	}
}

