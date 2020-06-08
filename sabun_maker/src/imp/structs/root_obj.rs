use std::collections::{HashMap, HashSet};
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::rust_param::RustParam;

#[derive(Debug, PartialEq)]
pub struct RootObject{
    ///listのobjectの場合、defaultはlist側にあるが、ここには初期値が入る。
    default : Box<HashMap<String, RootValue>>,
    ///変更されたものを記録
    ///listの変更はMutListが直接上書きされるので、sabunには入らない。よってparamだけ記録される
    sabun : Box<HashMap<String, RustParam>>,

    ///oldに設定されたメンバは、_Oldを付けなければプログラムから使用できず、
    ///ConstDataである場合、jsonで Refできない
    old : Box<HashSet<String>>,
}

impl RootObject{
    pub(crate) fn new(default : HashMap<String, RootValue>, sabun : HashMap<String, RustParam>, old : HashSet<String>) -> RootObject{
        RootObject{ default: Box::new(default), sabun : Box::new(sabun), old : Box::new(old) }
    }
    pub(crate) fn default(&self) -> &HashMap<String, RootValue>{ self.default.as_ref() }
    pub(crate) fn deconstruct(self) -> (HashMap<String, RootValue>, HashMap<String, RustParam>, HashSet<String>){ (*self.default, *self.sabun, *self.old) }
    pub(crate) fn sabun(&self) -> &HashMap<String, RustParam>{ self.sabun.as_ref() }
    pub(crate) fn old(&self) -> &HashSet<String>{ self.old.as_ref() }
}