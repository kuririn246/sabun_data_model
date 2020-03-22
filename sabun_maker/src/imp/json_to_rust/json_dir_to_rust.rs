
use std::error::Error;
use std::fs::File;

use std::io::prelude::*;
use std::path::Path;
use crate::error::Result;
use crate::rust_struct::JsonFile;
use std::ffi::{OsString, OsStr};

pub fn json_dir_to_rust(dir_path : &str) -> Result<()>{
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

    for item in vec {
        println!("{}", item.file_name_without_ext);
    }
    Ok(())


    // `file` goes out of scope, and the "hello.txt" file gets closed
}

pub fn hoge(ite : impl Iterator<Item = u32>){
    for hoge in ite{
        println!("{}", hoge);
    }
}