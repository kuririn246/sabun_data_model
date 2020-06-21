use crate::imp::structs::list_def_obj::ListDefObj;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
pub struct InnerMutDefObj {
    list_def : Box<ListDefObj>,
    undefinable: bool,
    compatible : Box<HashSet<String>>,
}

impl InnerMutDefObj{
    pub(crate) fn new(list_def : ListDefObj, undefinable : bool, compatible : HashSet<String>) -> InnerMutDefObj{
        InnerMutDefObj{ list_def : Box::new(list_def), undefinable, compatible : Box::new(compatible) }
    }
    pub(crate) fn list_def(&self) -> &ListDefObj{ self.list_def.as_ref() }
    pub(crate) fn undefinable(&self) -> bool{ self.undefinable }
    pub(crate) fn compatible(&self) -> &HashSet<String>{ self.compatible.as_ref() }
}

