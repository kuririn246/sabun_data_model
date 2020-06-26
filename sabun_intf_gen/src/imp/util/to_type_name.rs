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

pub fn to_item_name(s : &str) -> String{
    let mut r = String::with_capacity(s.len());
    for (i,c) in s.chars().enumerate(){
        if i == 0{
            r.push(c.to_ascii_lowercase());
        } else{
            r.push(c)
        }
    }
    r
}