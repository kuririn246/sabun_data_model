#[derive(Debug, PartialEq)]
pub enum Kihon{
    Null,
    Bit(bool),
    Bool(bool),
    Byte(i8),
    Str16(String),
    ///Intのバイト数は2-8である必要がある。1の場合はByteを使わねばならず、処理しようとすると勝手にByteに変わっているので注意
    Int(i64),
    ///Floatはバイナリ上ではIntの1Byteの場合になるので、Intに1ByteがあるとFloatと区別できない
    Float(f32),
    Str256(String),
    Double(f64),
    ///u8はドットの位置を表す。最後が.である表現は認めず、0はドットがないことを表す。
    Decimal(i128, u8),
    BigStr(String),
    Undefined(u8),
}

#[derive(Debug, PartialEq)]
pub enum KihonFromTag{
    Null,
    Bit(bool),
    Bool(bool),
    Byte,
    Str16(u8),
    Int(u8),
    Float,
    Str256,
    Double,
    Decimal(u8),
    BigStr(u8),
    Undefined(u8),
}