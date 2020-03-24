
use std::fs::File;

use std::io::prelude::*;
use crate::error::Result;
use crate::rust_struct::{JsonFile, RustValue, RustObject};
use std::ffi::{OsStr};
use crate::imp::json_to_rust::{json_root_to_rust, json_item_str_to_rust};
use std::collections::HashMap;
use crate::imp::json_to_rust::validation::construct_root::construct_root;

pub fn json_dir_to_rust(dir_path : &str, validation : bool) -> Result<RustObject>{
    let dirs = std::fs::read_dir(dir_path)?;

    let mut vec : Vec<JsonFile> = vec![];

    for dir in dirs{
        match dir{
            Ok(de) =>{

                let path = de.path();

                let oss : &OsStr = path.file_stem().ok_or_else(|| format!("file stem couldn't be read {:?}", &de))?;
                let file_stem_ref = oss.to_str().ok_or_else(|| format!("os_string couldn't be converted to a rust string {:?}", oss))?;
                let file_stem = file_stem_ref.to_string();



                let mut file =  File::open(de.path())?;
                let mut buf = String::new();
                match file.read_to_string(&mut buf){
                    Ok(_) => vec.push(JsonFile{ json : buf, file_name_without_ext : file_stem }),
                    Err(e) =>{ Err(format!("{} couldn't be read", e))?; }
                };
            },
            Err(e) =>{
                //???
                Err(format!("{}", e.to_string()))?;
            }
        }
    }


    json_files_to_rust(vec.into_iter(), validation)


    // `file` goes out of scope, and the "hello.txt" file gets closed
}

pub fn json_files_to_rust(ite : impl Iterator<Item = JsonFile>, validation : bool) -> Result<RustObject>{
    let mut map : HashMap<String, RustValue> = HashMap::new();
    let mut root= None;

    for file in ite{
        let name = &file.file_name_without_ext;
        if name == "root"{
            if root.is_none() {
                root = Some(json_root_to_rust(&file.json)?);
            } else{
                Err("There's two 'root.json5's in the directory")? //unreachableだけど一応
            }
        } else{
            let val = json_item_str_to_rust(name, &file.json)?;
            map.insert(name.to_string(), val);
        }
    }

    if root.is_none(){
        Err("root.json5 is needed")?
    }

    return construct_root(root.unwrap(), map, validation);
}