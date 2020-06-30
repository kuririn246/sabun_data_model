use sabun_maker::intf::*;
use sabun_maker::structs::*;
#[derive(Debug, PartialEq, Clone)]
pub struct RootItem {
    pub ptr : RootObjectPtr,
    p_bu : Option<bool>,
    p_refed_data : Option<ConstDataPtr>,
    p_col_data : Option<ConstDataPtr>,
}
impl RootItem {
    pub fn new(ptr : RootObjectPtr) -> RootItem{
        RootItem{ ptr, p_bu : None, p_refed_data : None, p_col_data : None, }
    }
    pub fn bu(&mut self) -> bool{
        if let Some(v) = &self.p_bu{
            return v.clone();
        }
        let qv = root::get_bool(self.ptr, "bu").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_bu = Some(ans);
        return self.p_bu.clone().unwrap();
    }
    pub fn set_bu(&mut self, bu : bool){
        self.p_bu = Some(bu.clone());
        root::set_bool(self.ptr, "bu", Qv::Val(bu));
    }
    pub fn refed_data(&mut self) -> ConstDataPtr{
        if let Some(v) = &self.p_refed_data{
            return v.clone();
        }
        let ans = root::get_data(self.ptr, "refed_data").unwrap();
        self.p_refed_data = Some(ans);
        return self.p_refed_data.clone().unwrap();
    }
    pub fn col_data(&mut self) -> ConstDataPtr{
        if let Some(v) = &self.p_col_data{
            return v.clone();
        }
        let ans = root::get_data(self.ptr, "col_data").unwrap();
        self.p_col_data = Some(ans);
        return self.p_col_data.clone().unwrap();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefedData {
    pub ptr : ConstDataPtr,
}
impl RefedData {
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
}
impl RefedItem {
    pub fn new(ptr : ListItemPtr) -> RefedItem{
        RefedItem{ ptr, }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColData {
    pub ptr : ConstDataPtr,
}
impl ColData {
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
    pub fn nakabu(&mut self) -> bool{
        if let Some(v) = &self.p_nakabu{
            return v.clone();
        }
        let qv = list_item::get_bool(self.ptr, "nakabu").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_nakabu = Some(ans);
        return self.p_nakabu.clone().unwrap();
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