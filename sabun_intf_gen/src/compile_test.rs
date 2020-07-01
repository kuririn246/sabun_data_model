use sabun_maker::intf::*;
use sabun_maker::structs::*;
#[derive(Debug, PartialEq, Clone)]
pub struct RootItem {
    pub ptr : RootObjectPtr,
    p_bu : Option<bool>,
    p_refed_data : Option<RefedData>,
    p_col_data : Option<ColData>,
}
impl RootItem {
    pub fn new(ptr : RootObjectPtr) -> RootItem{
        RootItem{ ptr, p_bu : None, p_refed_data : None, p_col_data : None, }
    }
    pub fn bu(&self) -> &bool{
        if let Some(v) = &self.p_bu{
            return v;
        }
        let qv = root::get_bool(self.ptr, "bu").unwrap();
        let ans = qv.into_value().unwrap();
        let mp = unsafe { (self as *const RootItem as *mut RootItem).as_mut().unwrap() };
        mp.p_bu = Some(ans);
        return self.p_bu.as_ref().unwrap();
    }
    pub fn set_bu(&mut self, bu : bool){
        self.p_bu = Some(bu.clone());
        root::set_bool(self.ptr, "bu", Qv::Val(bu));
    }
    pub fn refed_data(&mut self) -> RefedData{
        if let Some(v) = &self.p_refed_data{
            return v.clone();
        }
        let ans = root::get_data(self.ptr, "refed_data").unwrap();
        self.p_refed_data = Some(RefedData::new(ans));
        return self.p_refed_data.clone().unwrap();
    }
    pub fn col_data(&mut self) -> ColData{
        if let Some(v) = &self.p_col_data{
            return v.clone();
        }
        let ans = root::get_data(self.ptr, "col_data").unwrap();
        self.p_col_data = Some(ColData::new(ans));
        return self.p_col_data.clone().unwrap();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefedData {
    pub ptr : ConstDataPtr,
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefedItem {
    pub ptr : ListItemPtr,
    p_mem : Option<f64>,
}
impl RefedItem {
    pub fn new(ptr : ListItemPtr) -> RefedItem{
        RefedItem{ ptr, p_mem : None, }
    }
    pub fn mem(&self) -> &f64{
        if let Some(v) = &self.p_mem{
            return v;
        }
        let qv = list_item::get_num(self.ptr, "mem").unwrap();
        let ans = qv.into_value().unwrap();
        let mp = unsafe { (self as *const RefedItem as *mut RefedItem).as_mut().unwrap() };
        mp.p_mem = Some(ans);
        return self.p_mem.as_ref().unwrap();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColData {
    pub ptr : ConstDataPtr,
}
impl ColData {
    pub fn new(ptr : ConstDataPtr) -> ColData{ ColData{ ptr } }
    pub fn huga(&self) -> ColItem {
        let ptr = data::get_value(self.ptr, "huga").unwrap();
        ColItem::new(ptr)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColItem {
    pub ptr : ListItemPtr,
    p_nakabu : Option<bool>,
    ref_refed : Option<RefedItem>,
}
impl ColItem {
    pub fn new(ptr : ListItemPtr) -> ColItem{
        ColItem{ ptr, p_nakabu : None, ref_refed : None, }
    }
    pub fn nakabu(&self) -> &bool{
        if let Some(v) = &self.p_nakabu{
            return v;
        }
        let qv = list_item::get_bool(self.ptr, "nakabu").unwrap();
        let ans = qv.into_value().unwrap();
        let mp = unsafe { (self as *const ColItem as *mut ColItem).as_mut().unwrap() };
        mp.p_nakabu = Some(ans);
        return self.p_nakabu.as_ref().unwrap();
    }
    pub fn ref_refed(&mut self) -> RefedItem{
        if let Some(v) = &self.ref_refed{
            return v.clone();
        }
        let qv = list_item::get_ref(self.ptr, "refed").unwrap();
        let ans = qv.into_value().unwrap();
        self.ref_refed = Some(RefedItem::new(ans));
        return self.ref_refed.clone().unwrap();
    }
}
