///1文字目を大文字にしたいだけなんだけどやたらと大変やね・・・
pub fn to_type_name(s : &str) -> String{
    let mut r = String::with_capacity(s.len());
    for (i,c) in s.chars().enumerate(){
        if i == 0{
            if c == '@'{
                r.push_str("At");
            } else {
                r.push(c.to_ascii_uppercase());
            }
        } else{
            r.push(c)
        }
    }
    r
}

///小文字にしてsnake_caseにする
pub fn to_snake_name(s : &str) -> String{
    let mut r = String::new();
    for (i,c) in s.chars().enumerate(){
        if i == 0{
            if c == '@'{
                r.push_str("at");
            } else {
                r.push(c.to_ascii_lowercase());
            }
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

pub fn to_const_item_type_name(stem : &str) -> String{
    format!("{}CItem", to_type_name(stem))
}
pub fn to_mut_item_type_name(stem : &str) -> String{
    format!("{}MItem", to_type_name(stem))
}

pub fn to_table_type_name(stem : &str) -> String{
    format!("{}Table", to_type_name(stem))
}

pub fn to_ids_type_name(stem : &str) -> String{
    format!("{}IDs", to_type_name(stem))
}

pub fn to_const_list_type_name(stem : &str) -> String{
    format!("{}CList", to_type_name(stem))
}
pub fn to_mut_list_type_name(stem : &str) -> String{
    format!("{}MList", to_type_name(stem))
}
pub fn to_const_vec_name(stem : &str) -> String{ format!("{}CVec", to_type_name(stem)) }
pub fn to_mut_vec_type_name(stem : &str) -> String{ format!("{}MVec", to_type_name(stem)) }