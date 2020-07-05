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
    p_bbikkuri : Option<UndefOr<bool>>,
    p_sbikkuri : Option<UndefOr<String>>,
    p_sbh : Option<Qv<String>>,
    p_s : Option<String>,
    p_bbh : Option<Qv<bool>>,
    p_shatena : Option<NullOr<String>>,
    p_n : Option<f64>,
    p_bhatena : Option<NullOr<bool>>,
    p_b : Option<bool>,
}
impl RootItem {
    pub fn new(ptr : RootObjectPtr) -> RootItem{
        RootItem{ ptr, p_bbikkuri : None, p_sbikkuri : None, p_sbh : None, p_s : None, p_bbh : None, p_shatena : None, p_n : None, p_bhatena : None, p_b : None, }
    }
    pub fn root(&mut self) -> *mut RootItem{ self }

    pub fn bbikkuri(&mut self) -> UndefOr<bool>{
        if self.p_bbikkuri.is_some() {
            return self.p_bbikkuri.clone().unwrap();
        }
        let qv = root::get_bool(self.ptr, "bbikkuri").unwrap();
        let ans = UndefOr::from_qv(qv).unwrap();
        self.p_bbikkuri = Some(ans);
        return self.p_bbikkuri.clone().unwrap()
    }
    pub fn set_bbikkuri(&mut self, bbikkuri : UndefOr<bool>){
        self.p_bbikkuri = Some(bbikkuri.clone());
        root::set_bool(self.ptr, "bbikkuri", bbikkuri.into_qv());
    }
    pub fn sbikkuri(&mut self) -> &UndefOr<String>{
        if self.p_sbikkuri.is_some() {
            return self.p_sbikkuri.as_ref().unwrap();
        }
        let qv = root::get_str(self.ptr, "sbikkuri").unwrap();
        let ans = UndefOr::from_qv(qv).unwrap();
        self.p_sbikkuri = Some(ans);
        return self.p_sbikkuri.as_ref().unwrap();
    }
    pub fn set_sbikkuri(&mut self, sbikkuri : UndefOr<String>){
        self.p_sbikkuri = Some(sbikkuri.clone());
        root::set_str(self.ptr, "sbikkuri", sbikkuri.into_qv());
    }
    pub fn sbh(&mut self) -> &Qv<String>{
        if self.p_sbh.is_some() {
            return self.p_sbh.as_ref().unwrap();
        }
        let qv = root::get_str(self.ptr, "sbh").unwrap();
        let ans = qv;
        self.p_sbh = Some(ans);
        return self.p_sbh.as_ref().unwrap();
    }
    pub fn set_sbh(&mut self, sbh : Qv<String>){
        self.p_sbh = Some(sbh.clone());
        root::set_str(self.ptr, "sbh", sbh.into_qv());
    }
    pub fn s(&mut self) -> &String{
        if self.p_s.is_some() {
            return self.p_s.as_ref().unwrap();
        }
        let qv = root::get_str(self.ptr, "s").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_s = Some(ans);
        return self.p_s.as_ref().unwrap();
    }
    pub fn set_s(&mut self, s : String){
        self.p_s = Some(s.clone());
        root::set_str(self.ptr, "s", Qv::Val(s));
    }
    pub fn bbh(&mut self) -> Qv<bool>{
        if self.p_bbh.is_some() {
            return self.p_bbh.clone().unwrap();
        }
        let qv = root::get_bool(self.ptr, "bbh").unwrap();
        let ans = qv;
        self.p_bbh = Some(ans);
        return self.p_bbh.clone().unwrap()
    }
    pub fn set_bbh(&mut self, bbh : Qv<bool>){
        self.p_bbh = Some(bbh.clone());
        root::set_bool(self.ptr, "bbh", bbh.into_qv());
    }
    pub fn shatena(&mut self) -> &NullOr<String>{
        if self.p_shatena.is_some() {
            return self.p_shatena.as_ref().unwrap();
        }
        let qv = root::get_str(self.ptr, "shatena").unwrap();
        let ans = NullOr::from_qv(qv).unwrap();
        self.p_shatena = Some(ans);
        return self.p_shatena.as_ref().unwrap();
    }
    pub fn set_shatena(&mut self, shatena : NullOr<String>){
        self.p_shatena = Some(shatena.clone());
        root::set_str(self.ptr, "shatena", shatena.into_qv());
    }
    pub fn n(&mut self) -> f64{
        if self.p_n.is_some() {
            return self.p_n.clone().unwrap();
        }
        let qv = root::get_num(self.ptr, "n").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_n = Some(ans);
        return self.p_n.clone().unwrap()
    }
    pub fn set_n(&mut self, n : f64){
        self.p_n = Some(n.clone());
        root::set_num(self.ptr, "n", Qv::Val(n));
    }
    pub fn bhatena(&mut self) -> NullOr<bool>{
        if self.p_bhatena.is_some() {
            return self.p_bhatena.clone().unwrap();
        }
        let qv = root::get_bool(self.ptr, "bhatena").unwrap();
        let ans = NullOr::from_qv(qv).unwrap();
        self.p_bhatena = Some(ans);
        return self.p_bhatena.clone().unwrap()
    }
    pub fn set_bhatena(&mut self, bhatena : NullOr<bool>){
        self.p_bhatena = Some(bhatena.clone());
        root::set_bool(self.ptr, "bhatena", bhatena.into_qv());
    }
    pub fn b(&mut self) -> bool{
        if self.p_b.is_some() {
            return self.p_b.clone().unwrap();
        }
        let qv = root::get_bool(self.ptr, "b").unwrap();
        let ans = qv.into_value().unwrap();
        self.p_b = Some(ans);
        return self.p_b.clone().unwrap()
    }
    pub fn set_b(&mut self, b : bool){
        self.p_b = Some(b.clone());
        root::set_bool(self.ptr, "b", Qv::Val(b));
    }
}


