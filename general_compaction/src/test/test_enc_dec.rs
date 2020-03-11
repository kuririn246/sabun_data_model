use crate::kihon_enum::{Kihon, Decimal};
use crate::enc_dec::encode::encode;
use crate::enc_dec::decode::decode;


pub fn _test_enc_dec(){

    fn decimal(int : i128, dot : u8) -> Kihon{
        Kihon::Decimal(Decimal::new(int, dot))
    }
    let vec : Vec<Kihon> = vec![
        Kihon::Null,
        Kihon::Bit(false),
        Kihon::Bool(true),
        Kihon::Byte(0),
        Kihon::Byte(20),
        Kihon::Byte(-128),
        Kihon::Str16("amenbou".to_string()),
        Kihon::Str16("".to_string()),
        Kihon::Str16("012345678901234".to_string()),
        Kihon::Int(0),
        Kihon::Int(-50),
        Kihon::Int(200),
        Kihon::Int(-200),
        Kihon::Int(32767),
        Kihon::Int(-32767),
        Kihon::Int(-32768),
        Kihon::Int(32768),
        Kihon::Float(0.1f32),
        Kihon::Str256("01234567890123456".to_string()),
        Kihon::Str256("".to_string()),
        Kihon::Str256("01234".to_string()),
        Kihon::Double(0.1f64),
        decimal(65536*65536*65536, 4),
        decimal(-65536*65536*65536, 4),
        decimal(65536*65536*65536*65536, 0),
        decimal(-65536*65536*65536*65536, 0),
        decimal(65536*65536*65536*65536 / 2, 0),
        decimal(-65536*65536*65536*65536 / 2, 0),
        decimal(65536*65536*65536*65536 / 4, 0),
        decimal(-65536*65536*65536*65536 / 4, 0),
        decimal(-65536*65536*65536*65536*65536, 0),
        decimal(65536*65536*65536*65536*65536 / 2, 0),
        decimal(-65536*65536*65536*65536*65536 / 2, 0),
        decimal(65536*65536*65536*65536*65536 / 4, 0),
        decimal(-65536*65536*65536*65536*65536 / 4, 0),
        decimal(65536*65536*65536*65536*65536, 0),

        Kihon::BigStr(String::from_utf8(vec!['a' as u8; 256]).unwrap()),
        Kihon::BigStr("".to_string()),
        Kihon::Undefined(0),
        Kihon::Undefined(7),
    ];

    let encoded = encode(&vec);
    let decoded = decode(encoded);
    assert_eq!(vec, decoded);
}