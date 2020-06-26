use sabun_maker::intf::*;
use sabun_maker::structs::*;
pub struct RootItem {
    pub ptr : *mut RootObject,
    p_bu : Option<bool>,
    p_colData : Option<ColData>,
}
impl RootItem {
    pub fn bu(&mut self) -> bool{
        if let Some(v) = &self.p_bu{
            return v.clone();
        }
        let qv = root::get_bool(unsafe{ self.ptr.as_ref().unwrap() }, "bu").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_bu = Some(ans);
        return self.p_bu.clone().unwrap();
    }
    pub fn set_bu(&mut self, bu : bool) -> {
        self.p_bu = Some(bu.clone());	bool::set_bool(unsafe{ self.ptr.as_mut().unwrap() }, "bu", Qv::Val(bu));}
    pub fn colData(&mut self) -> ColData{
        if let Some(v) = &self.p_colData{
            return v.clone();
        }
        let qv = data::get_ColData(unsafe{ self.ptr.as_ref().unwrap() }, "colData").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_colData = Some(ans);
        return self.p_colData.clone().unwrap();
    }
}

pub struct ColData {
    pub ptr : *const ConstData,
}
impl ColData {
}

pub struct ColItem {
    pub ptr : *const ListItem,
    p_nakabu : Option<bool>,
}
impl ColItem {
    pub fn nakabu(&mut self) -> bool{
        if let Some(v) = &self.p_nakabu{
            return v.clone();
        }
        let qv = list_item::get_bool(unsafe{ self.ptr.as_ref().unwrap() }, "nakabu").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_nakabu = Some(ans);
        return self.p_nakabu.clone().unwrap();
    }
}
