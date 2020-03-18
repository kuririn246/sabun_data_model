use std::collections::{BTreeMap};



pub enum RenameMapError{
    DuplicatedOld(String),
    CircularReference(String),
}


pub fn rename_map(input : Vec<(String, String)>) -> Result<BTreeMap<String, String>, RenameMapError> {
    let mut map: BTreeMap<String, String> = BTreeMap::new();

    for (old, new) in input {
        if map.contains_key(&old) {
            return Err(RenameMapError::DuplicatedOld(old));
        }
        map.insert(old, new);
    }

    let mut new_map: BTreeMap<String, String> = BTreeMap::new();

    for (old, new) in &map {
        let mut new = new;
        loop {
            if let Some(new2) = map.get(new) {
                if new2 == old {
                    return Err(RenameMapError::CircularReference(new2.to_string()))
                }
                new = new2;
            } else {
                new_map.insert(old.to_string(), new.to_string());
                break;
            }
        }
    }

    Ok(new_map)
}