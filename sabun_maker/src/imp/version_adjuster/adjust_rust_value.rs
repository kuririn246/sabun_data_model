use crate::structs::rust_value::RustValue;
use crate::imp::version_adjuster::adjust_list::adjust_list;

///r の qv が l の value_typeに対して代入可能ならば、代入したRustValueを返す
pub fn adjust_rust_value(l : &RustValue, r : RustValue) -> Option<RustValue>{
    match l{
        RustValue::Bool(b, t) => match r{
            RustValue::Bool(rb, _t) =>
                if t.acceptable(&rb.qv_type()) {
                    Some(RustValue::Bool(rb, t.clone()))
                } else{ None },
            _ => None,
        },
        RustValue::Number(n, t) => match r{
            RustValue::Number(rn, _t) =>
                if t.acceptable(&rn.qv_type()) {
                    Some(RustValue::Number(rn, t.clone()))
                } else{ None },
            _ => None,
        },
        RustValue::String(s, t) => match r{
            RustValue::String(rs, _t) =>
                if t.acceptable(&rs.qv_type()) {
                    Some(RustValue::String(rs, t.clone()))
                } else{ None },
            _ => None,
        },
        RustValue::Array(a, at, t) => match r {
            RustValue::Array(ra, rat, _t) => {
                if t.acceptable(&ra.qv_type()) && at.type_num() == rat.type_num() {
                    return Some(RustValue::Array(ra, rat, t.clone()))
                } else { None }
            },
            _ => None,
        },
        RustValue::List(l) => match r{
            RustValue::List(r) => Some(adjust_list())
        },
        //RustValue::Object(_) => r,
    }
}