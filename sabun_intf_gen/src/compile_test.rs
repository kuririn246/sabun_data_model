use sabun_maker::intf::*;
use sabun_maker::structs::*;

pub struct RootIntf{
    obj : Box<RootObject>,
    intf : Box<RootItem>,
}
impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{
        let mut b = Box::new(obj);
        let intf = RootItem::new(RootObjectPtr::new(b.as_mut()));
        RootIntf{ obj : b, intf : Box::new(intf) }
    }
    pub fn intf(&mut self) -> &mut RootItem{ &mut self.intf }
    pub fn deconstruct(self) -> (Box<RootObject>, Box<RootItem>){ (self.obj, self.intf) }
}

#[derive(Debug, PartialEq)]
pub struct RootItem {
    pub ptr : RootObjectPtr,
    p_bu : Option<bool>,
    p_str : Option<String>,
    p_col : Option<ColList>,
    p_refed : Option<RefedData>,
}
impl RootItem {
    pub fn new(ptr : RootObjectPtr) -> RootItem{
        RootItem{ ptr, p_bu : None, p_str : None, p_col : None, p_refed : None, }
    }
    pub fn root(&mut self) -> *mut RootItem{ self }

    pub fn bu(&mut self) -> bool{
        if self.p_bu.is_some() {
            return self.p_bu.clone().unwrap();
        }
        let qv = root::get_bool(self.ptr, "bu").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_bu = Some(ans);
        return self.p_bu.clone().unwrap()
    }
    pub fn set_bu(&mut self, bu : bool){
        self.p_bu = Some(bu.clone());
        root::set_bool(self.ptr, "bu", Qv::Val(bu));
    }
    pub fn str(&mut self) -> &String{
        if self.p_str.is_some() {
            return self.p_str.as_ref().unwrap();
        }
        let qv = root::get_str(self.ptr, "str").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_str = Some(ans);
        return self.p_str.as_ref().unwrap();
    }
    pub fn set_str(&mut self, str : String){
        self.p_str = Some(str.clone());
        root::set_str(self.ptr, "str", Qv::Val(str));
    }
    pub fn col(&mut self) -> &mut ColList{
        if self.p_col.is_some() {
            return self.p_col.as_mut().unwrap();
        }
        let ans = root::get_list(self.ptr, "col").unwrap();
        self.p_col = Some(ColList::new(ans, self.root()));
        return self.p_col.as_mut().unwrap();
    }
    pub fn refed(&mut self) -> &mut RefedData{
        if self.p_refed.is_some() {
            return self.p_refed.as_mut().unwrap();
        }
        let ans = root::get_data(self.ptr, "refed").unwrap();
        self.p_refed = Some(RefedData::new(ans, self.root()));
        return self.p_refed.as_mut().unwrap();
    }
}



#[derive(Debug, PartialEq)]
pub struct ColItem {
    pub ptr : ListItemPtr,
    pub root : *mut RootItem,
    p_nakabu : Option<bool>,
    ref_refed : Option<*mut RefedItem>,
}
impl ColItem {
    pub fn new(ptr : ListItemPtr, root : *mut RootItem) -> ColItem{
        ColItem{ ptr, root, p_nakabu : None, ref_refed : None, }
    }
    pub fn root(&mut self) -> *mut RootItem{ self.root }

    pub fn nakabu(&mut self) -> bool{
        if self.p_nakabu.is_some() {
            return self.p_nakabu.clone().unwrap();
        }
        let qv = list_item::get_bool(self.ptr, "nakabu").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_nakabu = Some(ans);
        return self.p_nakabu.clone().unwrap()
    }
    pub fn ref_refed(&mut self) -> &mut RefedItem{
        if self.ref_refed.is_some() {
            return unsafe{ self.ref_refed.unwrap().as_mut().unwrap() };
        }
        let qv = list_item::get_ref(self.ptr, "refed").unwrap();
        let ref_id = if let Qv::Val(v) = qv{ v } else { unreachable!() };
        let mut root = unsafe{ self.root.as_mut().unwrap() };
        let ref_ptr : *mut RefedItem = root.refed().from_id(ref_id).unwrap();
        self.ref_refed = Some(ref_ptr);
        let pp = self.ref_refed.unwrap();
        unsafe{ pp.as_mut().unwrap() }
    }
}

#[derive(Debug, PartialEq)]
pub struct RefedData {
    pub ptr : ConstDataPtr,
    root : *mut RootItem,
    p_first : Option<RefedItem>,
    p_second : Option<RefedItem>,
}
impl RefedData {
    pub fn new(ptr : ConstDataPtr, root : *mut RootItem) -> RefedData{ RefedData{ ptr, root, p_first : None, p_second : None, } }
    pub fn root(&mut self) -> *mut RootItem{ self.root }
    pub fn first(&mut self) -> &mut RefedItem {
        if self.p_first.is_some() {
            return self.p_first.as_mut().unwrap();
        }
        let ptr = data::get_value(self.ptr, "first").unwrap();
        let item = RefedItem::new(ptr, self.root());
        self.p_first = Some(item);
        self.p_first.as_mut().unwrap()
    }
    pub fn second(&mut self) -> &mut RefedItem {
        if self.p_second.is_some() {
            return self.p_second.as_mut().unwrap();
        }
        let ptr = data::get_value(self.ptr, "second").unwrap();
        let item = RefedItem::new(ptr, self.root());
        self.p_second = Some(item);
        self.p_second.as_mut().unwrap()
    }
    pub fn from_id(&mut self, id : &str) -> Option<&mut RefedItem> {
        match id{
            "first" => Some(self.first()),
            "second" => Some(self.second()),
            _ =>{ None },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RefedItem {
    pub ptr : ListItemPtr,
    pub root : *mut RootItem,
    p_mem : Option<i64>,
}
impl RefedItem {
    pub fn new(ptr : ListItemPtr, root : *mut RootItem) -> RefedItem{
        RefedItem{ ptr, root, p_mem : None, }
    }
    pub fn root(&mut self) -> *mut RootItem{ self.root }

    pub fn mem(&mut self) -> i64{
        if self.p_mem.is_some() {
            return self.p_mem.clone().unwrap();
        }
        let qv = list_item::get_int(self.ptr, "mem").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_mem = Some(ans);
        return self.p_mem.clone().unwrap()
    }
}


