// use sabun_maker::intf::*;
// use sabun_maker::structs::*;
// pub struct Root {
//     pub ptr : *mut RootObject,
//     p_bu : Option<bool>,
// }
// impl Root {
//     pub fn bu(&mut self) -> bool {
//         if let Some(v) = &self.p_bu{
//             return v.clone();
//         }
//         let qv = root::get_bool(unsafe{ self.ptr.as_ref().unwrap() }, "bu").unwrap();
//         let ans = qv.into_value().unwrap();
//         self.p_bu = Some(ans);
//         return self.p_bu.clone().unwrap();
//     }
//     pub fn set_bu(&mut self, bu : bool){
//         self.p_bu = Some(bu.clone());
//         root::set_bool(unsafe{ self.ptr.as_mut().unwrap() }, "bu", Qv::Val(bu));
//     }
// }