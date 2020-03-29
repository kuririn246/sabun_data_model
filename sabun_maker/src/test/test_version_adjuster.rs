use crate::json_dir_to_rust;
use crate::imp::version_adjuster::version_adjuster::adjust;
use crate::error::Result;

#[cfg(test)]
pub fn test_version_adjuster() -> Result<()>{
    let new = json_dir_to_rust("src/json_dir/version_adjuster/new", true)?;

    let old = json_dir_to_rust("src/json_dir/version_adjuster/new", true)?;

    adjust(new, old);
    return Ok(());
}