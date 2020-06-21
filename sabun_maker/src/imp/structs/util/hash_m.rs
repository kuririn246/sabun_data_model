use std::hash::Hash;

///デフォルトのHasherはランダムなので、毎回出てくる順序が違い直観的じゃないように思う。それに効率も少し悪い。
/// このシステムは外部の人間がHashtableのKeyを決めることが出来るようになっていないのでDos耐性はいらない
/// 固定のHasherに変える
pub type HashM<K,V> = fnv::FnvHashMap<K,V>;

pub type HashS<K> = fnv::FnvHashSet<K>;

pub(crate) struct HashMt{}
impl HashMt{
    pub fn new<K: Hash+Eq,V>() -> HashM<K,V>{ HashM::default() }
    pub fn with_capacity<K: Hash+Eq,V>(capacity : usize) -> HashM<K,V>{ HashM::with_capacity_and_hasher(capacity, Default::default()) }
}

pub(crate) struct HashSt{}
impl HashSt{
    pub fn new<K: Hash+Eq>() -> HashS<K>{ HashS::default() }
    pub fn with_capacity<K: Hash+Eq>(capacity : usize) -> HashS<K>{ HashS::with_capacity_and_hasher(capacity, Default::default()) }
}