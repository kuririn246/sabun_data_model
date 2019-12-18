use std::cmp::{max, min};

struct StartFrom{
    index : usize,
    vec : Vec<i32>
}

impl StartFrom{
    fn size(&self) -> usize{ self.vec.len() }
    fn last_index(&self) -> usize{
        return self.index + self.vec.len();
    }
}

#[derive(Debug)]
enum Sabun{
    Initial, A,B,C
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
    initial : Vec<i32>,
    a_sabuns : Vec<ASabun>,
    last_sabun : Sabun,
}

impl Sabuns{
    fn get_a_mut(&mut self) -> &mut ASabun{
        self.a_sabuns.last_mut().unwrap()
    }

    fn get_b_mut(&mut self) -> &mut BSabun{
        self.get_a_mut().b_sabuns.last_mut().unwrap()
    }

    fn get_c_mut(&mut self) -> &mut CSabuns{
        self.get_b_mut().c_sabuns.last_mut().unwrap()
    }

    fn get_a(&self) -> &ASabun{
        self.a_sabuns.last().unwrap()
    }

    fn get_b(&self) -> &BSabun{
        self.get_a().b_sabuns.last().unwrap()
    }

    fn get_c(&self) -> &CSabuns{
        self.get_b().c_sabuns.last().unwrap()
    }
}

fn apply(vec : &mut Vec<i32>, sabun : &StartFrom){
    for n in sabun.index..sabun.last_index(){
        if n < vec.len(){
            vec[n] = sabun.vec[n - sabun.index];
        }
        else{
            vec.push(sabun.vec[n - sabun.index]);
        }
    }
}


fn construct(last_sabun : &Sabun, sabuns : &Sabuns) -> Vec<i32>{
    let last_sabun = match last_sabun{
        Sabun::Initial => 0,
        Sabun::A => 1,
        Sabun::B => 2,
        Sabun::C => 3,
    };

    let mut vec = sabuns.initial.clone();
    if last_sabun >= 1 {
        let a = sabuns.get_a();
        apply(&mut vec, &a.sabun);

        if last_sabun >= 2 {
            let b = a.b_sabuns.last().unwrap();
            apply(&mut vec, &b.sabun);

            if last_sabun >= 3{
                let c = b.c_sabuns.last().unwrap();
                for item in &c.vec{
                    apply(&mut vec, item);
                }
            }
        }
    }
    return vec;
}

fn make_sabun(current : &Vec<i32>, prev : &Vec<i32>) -> StartFrom {
    for i in 0..current.len() {
        if i < prev.len() {
            if current[i] != prev[i] {
                return StartFrom { vec: (&current[i..]).to_vec(), index: i }
            }
        } else { break; }
    }
    //データは絶対に増え続けることにしてるのでこれでいい
    let i = prev.len();
    return StartFrom { vec: (&current[i..]).to_vec(), index: i };
}

fn c_is_full(sabuns : &Sabuns) -> bool{
    let a_size = sabuns.get_a().sabun.size();
    let b_size = sabuns.get_b().sabun.size();
    let c_size = sabuns.get_c().vec[0].size();
    let cp_sum : usize = sabuns.get_c().vec[1..].iter().map(|s| s.size()).sum();
    return a_size + b_size + c_size < cp_sum;
    //return 20 < cp_sum;
}

fn calc_common_size(a : &StartFrom, b : &StartFrom) -> usize{
    let start = max(a.index, b.index);
    let end = min(a.index + a.vec.len(), b.index + b.vec.len());
    for n in start..end{
        let a_ind = n - a.index;
        let b_ind = n - b.index;
        if a.vec[a_ind] != b.vec[b_ind]{
            return n - start;
        }
    }
    return 0;
}

fn should_update_b(current : &Vec<i32>, sabuns : &Sabuns) -> bool{
    //should_update_b_easy(current, sabuns)
    should_update_b_detailed(current, sabuns)
    //should_update_b_super_easy(current, sabuns)
    //true
}

fn should_update_b_easy(current : &Vec<i32>, sabuns : &Sabuns) -> bool {
    let prev_cb_sabun = &sabuns.get_c().vec[0];

    let b_data = construct(&Sabun::B, sabuns);
    let new_cb_sabun = &make_sabun(current, &b_data);

    let b = sabuns.get_b().sabun.size();
    let cb = new_cb_sabun.size();
    let answer = b * 2 < cb * 3;
    if answer {
        println!("a {} b {} cb {}", sabuns.get_a().sabun.size(), b, cb);
    }
    return answer;
}


fn should_update_b_super_easy(current : &Vec<i32>, sabuns : &Sabuns) -> bool {
    let prev_cb_sabun = &sabuns.get_c().vec[0];

    let b_data = construct(&Sabun::B, sabuns);
    let new_cb_sabun = &make_sabun(current, &b_data);

    sabuns.get_b().sabun.size() < new_cb_sabun.size()
}

