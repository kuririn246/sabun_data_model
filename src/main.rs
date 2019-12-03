use std::iter::repeat;
use std::cmp::{max, min};

struct StartFrom{
    index : usize,
    vec : Vec<i32>
}

impl StartFrom{
    fn size(&self) -> usize{ self.vec.len() }
}

struct ASabun{
    sabun : StartFrom,
    b_sabuns : Vec<BSabun>
}

struct BSabun{
    sabun : StartFrom,
    c_sabuns : Vec<CSabuns>,
}

struct CSabuns{
    vec : Vec<StartFrom>
}

struct Sabuns{
    start_index : usize,
    initial : Vec<i32>,
    a_sabuns : Vec<ASabun>,
    last_sabun : String,
}

fn last(vec : &Vec<StartFrom>) -> &StartFrom{
    return vec.last().unwrap();
}

fn last_index(sabun : &StartFrom) -> usize{
    return sabun.index + sabun.vec.len();
}

fn apply(vec : &mut Vec<i32>, sabun : &StartFrom){
    for n in sabun.index..last_index(sabun){
        if n < vec.len(){
            vec[n] = sabun.vec[n - sabun.index];
        }
        else{
            vec.push(0);
            vec[n] = sabun.vec[n - sabun.index];
        }
    }
}

fn apply_c(vec : &mut Vec<i32>, sabuns : &Vec<StartFrom>){
    for sabun in sabuns{
        apply(vec, sabun);
    }
}

fn construct(last_sabun : &str, sabuns : &Sabuns) -> Vec<i32>{
    let last_sabun = match last_sabun{
        "i" => 0,
        "a" => 1,
        "b" => 2,
        "cb" => 3,
        "cp" => 4,
        _ => -1,
    };

    let mut vec = sabuns.initial.clone();
    if last_sabun <= 1{
        apply(&mut vec,last(&sabuns.a_sabuns));
    }
    if last_sabun <= 2{
        apply(&mut vec, last(&sabuns.b_sabuns));
    }
    let c = sabuns.c_sabuns.last().unwrap();
    if last_sabun <= 3{
        apply(&mut vec, c[0]);
    }
    if last_sabun <= 4{
        for n in c.vec[1..]{
           apply(&mut vec, n);
        }
    }
    return vec;
}

fn make_sabun(current : &Vec<i32>, prev : &Vec<i32>) -> StartFrom {
    for i in 0..current.len() {
        if i < prev.len() {
            if current[i] != prev[i] {
                return StartFrom { vec: (&current[n..]).to_vec(), index: i }
            }
        } else { break; }
    }
    //データは絶対に増え続けることにしてるのでこれでいい
    let i = prev.len();
    return StartFrom { vec: (&current[i..]).to_vec(), index: i };
}

fn c_is_full(sabuns : &Sabuns) -> bool{
    return sabuns.c_sabuns.last().unwrap().vec.len() > 3;
}

fn calc_common_size(a : &StartFrom, b : &StartFrom) -> usize{
    let start = max(a.index, b.index);
    let end = min(a.index + a.vec.len(), b.index + b.vec.len());
    for n in start..end{
        a_ind = n - a.index;
        b_ind = n - b.index;
        if a.vec[a_ind] != b.vec[b_ind]{
            return n - start;
        }
    }
    return 0;
}

fn should_update_b(current : &Vec<i32>, sabuns : &Sabuns) -> bool{

    let prev_cb_sabun = &sabuns.c_sabuns.last().unwrap().vec[0];

    let b_data = construct("b", sabuns);
    let new_cb_sabun = &make_sabun(current, &b_data);

    let common_size = calc_common_size(prev_cb_sabun, new_cb_sabun);

    ///new_cb_sabunのうちprev_cb_sabunとの共通部分がどれだけあるか調べる
    ///その共通部分はBをアップデートすればそこに取り込まれ続くファイルのサイズを減らすことができるので、
    ///それを引いてアップデート後の新サイズを推測する
    let guessed_cb_size = new_cb_sabun.size() as i32 - common_size as i32;

    let b_sabun = sabuns.b_sabuns.last().unwrap();

    4 * b_sabun.size() - 2 * (new_cb_sabun.size() + guessed_cb_size) <
    //4A - 2(B' + B) < n(B' - B)
}

fn try_push_c_sabun(sabuns : &mut Sabuns, current : Vec<i32>){
    if !c_is_full(sabuns){
        let prev = construct("cp", sabuns);
        let last_sabun = make_sabun(&current, &prev);
        sabuns.c_sabuns.last_mut().unwrap().vec.push(last_sabun);
    }
    else{
        let prev_cb = construct("cb", sabuns);
        let prev_b = construct("b", sabuns);
        let current_cb = make_sabun(&current, &prev_b);

    }
}

fn calc(sabuns : &mut Sabuns, current : &Vec<i32>){
    let prev = construct(&sabuns.last_sabun, current);

    let sabun = make_sabun(current, &prev);
    let last_sabun = sabuns.last_sabun.clone();
    match &last_sabun{
        "i" =>{
            sabuns.a_sabuns.push(sabun);
            sabuns.last_sabun = "a".to_string();
        },
        "a" =>{
            sabuns.b_sabuns.push(sabun);
            sabuns.last_sabun = "b".to_string();
        }
        "b" =>{
            sabuns.c_sabuns.push(vec![sabun]);
            sabuns.last_sabun = "cb".to_string();
        }
        "cb" =>{

        }
    }
}

fn main() {
    let mut current = vec![0; 100];

    //データが一個ずつふえ、rewrite個のデータがrewriteされる。
    //rewriteする数は一定で、かならず配列の最後の部分だけrewriteする。
    //なので変化しない部分も一つずつ増えていく。
    let increase = 1;
    let rewrite = 10;

    let mut sabuns = Sabuns{
        initial : current.clone(),
        a_sabuns : vec![],
        b_sabuns : vec![],
        c_sabuns : vec![],
        start_index : current.len() - rewrite + increase,
        last_sabun : "i".to_string(),
    };

    for n in 1..100{
        current.push(n);
        let start = current.len() - rewrite;
        for i in start..current.len(){
            current[i] = n;
        }




    }






}
