use crate::kihon_enum::Kihon;
use crate::encode::encode;
use crate::decode::decode;


pub fn _test_enc_dec(){
    let vec : Vec<Kihon> = vec![
        Kihon::Bit(false),
        Kihon::Byte(20),
        Kihon::Byte(-128),
        Kihon::Str16("amenbou".to_string()),
        Kihon::Str16("".to_string()),
        Kihon::Str16("012345678901234".to_string()),
        Kihon::Int(200),
        Kihon::Int(-200),
        Kihon::Int(32767),
        Kihon::Int(-32767),
        Kihon::Int(-32768),
        Kihon::Int(32768),
        Kihon::Float(0.1f32),
        Kihon::Str256("01234567890123456".to_string()),
        Kihon::Str256("".to_string()),
        Kihon::Double(0.1f64),
        Kihon::Decimal(65536*65536*65536, 4),
        Kihon::Decimal(-65536*65536*65536, 4),
        Kihon::Decimal(65536*65536*65536*65536, 0),
        Kihon::Decimal(-65536*65536*65536*65536, 0),
        Kihon::Decimal(65536*65536*65536*65536 / 2, 0),
        Kihon::Decimal(-65536*65536*65536*65536 / 2, 0),
        Kihon::Decimal(65536*65536*65536*65536 / 4, 0),
        Kihon::Decimal(-65536*65536*65536*65536 / 4, 0),
        Kihon::Decimal(65536*65536*65536*65536*65536, 0),
        Kihon::Decimal(-65536*65536*65536*65536*65536, 0),
        Kihon::Decimal(65536*65536*65536*65536*65536 / 2, 0),
        Kihon::Decimal(-65536*65536*65536*65536*65536 / 2, 0),
        Kihon::Decimal(65536*65536*65536*65536*65536 / 4, 0),
        Kihon::Decimal(-65536*65536*65536*65536*65536 / 4, 0),

        Kihon::BigStr(String::from_utf8(vec!['a' as u8; 256]).unwrap()),
        Kihon::BigStr("".to_string()),
    ];

    let encoded = encode(&vec);
    let decoded = decode(encoded);
    assert_eq!(vec, decoded);
}