use serde_json::{Result, Value};
use json5;

#[allow(dead_code)]
pub fn untyped_example() -> Result<Value> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
{
  hogeNumber : 10,
  hogeString : "hoge",
  hogeBool : true,
  hogeObj : {
    hugaNumber : 30,
    "@HugaNumber" : 30, //先頭が大文字のメンバ名はすべてシステムが予約しているので認められない。しかしメンバ名をどうしても大文字で始めたい場合、
                        //jsonでは戦闘に@をつけ、プログラム側では@なしでもアクセス可能にするという技も使えなくはなかろうと思う。

    //名前が?で終わるメンバはnull値を含むことが出来る。
    //プログラム側のメンバ名にはこの?は反映されず、型が変わるだけ(もし俺以外の実装が現れたらわからないが・・・）
    //初期値にnullを入れるには、特殊な記法を使う必要がある
    "hegoNumber?" : ["Num", null], //型を指定し、null値を入れる。これでnullになる。

    //!で終わる場合、バージョン違いでこのメンバを持っていなかった場合、デフォルト値でなくundefinedが入る。
    //undefinedを代入する手段はない。
    "pugyaNumber!" : 10,
    "pugyaNumber2!?" : ["Num", null], //!?も出来る。?!でも良い。

    //配列はいまのところnumber配列、string配列、number配列の配列の4通り。
    numArray2 : [ "Num-Array", 0, 3, 10 ],
    numArray3 : [ "Num-Array", [0, 3, 10] ],

    emptyNumArray : [ "Num-Array", [] ], //初期値が空配列のnum-array
    "nullableNumArray?" : [ "Num-Array", null ], //nullable配列の初期値をnullにする場合
    numArray2 : [ "Num-Array2", [2,3], [3,1] ], //二次元のnumarray
    numArray2 : [ "Num-Array2", [[2,3], [3,1]] ], //空配列やnullとの一貫性のためにこの書き方も認める


	strArray : [ "Str-Array", "hoge", "hogehoge" ], //文字列配列
	strArray2 : [ "Str-Array", ["ababa", "ibibi"] ],
	//そもそも配列なんてこのシステムに必要なんだろうか・・・？　まともに差分生成出来る気もしないしなあ。

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

	hogeList2 : [
	  "List",
	  ["Default", "second" ], //デフォルトのIDを指定。Defaultを指定しない場合最初のものがデフォルトになる。
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
    ["RefListIDs",  //複数の参照が必要な場合、RefListIDsを設定する
      "hoge", "huga?", //nullableにも出来る
      //{hegoNew : "hego"} //名前を変更する機能を作るかは未定。idである以上いらなくないか？
    ],
    {
      RefIDs : { //RefListIDsが設定されている場合、RefIDsが必要。必要なメンバを設定する。
        hoge : "hogehoge", //RefListIDと、RefIDをセットで記述していく。
        //nullableは入力しなければデフォルトでnull
        hegoNew : "hegohego",
      }
      "memOverride?" : ["String"],
    }
  ],
  "nullableObj?" : {
    member1 : 31,
  }
}

//使用側 概念コード
//let item = itemList3[0]; //item は hogehoge と hegohego を参照している
//item.RefIDs.hoge.mem <これは"a"である
//item.hoge.mem <直接メンバとして呼び出すことも可能。同名のメンバがある場合はそちらが優先。
//list idは究極的なところ、一度作ったらもう二度と変えられないと思う。メンバ名は変えられるようになってるので、必要ならそっちを変えればよろしかろう。
//item.hoge.mem = "b" //参照越しに書き換えてもいいのか？　わからないけれど、システムレベルで禁止する必要はないだろう。必要なければ使わなければ良い。
//存在自体が害悪になるような機能ならライブラリ側で勝手に隠せば良い。
//個別にoverrideしたいケースがあるかもしれないが、override機能を実装するのはやはりおかしいと思うので（これはただのデータ記述システムであり、オブジェクト指向言語じゃない。
//nullableなメンバを上書きして、実質overrideのようにした方が良かろうと思う。
////item.memOverride = "c" //nullableなメンバを書き換える
//if(item.memOverride != null){ return item.memOverride } else{ return item.mem }

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