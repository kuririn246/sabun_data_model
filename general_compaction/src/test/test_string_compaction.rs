pub fn _test_string_compaction(){
    let mut v : Vec<&str> = vec![
        "1",
        "0",
        "2",
        "0.1",
        "0.001",
        "127",
        "-128",
        "128",
        "-129",
        "32767",
        "32768",
        "-32767",
        "-32768",
        "-9223372036854775808",
        "9223372036854775807",
        "-9223372036854775809",
        "9223372036854775808",
        "18446744073709551615",
        "18446744073709551616",
        "-18446744073709551615",
        "-18446744073709551616",
        "",
        "a",
        "abcdeabcdeabcde",
        "abcdeabcdeabcdef",
        "0.001",
        "200.01",
        "-9999.01",
        "0.",
        "-0.",
        "-00.",
        "-00.0",
        "0.0000",
        "-0.000",
    ];
    let s1 = String::from_utf8(vec!['a' as u8; 255]).unwrap();
    let s2 = String::from_utf8(vec!['a' as u8; 256]).unwrap();
    v.push(&s1);
    v.push(&s2);

    let _vx : Vec<&str> = vec![
        "-0.000"
    ];



    let vv : Vec<String> = v.iter().map(|s| s.to_string()).collect();
    let _vv2 = vv.to_vec();

    let kihons = crate::string_compaction::to_kihons(vv);
    let encoded = crate::enc_dec::encode::encode(&kihons);
    let url = crate::url_string::get_url_string(&encoded);
    //println!("{}", url);
    //println!("{}", url.len());
    let bytes = crate::url_string::get_bytes(&url).unwrap();
    let decoded = crate::string_compaction::decompress(bytes);

    assert_eq!(decoded, v);

//    for k in &decoded {
//        println!("{:?}", k);
//    }
}