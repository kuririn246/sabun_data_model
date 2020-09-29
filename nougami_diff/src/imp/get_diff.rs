use sabun_maker::structs::RootObject;
use crate::imp::structs::Diff;

pub fn get_diff(from : &RootObject, to : &RootObject) -> Diff{
    let f = from.sabun();
    let t = to.sabun();
    f.keys().
}