use sabun_maker::intf::*;
use sabun_maker::structs::*;
#[derive(Debug, PartialEq, Clone)]
pub struct RootItem {
    pub ptr : RootObjectPtr,
    p_bu : Option<bool>,
    p_colData : Option<ConstDataPtr>,
}
impl RootItem {
    pub fn new(ptr : RootObjectPtr) -> RootItem{
        RootItem{ ptr, p_bu : None, p_colData : None, }
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
    pub fn colData(&mut self) -> ConstDataPtr{
        if let Some(v) = &self.p_colData{
            return v.clone();
        }
        let ans = root::get_data(self.ptr, "colData").unwrap();
        self.p_colData = Some(ans);
        return self.p_colData.clone().unwrap();
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
}
impl ColItem {
    pub fn new(ptr : ListItemPtr) -> ColItem{
        ColItem{ ptr, p_nakabu : None, }
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
}