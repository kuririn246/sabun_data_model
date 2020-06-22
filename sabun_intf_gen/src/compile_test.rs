use sabun_maker::intf::*;
use sabun_maker::structs::*;
pub struct Root { pub ptr : *mut RootObject }
impl Root {
    pub fn bu(&self) -> bool {
        root::get_bool(unsafe{ self.ptr.as_ref().unwrap() }, "bu");
    }
    pub fn set_bu(&mut self, bu : Qv<bool>){
        root::set_bool(unsafe{ self.ptr.as_mut_ref().unwrap() }, "bu", bu);
    }
}