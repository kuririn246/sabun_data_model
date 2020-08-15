use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;
use crate::{HashM, HashS, HashSt, HashMt};
use json5_parser::Span;
use crate::error::Result;
use crate::imp::structs::rust_list::{ConstList, InnerList, ConstData, MutList, InnerMutList, ListItem, MutListItem};
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::inner_mut_def_obj::InnerMutDefObj;
use crate::imp::structs::linked_m::LinkedMap;

pub struct TmpList{
    pub vec : Vec<TmpObj>,
    ///複数回定義のエラーを検出したいのでOptionにする
    pub old : Option<HashS<String>>,
    pub default : Option<ListDefObj>,
    pub compatible : Option<HashS<String>>,
    pub next_id : Option<u64>,
    pub span : Span,
}

impl TmpList{
    pub fn new(capacity : usize, span : Span) -> TmpList{
        TmpList{ vec : Vec::with_capacity(capacity), old : None, default : None, compatible : None, next_id : None, span }
    }

    pub fn into_const_list(self) -> Result<ConstList>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for a List {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for a List {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID must not be defined {}", self.span.line_str(), self.span.slice()))?
        }

        Ok(ConstList::new(self.default.unwrap(),to_list_items(self.vec)?))
    }



    pub fn into_inner_list(self) -> Result<InnerList>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for InnerList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for InnerList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_some(){
            Err(format!("{} Default must not be defined for InnerList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID must not be defined for InnerList {}", self.span.line_str(), self.span.slice()))?
        }

        Ok(InnerList::new(to_list_items(self.vec)?))
    }

    pub fn into_const_data(self) -> Result<ConstData>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for Data {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID must not be defined {}", self.span.line_str(), self.span.slice()))?
        }
        let old = self.old.unwrap_or_else(|| HashSt::new());

        Ok(ConstData::new(self.default.unwrap(), to_data_items(self.vec)?,  old))
    }
    // pub fn into_inner_data(self) -> Result<InnerData>{
    //     if self.compatible.is_some(){
    //         Err(format!("{} Compatible is not needed for Data {}", self.span.line_str(), self.span.slice()))?
    //     }
    //     if self.default.is_some(){
    //         Err(format!("{} Default must not be defined {}", self.span.line_str(), self.span.slice()))?
    //     }
    //     if self.next_id.is_some(){
    //         Err(format!("{} NextID must not be defined {}", self.span.line_str(), self.span.slice()))?
    //     }
    //     let old = self.old.unwrap_or_else(|| HashSt::new());
    //
    //     Ok(InnerData::new(to_data_items(self.vec)?, old))
    // }

    pub fn into_mut_list(self) -> Result<MutList>{
        if self.old.is_some(){
            Err(format!("{} Old is not needed for MutList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        // mut_listのときだけnext_idを消す処理が難しいしめんどいので無視してしまう・・・
        //if self.next_id.is_some(){
          //  Err(format!("{} NextID is not needed for MutList {}", self.span.line_str(), self.span.slice()))?
        //}
        if self.vec.len() != 0{
            Err(format!("{} MutList must not have items {}", self.span.line_str(), self.span.slice()))?
        }
        let compatible = self.compatible.unwrap_or_else(|| HashSt::new());
        Ok(MutList::new(self.default.unwrap(),LinkedMap::new(), compatible))
    }

    pub fn into_inner_mut_list(self) -> Result<InnerMutList>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for InnerMutList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for InnerMutList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_some(){
            Err(format!("{} Default must not be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID is not needed for InnerMutList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.vec.len() != 0{
            Err(format!("{} InnerMutList must not have items {}", self.span.line_str(), self.span.slice()))?
        }
        Ok(InnerMutList::new(LinkedMap::new()))
    }

    ///MutListは中身があってはいけないのだが、そのルールを破壊する裏道が用意されている。
    pub fn into_violated_list(self) -> Result<MutList>{

        if self.old.is_some(){
            Err(format!("{} Old is not needed for ViolatedList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }

        let next_id = self.next_id.unwrap_or(self.vec.len() as u64);

        let items = to_violated_list_items(self.vec, next_id)?;
        let compatible = self.compatible.unwrap_or_else(|| HashSt::new());

        Ok(MutList::new(self.default.unwrap(), items , compatible))
    }

    ///MutListは中身があってはいけないのだが、そのルールを破壊する裏道が用意されている。
    pub fn into_inner_violated_list(self) -> Result<InnerMutList>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for InnerViolatedList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for ViolatedList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_some(){
            Err(format!("{} Default must not be defined {}", self.span.line_str(), self.span.slice()))?
        }

        let next_id = self.next_id.unwrap_or(self.vec.len() as u64);

        let items = to_violated_list_items(self.vec, next_id)?;

        Ok(InnerMutList::new(items))
    }

    pub fn into_inner_def(self) -> Result<ListDefObj>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.vec.len() != 0{
            Err(format!("{} InnerDef must not have items {}", self.span.line_str(), self.span.slice()))?
        }

        Ok(self.default.unwrap())
    }

    pub fn into_inner_mut_def(self, undefinable : bool) -> Result<InnerMutDefObj>{
        if self.old.is_some(){
            Err(format!("{} Old is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.vec.len() != 0{
            Err(format!("{} InnerDef must not have items {}", self.span.line_str(), self.span.slice()))?
        }
        let compatible = self.compatible.unwrap_or_else(|| HashSt::new());
        Ok(InnerMutDefObj::new(self.default.unwrap(), undefinable, compatible))
    }
}


fn to_list_items(vec : Vec<TmpObj>) -> Result<Vec<ListItem>>{
    let mut result : Vec<ListItem> = Vec::with_capacity(vec.len());
    for item in vec{
        result.push(item.into_list_item()?);
    }
    return Ok(result);
}

fn to_data_items(vec : Vec<TmpObj>) -> Result<HashM<String, ListItem>>{
    let mut result : HashM<String, ListItem> = HashMt::with_capacity(vec.len());
    for item in vec{
        let (s,m) = item.into_list_item_with_id()?;
        result.insert(s, m);
    }
    return Ok(result);
}

fn to_violated_list_items(vec : Vec<TmpObj>, next_id : u64) -> Result<LinkedMap<MutListItem>>{
    let mut result : Vec<(u64, MutListItem)> = Vec::with_capacity(vec.len());
    for (idx, tmp_item) in vec.into_iter().enumerate(){
        //let span = tmp_item.span.clone();
        let item = tmp_item.into_violated_list_item(idx)?;
        result.push((idx as u64, item));
    }
    return Ok(LinkedMap::construct(result, next_id));
}