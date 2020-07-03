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
    pub fn root(&self) -> *const RootItem{ self }

    pub fn bu(&mut self) -> bool{
        if let Some(v) = &self.p_bu{
            return v.clone();
        }
        let qv = root::get_bool(self.ptr, "bu").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_bu = Some(ans);
        return self.p_bu.as_ref().unwrap().clone();
    }
    pub fn set_bu(&mut self, bu : bool){
        self.p_bu = Some(bu);
        root::set_bool(self.ptr, "bu", Qv::Val(bu));
    }
    pub fn refed(&self) -> &RefedData{
        if let Some(v) = &self.p_refed{
            return v;
        } else {
            let ans = root::get_data(self.ptr, "refed").unwrap();
            let m = unsafe{ (self as *const RefedData as *mut RefedData).as_mut().unwrap() };
            m.p_refed = Some(RefedData::new(ans, self.root()));
            return self.p_refed.as_ref().unwrap();
        }
    }
    pub fn col(&mut self) -> &ColData{
        if let Some(v) = &self.p_col{
            return v;
        }
        let ans = root::get_data(self.ptr, "col").unwrap();
        self.p_col = Some(ColData::new(ans, self.root()));
        return self.p_col.as_ref().unwrap();
    }
}

#[derive(Debug, PartialEq)]
pub struct RefedData {
    pub ptr : ConstDataPtr,
    root : *const RootItem,
    p_first : Option<RefedItem>,
    p_second : Option<RefedItem>,
}
impl RefedData {
    pub fn new(ptr : ConstDataPtr, root : *const RootItem) -> RefedData{ RefedData{ ptr, root, p_first : None, p_second : None, } }
    pub fn root(&self) -> *const RootItem{ self.root }
    pub fn first(&self) -> &RefedItem {
        if let Some(item) = &self.p_first {
            return item;
        }
        let ptr = data::get_value(self.ptr, "first").unwrap();
        let item = RefedItem::new(ptr, self.root());
        self.p_first = Some(item);
        self.p_first.as_ref().unwrap()
    }
    pub fn second(&mut self) -> &RefedItem {
        if let Some(item) = &self.p_second {
            return item;
        }
        let ptr = data::get_value(self.ptr, "second").unwrap();
        let item = RefedItem::new(ptr, self.root());
        self.p_second = Some(item);
        self.p_second.as_ref().unwrap()
    }
    pub fn from_id(&mut self, id : &str) -> &RefedItem {
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
    pub root : *const RootItem,
    p_mem : Option<f64>,
}
impl RefedItem {
    pub fn new(ptr : ListItemPtr, root : *const RootItem) -> RefedItem{
        RefedItem{ ptr, root, p_mem : None, }
    }
    pub fn root(&self) -> *const RootItem{ self.root }

    pub fn mem(&self) -> f64{
        if let Some(v) = &self.p_mem{
            return v.clone();
        }
        let qv = list_item::get_num(self.ptr, "mem").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_mem = Some(ans);
        return self.p_mem.as_ref().unwrap().clone();
    }
}

#[derive(Debug, PartialEq)]
pub struct ColData {
    pub ptr : ConstDataPtr,
    root : *const RootItem,
    p_huga : Option<ColItem>,
}
impl ColData {
    pub fn new(ptr : ConstDataPtr, root : *mut RootItem) -> ColData{ ColData{ ptr, root, p_huga : None, } }
    pub fn root(&self) -> *const RootItem{ self.root }
    pub fn huga(&self) -> &ColItem {
        if let Some(item) = &self.p_huga {
            return item;
        }
        let ptr = data::get_value(self.ptr, "huga").unwrap();
        let item = ColItem::new(ptr, self.root());
        let m = unsafe{ (self as *const ColData as *mut ColData).as_mut().unwrap() };
        m.p_huga = Some(item);
        self.p_huga.as_ref().unwrap()
    }
    pub fn from_id(&mut self, id : &str) -> &ColItem {
        match id{
            "huga" => self.huga(),
            _ =>{ unreachable!() },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ColItem {
    pub ptr : ListItemPtr,
    pub root : *const RootItem,
    p_nakabu : Option<bool>,
    ref_refed : Option<*const RefedItem>,
}
impl ColItem {
    pub fn new(ptr : ListItemPtr, root : *const RootItem) -> ColItem{
        ColItem{ ptr, root, p_nakabu : None, ref_refed : None, }
    }
    pub fn root(&self) -> *const RootItem{ self.root }

    pub fn nakabu(&self) -> bool{
        if let Some(v) = &self.p_nakabu{
            return v.clone();
        }
        let qv = list_item::get_bool(self.ptr, "nakabu").unwrap();
        let ans = qv.into_value().unwrap();
        let m = unsafe{ (self as *const ColItem as *mut ColItem).as_mut().unwrap() };
        m.p_nakabu = Some(ans);
        return self.p_nakabu.as_ref().unwrap().clone();
    }
    pub fn ref_refed(&self) -> &RefedItem{
        if let Some(v) = self.ref_refed{
            return unsafe{ v.as_ref().unwrap() };
        }
        let qv = list_item::get_ref(self.ptr, "refed").unwrap();
        let ref_id = if let Qv::Val(v) = qv{ v } else { unreachable!() };
        let root = unsafe{ self.root.as_ref().unwrap() };
        let ref_ptr : *const RefedItem = root.refed().from_id(ref_id);
        let m = unsafe{ (self as *const ColItem as *mut ColItem).as_mut().unwrap() };
        m.ref_refed = Some(ref_ptr);
        let pp = self.ref_refed.unwrap();
        unsafe{ pp.as_ref().unwrap() }
    }
}


