#[derive(Debug)]
pub enum ListType{
    ///稼働中のシステムでは、次に割り振るべきIDが入っている。IDを割り振った後インクリメントしていく。
    AutoID(Option<u64>),
    Reffered,
    Normal
}