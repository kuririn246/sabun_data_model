use sabun_maker::intf::*;
use sabun_maker::structs::*;
pub struct Root { pub ptr : *mut RootObject }
impl Root {
    pub fn bu(&self) -> bool {
        let qv = root::get_bool(unsafe{ self.ptr.as_ref().unwrap() }, "bu").unwrap();
        qv.into_value().unwrap()
    }
    pub fn set_bu(&mut self, bu : bool){
        root::set_bool(unsafe{ self.ptr.as_mut().unwrap() }, "bu", Qv::Val(bu));
    }
}