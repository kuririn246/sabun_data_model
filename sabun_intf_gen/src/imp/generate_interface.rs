use sabun_maker::intf::*;
use sabun_maker::structs::RootObject;

use crate::imp::create_struct_descs::create_struct_desc_root;
use crate::imp::structs::struct_desc::StructDesc;
use crate::imp::structs::sources::{SourceTree, StructSource, Sources};
use crate::imp::to_source_from_col_temp::to_source_from_col_temp;
use crate::imp::to_struct_temp_from_struct_desc::to_struct_temp_from_struct_desc;
use crate::imp::to_source_from_struct_temp::to_source_from_struct_temp;

/// データに静的にアクセスできるラッパーを生成する。
/// RootIntf::newし、intf()を起点にしてアクセスする。
/// 全データはget時にキャッシュされるので、二度目以降はハッシュテーブルアクセスは行われない。
/// キャッシュされるので、getでもmodifyが必要となり、&mutを要求する。
/// キャッシュ値を入れるだけなので、マルチスレッドでgetアクセスしても、キャッシュが二度作られることはあるだろうが、別に安全だと思う。
/// もちろんgetとsetを同時に行えば不整合が起きることもあるだろう。マルチスレッドでの動作は安全ではない。
/// キャッシュされているので、ポインタは一定値をとる。なので&mut参照を*mut(or *const)ポインタとして保持しておいても良い。
/// もちろんMutListのアイテムへのポインタを持ちながら、アイテムを削除すればそのポインタは不正になる。
/// RootObjectを削除してしまえば、RootItem以下の全部が不正になる
pub fn generate_interface(root : &RootObject) -> Sources{
    let mem_descs = member_desc::get_member_desc(root);
    let desc = create_struct_desc_root(&mem_descs);

    let st = to_struct_temp_from_struct_desc(&desc);
    let root= to_source_from_struct_temp(&st);

    let mut vec : Vec<StructSource> = vec![];
    for child in &desc.children{
        let tree = generate_source_tree(child);
        flatten(&mut vec, tree);
    }

    let usings = "\
use sabun_maker::intf::*;
use sabun_maker::structs::*;

pub struct RootIntf{
    obj : Box<RootObject>,
    intf : RootItem,
}
impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{
        let mut b = Box::new(obj);
        let intf = RootItem::new(RootObjectPtr::new(b.as_mut()));
        RootIntf{ obj : b, intf }
    }
    pub fn intf(&mut self) -> &mut RootItem{ &mut self.intf }
    pub fn deconstruct(self) -> (Box<RootObject>, RootItem){ (self.obj, self.intf) }
}
".to_string();
    Sources::new(usings, root.source().to_string(), vec)
}

fn generate_source_tree(desc : &StructDesc) -> SourceTree{
    let struct_temp = to_struct_temp_from_struct_desc(desc);

    let col_source = to_source_from_col_temp(&desc);
    let item_source = to_source_from_struct_temp(&struct_temp);

    let mut vec : Vec<SourceTree> = vec![];
    for child in &desc.children{
        vec.push(generate_source_tree(child));
    }

    SourceTree{
        item_source,
        col_source,
        children : vec
    }
}

fn flatten(vec : &mut Vec<StructSource>, tree : SourceTree){
    vec.push(tree.col_source);
    vec.push(tree.item_source);
    for child in tree.children{
        flatten(vec, child);
    }
}