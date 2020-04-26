// use crate::structs::rust_object::RustObject;
// use crate::imp::version_adjuster::adjust_ref::adjust_ref;
// use crate::error::Result;

// pub fn adjust(new_def : &RustObject, new : RustObject, old_def : RustObject, old : RustObject) -> Result<()>{
//     let mut new = new;
//     if let Some(refs) = &new_def.refs {
//         new.refs = adjust_ref(&new.renamed, refs, new.refs, &old_def.refs, old.refs)?;
//     }
//     //new.
//     return Ok(());
// }