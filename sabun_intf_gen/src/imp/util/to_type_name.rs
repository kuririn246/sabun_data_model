///1文字目を大文字にしたいだけなんだけどやたらと大変やね・・・
pub fn to_type_name(s : &str) -> String{
    let mut r = String::with_capacity(s.len());
    for (i,c) in s.chars().enumerate(){
        if i == 0{
            r.push(c.to_ascii_uppercase());
        } else{
            r.push(c)
        }
    }
    r
}

///小文字にしてsnake_caseにする
pub fn to_item_name(s : &str) -> String{
    let mut r = String::new();
    for (i,c) in s.chars().enumerate(){
        if i == 0{
            r.push(c.to_ascii_lowercase());
        } else{
            if c.is_ascii_uppercase(){
                r.push('_');
                r.push(c.to_ascii_lowercase());
            } else {
                r.push(c)
            }
        }
    }
    r
}

pub fn to_item_type_name(mem : &str) -> String{
    format!("{}Item", to_type_name(mem))
}