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
    p_refed : Option<RefedData>,
    p_col : Option<ColData>,
}
impl RootItem {
    pub fn new(ptr : RootObjectPtr) -> RootItem{
        RootItem{ ptr, p_bu : None, p_refed : None, p_col : None, }
    }
    pub fn root(&mut self) -> *mut RootItem{ self }

    pub fn bu(&mut self) -> bool{
        if self.p_bu.is_some() {
            return self.p_bu.unwrap();
        }
        let qv = root::get_bool(self.ptr, "bu").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_bu = Some(ans);
        return self.p_bu.unwrap()
    }
    pub fn set_bu(&mut self, bu : bool){
        self.p_bu = Some(bu);
        root::set_bool(self.ptr, "bu", Qv::Val(bu));
    }
    pub fn refed(&mut self) -> &mut RefedData{
        if self.p_refed.is_some() {
            return self.p_refed.as_mut().unwrap();
        }
        let ans = root::get_data(self.ptr, "refed").unwrap();
        self.p_refed = Some(RefedData::new(ans, self.root()));
        return self.p_refed.as_mut().unwrap();
    }
    pub fn col(&mut self) -> &mut ColData{
        if self.p_col.is_some() {
            return self.p_col.as_mut().unwrap();
        }
        let ans = root::get_data(self.ptr, "col").unwrap();
        self.p_col = Some(ColData::new(ans, self.root()));
        return self.p_col.as_mut().unwrap();
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
    pub fn from_id(&mut self, id : &str) -> &mut RefedItem {
        match id{
            "first" => self.first(),
            "second" => self.second(),
            _ =>{ unreachable!() },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RefedItem {
    pub ptr : ListItemPtr,
    pub root : *mut RootItem,
    p_mem : Option<f64>,
}
impl RefedItem {
    pub fn new(ptr : ListItemPtr, root : *mut RootItem) -> RefedItem{
        RefedItem{ ptr, root, p_mem : None, }
    }
    pub fn root(&mut self) -> *mut RootItem{ self.root }

    pub fn mem(&mut self) -> f64{
        if self.p_mem.is_some() {
            return self.p_mem.unwrap();
        }
        let qv = list_item::get_num(self.ptr, "mem").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_mem = Some(ans);
        return self.p_mem.unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub struct ColData {
    pub ptr : ConstDataPtr,
    root : *mut RootItem,
    p_huga : Option<ColItem>,
}
impl ColData {
    pub fn new(ptr : ConstDataPtr, root : *mut RootItem) -> ColData{ ColData{ ptr, root, p_huga : None, } }
    pub fn root(&mut self) -> *mut RootItem{ self.root }
    pub fn huga(&mut self) -> &mut ColItem {
        if self.p_huga.is_some() {
            return self.p_huga.as_mut().unwrap();
        }
        let ptr = data::get_value(self.ptr, "huga").unwrap();
        let item = ColItem::new(ptr, self.root());
        self.p_huga = Some(item);
        self.p_huga.as_mut().unwrap()
    }
    pub fn from_id(&mut self, id : &str) -> &mut ColItem {
        match id{
            "huga" => self.huga(),
            _ =>{ unreachable!() },
        }
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
            return self.p_nakabu.unwrap();
        }
        let qv = list_item::get_bool(self.ptr, "nakabu").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_nakabu = Some(ans);
        return self.p_nakabu.unwrap()
    }
    pub fn ref_refed(&mut self) -> &mut RefedItem{
        if self.ref_refed.is_some() {
            return unsafe{ self.ref_refed.unwrap().as_mut().unwrap() };
        }
        let qv = list_item::get_ref(self.ptr, "refed").unwrap();
        let ref_id = if let Qv::Val(v) = qv{ v } else { unreachable!() };
        let mut root = unsafe{ self.root.as_mut().unwrap() };
        let ref_ptr : *mut RefedItem = root.refed().from_id(ref_id);
        self.ref_refed = Some(ref_ptr);
        let pp = self.ref_refed.unwrap();
        unsafe{ pp.as_mut().unwrap() }
    }
}


