

#[cfg(test)]
mod testion {
    use crate::{json_dir_to_rust, rust_to_json_new_default};
    use crate::imp::version_adjuster::version_adjuster::adjust_versions;
    use crate::error::Result;
    use crate::imp::json_to_rust::json_root_to_rust;

    #[test]
    pub fn test_version_adjuster() -> Result<()> {
        let new = match json_dir_to_rust("src/json_dir/version_adjuster/new", true) {
            Ok(j) => j,
            Err(e) => {
                println!("new {}", e.message);
                return Ok(());
            }
        };

        let old = match json_dir_to_rust("src/json_dir/version_adjuster/old", true) {
            Ok(j) =>{
                match rust_to_json_new_default(&j){
                    Ok(b) =>{
                        //println!("old pretty {}", b.to_string_pretty());
                    },
                    Err(e) =>{ println!("old nopretty {}", e.message); }
                }
                j
            },
            Err(e) => {
                println!("old {}", e.message);
                return Ok(());
            }
        };

        match adjust_versions(new, old, true) {
            Ok(a) => {
                match rust_to_json_new_default(&a) {
                    Ok(b) => {
                        println!("pretty {}", b.to_string_pretty());
                        match json_root_to_rust(&b.to_string_pretty()){
                            Ok(a) =>{
                                match rust_to_json_new_default(&a){
                                    Ok(json2) =>{
                                        assert_eq!(b.to_string_pretty(), json2.to_string_pretty());
                                        println!("OK");
                                    },
                                    Err(e) =>{
                                        println!("ERR 2 {}", e.message);
                                    }
                                }

                            }
                            Err(e) =>{ println!("ERR {}", e.message)}
                        }
                    },
                    Err(e) => {
                        println!("rust to json {}", e.message);
                    }
                }
            }
            Err(e) => { println!("adjust versions {}", e.message); }
        }
        return Ok(());
    }
}