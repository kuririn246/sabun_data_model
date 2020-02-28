use crate::error::Result;
use std::collections::BTreeMap;
use crate::jval::JVal;
use crate::de::{deserialize_any, Seq, Map};
use pest::Span;
use crate::de::Rule;

pub fn get_unit(span : Span) -> JVal { JVal::Null(s(span)) }

pub fn get_bool(b: bool, span : Span) -> JVal { JVal::Bool(b, s(span)) }

pub fn get_string(st: String, span : Span) -> JVal { JVal::String(st, s(span)) }

pub fn get_i64(i: i64, span : Span) -> JVal { JVal::Int(i, s(span)) }

pub fn get_f64(f: f64, span : Span) -> JVal { JVal::Double(f, s(span)) }

pub fn get_seq(seq: Seq, span : Span) -> Result<JVal> {
    let mut result : Vec<JVal> = vec![];
    let pairs = seq.pairs;
    for pair in pairs{
        result.push(deserialize_any(pair)?);
    }
    return Ok(JVal::Array(result, s(span)));
}

pub fn get_map(m: Map, span : Span) -> Result<JVal> {
    let mut result : BTreeMap<String, JVal> = BTreeMap::new();
    let mut pairs = m.pairs;
    loop{
        let op = pairs.next();
        if op.is_none(){ break; }
        let p = op.unwrap();
        let ident = match p.as_rule(){
            Rule::identifier =>{ p.as_str().to_string() },
            Rule::string =>{ p.as_str().to_string() },
            _ =>{
                //println!("{:?}",p.as_rule());
                //println!("{:?}",p.as_str());

                unreachable!()
            }
        };
        let p   = pairs.next().unwrap();
        let val = deserialize_any(p)?;
        result.insert(ident, val);
    }

    Ok(JVal::Map(result, s(span)))
}

fn s(span : Span) -> crate::jval::Span{
    crate::jval::Span{ start : span.start(), end : span.end() }
}


