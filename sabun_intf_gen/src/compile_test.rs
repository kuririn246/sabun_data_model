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
    p_at_at_desu : Option<String>,
    p_col : Option<ColData>,
    p_atrefed : Option<AtrefedData>,
}
impl RootItem {
    pub fn new(ptr : RootObjectPtr) -> RootItem{
        RootItem{ ptr, p_bu : None, p_str : None, p_at_at_desu : None, p_col : None, p_atrefed : None, }
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
    pub fn at_at_desu(&mut self) -> &String{
        if self.p_at_at_desu.is_some() {
            return self.p_at_at_desu.as_ref().unwrap();
        }
        let qv = root::get_str(self.ptr, "@AtDesu").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_at_at_desu = Some(ans);
        return self.p_at_at_desu.as_ref().unwrap();
    }
    pub fn set_at_at_desu(&mut self, at_at_desu : String){
        self.p_at_at_desu = Some(at_at_desu.clone());
        root::set_str(self.ptr, "@AtDesu", Qv::Val(at_at_desu));
    }
    pub fn col(&mut self) -> &mut ColData{
        if self.p_col.is_some() {
            return self.p_col.as_mut().unwrap();
        }
        let ans = root::get_data(self.ptr, "col").unwrap();
        self.p_col = Some(ColData::new(ans, self.root()));
        return self.p_col.as_mut().unwrap();
    }
    pub fn atrefed(&mut self) -> &mut AtrefedData{
        if self.p_atrefed.is_some() {
            return self.p_atrefed.as_mut().unwrap();
        }
        let ans = root::get_data(self.ptr, "@refed").unwrap();
        self.p_atrefed = Some(AtrefedData::new(ans, self.root()));
        return self.p_atrefed.as_mut().unwrap();
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
    pub fn from_id(&mut self, id : &str) -> Option<&mut ColItem> {
        match id{
            "huga" => Some(self.huga()),
            _ =>{ None },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ColItem {
    pub ptr : ListItemPtr,
    pub root : *mut RootItem,
    p_nakabu : Option<bool>,
    ref_atrefed : Option<*mut AtrefedItem>,
}
impl ColItem {
    pub fn new(ptr : ListItemPtr, root : *mut RootItem) -> ColItem{
        ColItem{ ptr, root, p_nakabu : None, ref_atrefed : None, }
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
    pub fn ref_atrefed(&mut self) -> &mut AtrefedItem{
        if self.ref_atrefed.is_some() {
            return unsafe{ self.ref_atrefed.unwrap().as_mut().unwrap() };
        }
        let qv = list_item::get_ref(self.ptr, "@refed").unwrap();
        let ref_id = if let Qv::Val(v) = qv{ v } else { unreachable!() };
        let mut root = unsafe{ self.root.as_mut().unwrap() };
        let ref_ptr : *mut AtrefedItem = root.atrefed().from_id(ref_id).unwrap();
        self.ref_atrefed = Some(ref_ptr);
        let pp = self.ref_atrefed.unwrap();
        unsafe{ pp.as_mut().unwrap() }
    }
}

#[derive(Debug, PartialEq)]
pub struct AtrefedData {
    pub ptr : ConstDataPtr,
    root : *mut RootItem,
    p_atsecond : Option<AtrefedItem>,
    p_first : Option<AtrefedItem>,
}
impl AtrefedData {
    pub fn new(ptr : ConstDataPtr, root : *mut RootItem) -> AtrefedData{ AtrefedData{ ptr, root, p_atsecond : None, p_first : None, } }
    pub fn root(&mut self) -> *mut RootItem{ self.root }
    pub fn atsecond(&mut self) -> &mut AtrefedItem {
        if self.p_atsecond.is_some() {
            return self.p_atsecond.as_mut().unwrap();
        }
        let ptr = data::get_value(self.ptr, "@second").unwrap();
        let item = AtrefedItem::new(ptr, self.root());
        self.p_atsecond = Some(item);
        self.p_atsecond.as_mut().unwrap()
    }
    pub fn first(&mut self) -> &mut AtrefedItem {
        if self.p_first.is_some() {
            return self.p_first.as_mut().unwrap();
        }
        let ptr = data::get_value(self.ptr, "first").unwrap();
        let item = AtrefedItem::new(ptr, self.root());
        self.p_first = Some(item);
        self.p_first.as_mut().unwrap()
    }
    pub fn from_id(&mut self, id : &str) -> Option<&mut AtrefedItem> {
        match id{
            "@second" => Some(self.atsecond()),
            "first" => Some(self.first()),
            _ =>{ None },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AtrefedItem {
    pub ptr : ListItemPtr,
    pub root : *mut RootItem,
    p_mem : Option<i64>,
}
impl AtrefedItem {
    pub fn new(ptr : ListItemPtr, root : *mut RootItem) -> AtrefedItem{
        AtrefedItem{ ptr, root, p_mem : None, }
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


