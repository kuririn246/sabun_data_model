use crate::imp::structs::rust_list::{ConstData, ListItem, ConstList};
use crate::{HashM, HashS};
use crate::imp::intf::list_item::ListItemPtr;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use std::ops::Index;

// pub fn get_member_desc(root : *const ConstData) -> MemberDescs{
//     let root = unsafe{ root.as_ref().unwrap() };
//     get_list_def_desc(root.default())
// }

// pub fn get_ref_desc(root : *const ConstData) -> RefDescs{
//     let root = unsafe{ root.as_ref().unwrap() };
//     get_ref_def_desc(root.default().refs())
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ConstListPtr{
    ptr : *const ConstList,
    root : *const RootObject,
}
impl ConstListPtr{
    pub fn new(ptr : *const ConstList, root : *const RootObject) -> ConstListPtr{ ConstListPtr{ ptr, root } }
}

pub fn get_len(list: ConstListPtr) -> usize{
    let d = unsafe{ list.ptr.as_ref().unwrap()};
    get_len_impl(d.list())
}

pub fn get_len_impl(list: *const Vec<ListItem>) -> usize{
    let d = unsafe{ list.as_ref().unwrap()};
    d.len()
}


pub fn get_value(list: ConstListPtr, idx : usize) -> Option<ListItemPtr>{
    let d = unsafe{ list.ptr.as_ref().unwrap()};
    get_value_impl(d.list(), d.default(), idx, list.root)
}

pub fn get_value_impl(vec: *const Vec<ListItem>, list_def : *const ListDefObj, idx : usize, root : *const RootObject) -> Option<ListItemPtr>{
    let vec = unsafe{ vec.as_ref().unwrap() };
    vec.get(idx).map(|item| ListItemPtr::new(item, list_def, root))
}

#[derive(Debug, PartialEq)]
pub struct ConstListIntf<T> {
    pub vec : *const Vec<ListItem>,
    pub def : *const ListDefObj,
    pub root : *const RootObject,
    proxy_vec : Option<Vec<T>>,
}
impl<T> ConstListIntf<T> {
    pub fn from_const_list(ptr : ConstListPtr, root : *const RootObject) -> ConstListIntf<T>{
        let ptr = unsafe{ ptr.ptr.as_ref().unwrap() };
        ConstListIntf{ vec : ptr.list(), def : ptr.default(), root, proxy_vec : None }
    }
    pub fn len(&self) -> usize{ get_len_impl(self.vec) }
    pub fn get(&mut self, index : usize, getter : impl Fn(ListItemPtr) -> T) -> &T{
        let p = self.proxy(getter);
        p.get(index).unwrap()
    }
    pub fn iter(&mut self, getter : impl Fn(ListItemPtr) -> T) -> std::slice::Iter<T>{
        let p = self.proxy(getter);
        p.iter()
    }
    fn proxy(&mut self, getter : impl Fn(ListItemPtr) -> T) -> &mut Vec<T>{
        if self.proxy_vec.is_none(){
            let mut vec : Vec<T> = Vec::with_capacity(self.len());
            for i in 0..self.len(){
                let ptr = get_value_impl(self.vec, self.def, i, self.root).unwrap();
                let value = getter(ptr);
                vec.push(value);
            }
            self.proxy_vec = Some(vec);
        }
        self.proxy_vec.as_mut().unwrap()
    }

}



#[derive(Debug)]
pub struct ConstListIter{
    counter : usize,
    ptr : ConstListPtr,
}

impl ConstListIter{
    pub fn new(ptr : ConstListPtr) -> ConstListIter{ ConstListIter{ counter : 0, ptr } }
}

impl Iterator for ConstListIter{
    type Item = ListItemPtr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < get_len(self.ptr) {
            let counter = self.counter;
            self.counter += 1;
            return Some(get_value(self.ptr, counter).unwrap())
        } else{
            return None;
        }
    }
}