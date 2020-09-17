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

	pub fn enum_test_list(&self) -> CListPtr<EnumTestListCItem>{
		root::get_clist(self.ptr, "enumTestList").unwrap()
	}
	pub fn refed1(&self) -> Refed1Table{
		let ans = root::get_table(self.ptr, "refed1").unwrap();
		Refed1Table::new(ans)
	}
	pub fn refed2(&self) -> Refed2Table{
		let ans = root::get_table(self.ptr, "refed2").unwrap();
		Refed2Table::new(ans)
	}
	pub fn bu(&self) -> bool{
		let qv = root::get_bool(self.ptr, "bu").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_bu(&mut self, bu : bool){
		root::set_bool(self.ptr, "bu", Qv::Val(bu));
	}
}
#[derive(Debug, PartialEq)]
pub struct EnumTestListCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for EnumTestListCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl EnumTestListCItem {
	pub fn nakabu(&self) -> bool{
		let qv = citem::get_bool(self.ptr, "nakabu").unwrap();
		qv.into_value().unwrap()
	}
	pub fn get_enum(&self) -> EnumTestListEnum{
		let (list_name, _) = citem::get_enum(self.ptr).unwrap();
		let p = if let Qv::Val(p) = citem::get_ref(self.ptr, &list_name).unwrap(){ p } else { unreachable!() };
		EnumTestListEnum::new(&list_name,p)
	}
}
	pub enum EnumTestListEnum{ Refed2CItem(Refed2CItem), Refed1CItem(Refed1CItem), }
	impl EnumTestListEnum{
		pub fn new(list_name : &str, ptr : CItemPtr) -> EnumTestListEnum{
			match list_name{
				"refed2" => EnumTestListEnum::Refed2CItem(Refed2CItem::from(ptr)),
				"refed1" => EnumTestListEnum::Refed1CItem(Refed1CItem::from(ptr)),
				_ => panic!("EnumTestListEnum there's no enum type named {}", &list_name),
			}
		}
	}

#[derive(Debug, PartialEq)]
pub struct Refed1Table {
	ptr : TablePtr,
}
impl Refed1Table {
	pub fn new(ptr : TablePtr) -> Refed1Table{ Refed1Table{ ptr } } 
	pub fn first(&self) -> Refed1CItem {
		let ptr = table::get_value(self.ptr, "first").unwrap();
		Refed1CItem::from(ptr)
	}
	pub fn second(&self) -> Refed1CItem {
		let ptr = table::get_value(self.ptr, "second").unwrap();
		Refed1CItem::from(ptr)
	}
	pub fn from_id(&self, id : Refed1TableID) -> Refed1CItem{
		match id{
			Refed1TableID::First => self.first(),
			Refed1TableID::Second => self.second(),
		}
	}
}
#[repr(u64)] pub enum Refed1TableID{ First, Second, }
impl Refed1TableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"first" => Some(Self::First),
			"second" => Some(Self::Second),
			_ =>{ None }
		}
	}
	pub fn from_num(id : u64) -> Self{
		match id{
			0 => Self::First,
			1 => Self::Second,
			_ => panic!("invalid ID num {} Refed1TableID", id),
		}
	}
	pub fn len() -> u64{ 2 }
	pub fn to_str(&self) -> &str{
		match self{
			Refed1TableID::First => "first",
			Refed1TableID::Second => "second",
		}
	}
}
#[derive(Debug, PartialEq)]
pub struct Refed1CItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for Refed1CItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl Refed1CItem {
	pub fn mem(&self) -> i64{
		let qv = citem::get_int(self.ptr, "mem").unwrap();
		qv.into_value().unwrap()
	}
	
}

#[derive(Debug, PartialEq)]
pub struct Refed2Table {
	ptr : TablePtr,
}
impl Refed2Table {
	pub fn new(ptr : TablePtr) -> Refed2Table{ Refed2Table{ ptr } } 
	pub fn a(&self) -> Refed2CItem {
		let ptr = table::get_value(self.ptr, "a").unwrap();
		Refed2CItem::from(ptr)
	}
	pub fn b(&self) -> Refed2CItem {
		let ptr = table::get_value(self.ptr, "b").unwrap();
		Refed2CItem::from(ptr)
	}
	pub fn from_id(&self, id : Refed2TableID) -> Refed2CItem{
		match id{
			Refed2TableID::A => self.a(),
			Refed2TableID::B => self.b(),
		}
	}
}
#[repr(u64)] pub enum Refed2TableID{ A, B, }
impl Refed2TableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"a" => Some(Self::A),
			"b" => Some(Self::B),
			_ =>{ None }
		}
	}
	pub fn from_num(id : u64) -> Self{
		match id{
			0 => Self::A,
			1 => Self::B,
			_ => panic!("invalid ID num {} Refed2TableID", id),
		}
	}
	pub fn len() -> u64{ 2 }
	pub fn to_str(&self) -> &str{
		match self{
			Refed2TableID::A => "a",
			Refed2TableID::B => "b",
		}
	}
}
#[derive(Debug, PartialEq)]
pub struct Refed2CItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for Refed2CItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl Refed2CItem {
	pub fn mem(&self) -> i64{
		let qv = citem::get_int(self.ptr, "mem").unwrap();
		qv.into_value().unwrap()
	}
	
}