fn should_update_b_detailed(current : &Vec<i32>, sabuns : &Sabuns) -> bool {
    let prev_cb_sabun = &sabuns.get_c().vec[0];

    let b_data = construct(&Sabun::B, sabuns);
    let new_cb_sabun = &make_sabun(current, &b_data);

    let common_size = calc_common_size(prev_cb_sabun, new_cb_sabun);

//new_cb_sabunのうちprev_cb_sabunとの共通部分がどれだけあるか調べる
//その共通部分はBをアップデートすればそこに取り込まれ続くファイルのサイズを減らすことができるので、
//それを引いてアップデート後の新サイズを推測する
    let guessed_cb_size = new_cb_sabun.size() - common_size;

    let b_sabun = sabuns.get_b();

    let n = b_sabun.c_sabuns.len() as i64;

    let b = b_sabun.sabun.size() as i64;
    let cb = new_cb_sabun.size() as i64;
    let guessed_cb = guessed_cb_size as i64;

    let answer = 4 * b - 2 * (cb + guessed_cb) < n * (cb - guessed_cb);

    if answer{
        println!("a {} b {} cb {} guess {} n {} common {}", sabuns.get_a().sabun.size(), b, cb, guessed_cb, n, common_size);
    }
    return answer;
//4A - 2(B' + B) < n(B' - B)
}

fn should_update_a(current : &Vec<i32>, sabuns : &Sabuns) -> bool{

    let prev_b_sabun = &sabuns.get_b().sabun;

    let a_data = construct(&Sabun::A, sabuns);
    let new_b_sabun = &make_sabun(current, &a_data);

    let common_size = calc_common_size(prev_b_sabun, new_b_sabun);

    //new_cb_sabunのうちprev_cb_sabunとの共通部分がどれだけあるか調べる
    //その共通部分はBをアップデートすればそこに取り込まれ続くファイルのサイズを減らすことができるので、
    //それを引いてアップデート後の新サイズを推測する
    let guessed_b_size = new_b_sabun.size() - common_size;

    let a_sabun = sabuns.get_a();

    let n = a_sabun.b_sabuns.len() as i64;

    let a = a_sabun.sabun.size() as i64;
    let b = new_b_sabun.size() as i64;
    let guessed_b = guessed_b_size as i64;

    4 * a - 2 * (b + guessed_b) < n * (b - guessed_b)
    //true
    //4A - 2(B' + B) < n(B' - B)
}

fn try_push_c_sabun(sabuns : &mut Sabuns, current : &Vec<i32>){
    if !c_is_full(sabuns){
        let prev = construct(&Sabun::C, sabuns);
        let last_sabun = make_sabun(current, &prev);
        sabuns.get_c_mut().vec.push(last_sabun);
        return;
    }
    else{
       if should_update_b(&current, sabuns){
            if should_update_a(&current, sabuns){
                let sabun = make_sabun(current, &sabuns.initial);
                sabuns.a_sabuns.push(ASabun{ sabun, b_sabuns : vec![] });
                sabuns.last_sabun = Sabun::A;
                return;
            }
           else{
               let prev = construct(&Sabun::A, sabuns);
               let sabun = make_sabun(current, &prev);
               sabuns.get_a_mut().b_sabuns.push(BSabun{ sabun, c_sabuns : vec![] });
               sabuns.last_sabun = Sabun::B;
               return;
           }
       }
       else{
           let prev = construct(&Sabun::B, sabuns);
           let sabun = make_sabun(current, &prev);
           sabuns.get_b_mut().c_sabuns.push(CSabuns{ vec: vec![ sabun ]});
           sabuns.last_sabun = Sabun::C;
           return;
       }
    }
}

fn calc(sabuns : &mut Sabuns, current : &Vec<i32>){
    let prev = construct(&sabuns.last_sabun, sabuns);

    let sabun = make_sabun(current, &prev);
    match &sabuns.last_sabun{
        &Sabun::Initial =>{
            sabuns.a_sabuns.push(ASabun{ sabun, b_sabuns : vec![] });
            sabuns.last_sabun = Sabun::A;
        },
        &Sabun::A =>{
            let a = sabuns.get_a_mut();
            a.b_sabuns.push(BSabun{ sabun, c_sabuns : vec![] });
            sabuns.last_sabun = Sabun::B;
        }
        &Sabun::B =>{
            let b = sabuns.get_b_mut();
            b.c_sabuns.push(CSabuns{ vec: vec![ sabun ]});
            sabuns.last_sabun = Sabun::C;
        }
        &Sabun::C =>{
            try_push_c_sabun(sabuns, current);
        }
    }
}

fn print_sabun(sabuns : &Sabuns){
    let mut sum : usize = 0;
    let mut a_sum : usize = 0;
    let mut b_sum : usize = 0;
    let mut c_sum : usize = 0;
    for ia in 0..sabuns.a_sabuns.len(){
        let a = &sabuns.a_sabuns[ia];
        let asize = a.sabun.size();
        sum += asize;
        a_sum += asize;
        println!("A{} size {} {} a_sum {}",ia, asize, sum, a_sum);
        for ib in 0..a.b_sabuns.len(){
            let b = &a.b_sabuns[ib];
            let bsize = b.sabun.size();
            sum += bsize;
            b_sum += bsize;
            println!("B{}-{} size {} {} b_sum {}", ia, ib, bsize, sum, b_sum);
            for ic in 0..b.c_sabuns.len(){
                let c = &b.c_sabuns[ic];
                let total : usize = c.vec.iter().map(|s| s.size()).sum();
                sum += total;
                c_sum += total;
                let vec : Vec<String> = c.vec.iter().map(|a| a.size().to_string()).collect();
                println!("C{}-{}-{} {} c_sum {} size {:?}", ia, ib, ic, sum, c_sum, vec.join(", "));
            }
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
        last_sabun : Sabun::Initial,
    };

    for n in 1..2000{
        current.push(n);
        let start = current.len() - rewrite;
        for i in start..current.len(){
            current[i] = n;
        }
        calc(&mut sabuns, &current)
    }

    print_sabun(&sabuns);
}
