use serde_json::{Result, Value};
use json5;

pub fn untyped_example() -> Result<Value> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
{
  hogeNumber : 10,
  hogeString : "hoge",
  hogeBool : true,
  hogeObj : {
    hugaNumber : "30",

    //名前が?で終わるメンバはnull値を含むことが出来る。
    //プログラム側のメンバ名にはこの?は反映されず、型が変わるだけと想定される
    //初期値をnullに設定することは出来ない(undefinedの時にnullを入れる機能しかない）
    //これにより初期値で型を特定できる
    "hegoNumber?" : 21,

    //配列はいまのところnumber配列、string配列、number配列の配列の4通り。最初に型を示し、その後初期値をいれる。
    numArray : [ "Num-Array", 0, 3, 10 ],
    emptyNumArray : [ "Num-Array" ], //初期値が空配列のnum-array
    numArray2 : [ "Num-Array2", [2,3], [3,1] ], //二次元のnumarray

    "nullableNumArray?" : [ "Num-Array" ], //nullableにすることも出来るが、初期値をnullにする機能はない。undefinedのときにnullにすることは出来る。nullの使いみちってそれだけだと思う。
	strArray : [ "Str-Array", "hoge", "hogehoge" ], //文字列配列

	hogeList : [
		"List", //Listは配列とは違う。オブジェクトのコレクションを作るためにはlistを使う必要がある。
		["ListID", "hogehoge"], //任意でListIDを与えることが出来る。ListIDは全データの中で一意である必要がある。
		["Default", {
			hogeNumber : 0,
			hogeString : "hoge"
		}], //デフォルト値を設定。実際のリストには加わらない。
		{
			ID : "first",
			hogeNumber : 12,
			hogeString : "huga"
		},
		{
			ID : "second",
			//リストでは、デフォルト値から変更がない場合は書かなくても良いんじゃないかと思う
		}
	],

	"Rename" : [ "prevName->currentName",
	             "prevName2->currentName2" ], //メンバ名の変更をした場合、これを書いておくことで自動でメンバ名の対応表を作ってくれる。

	"member?" : 3,//nullableメンバにはundefinedの時にnullを入れられる。


	hogeList2 : [
	  "List",
	  ["Default", "second" ], //デフォルトのIDを指定。dummy指定とは違いこれは実際にリストに加わる。
	                          //DummyもDefaultも指定しない場合最初のものがデフォルトになる。
	  {
	    ID : "first",
	    member1 : "hoge"
	  },
	  {
	    ID : "second",
	    member2 : "this_is_default", //これがデフォルト値となり、差分が取られる。
	  }
	],
  },

  usables : [
    "List",
    ["ListID", "usable"],
    {
      ID : "yakusou",
      num : 3
    },
    {
      ID : "dokukesisou",
      num : 31
    }
  ],

  weapons : [
    "List",
    ["ListID", "weapon"],
    {
      ID : "katana",
      atk : 5
    },
    {
      ID : "doutanuki",
      atk : 7
    }
  ],

  itemList : [
    "List",
    ["AutoID"],
    ["RefListID", "weapon"],
    {
      RefID : "doutanuki", //どうたぬきを参照。参照すると継承される。
      atk : 8 //overrideしてみる
    }
  ],

  hogeList : [
    "List",
    ["ListID", "hoge"],
    {
      ID : "hogehoge",
      mem : "a",
    }
  ],

  hugaList: [
    "List",
    ["ListID", "huga"],
    {
      ID : "hugahuga"
    }
  ],

  hegoList: [
    "List",
    ["ListID", "hego"],
    {
      ID : "hegohego",
      mem : "b",
    }
  ],

  itemList3 : [
    "List",
    ["AutoID"],
    ["RefListIDs",  //多重継承が必要な場合、RefListIDsを設定する
      "hoge", "huga?", //nullableにも出来る
      "hego",
    ],
    {
      RefIDs : { //RefListIDsが設定されている場合、RefIDsが必要。必要なメンバを設定する。
        hoge : "hogehoge", //RefListIDと、RefIDをセットで記述していく。
        //nullableは入力しなければデフォルトでnull
        hego : "hegohego",
      }
    }
  ],
  "nullableObj?" : {
    member1 : 31,
  }
}

//使用側 概念コード
//let item = itemList3[0]; //item は hogehoge と hegohego を継承している
//item.mem <エラー。同じメンバを複数継承してしまった場合、曖昧なのでアクセスできない
//item.RefIDs.hoge.mem <これは"a"である
//item.RefIDs.hego.mem = "c" //hegoのmemをオーバーライドする。overrideなので、元のListにあるhegoの方に影響はない。
//item.RefIDs.hoge.mem = "d" //hogeの方もoverride
//let hegohego = list["hego"]["hegohego"];
//hegohego.mem <これは"b"のままである。
//hegohego.mem = "e" //eに変わる。overrideしていなければ、refしてるitemの方もこの値に変わる。今回の場合はoverrideしてるのでdのまま

"#;

    // Parse the string of data into serde_json::Value.
    match json5::from_str(data){
        Ok(v) =>{
            //println!("{}", serde_json::to_string_pretty(&v).unwrap());
            Ok(v)
        },
        Err(e) =>{ println!("{:?}", e); panic!(); },
    }

}