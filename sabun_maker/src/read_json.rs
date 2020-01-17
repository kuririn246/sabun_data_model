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
    hugaNumber : 30,
    "@HugaNumber" : 30, //先頭が大文字のメンバ名はすべてシステムが予約しているので認められない。しかしメンバをどうしても大文字で始めたい場合、
                        //jsonでは戦闘に@をつけ、プログラム側では@なしでもアクセス可能にするという技も使えなくはなかろうと思う。

    //名前が?で終わるメンバはnull値を含むことが出来る。
    //プログラム側のメンバ名にはこの?は反映されず、型が変わるだけ(もし俺以外の実装が現れたらわからないが・・・）
    //初期値にnullを入れるには、特殊な記法を使う必要がある
    "hegoNumber?" : ["Num"], //型だけを指定し、値を入れない。これでnullになる

    //配列はいまのところnumber配列、string配列、number配列の配列の4通り。
    numArray : [0, 3, 10],
    //最初に型を示し、その後初期値をいれる事もできる。
    numArray2 : [ "Num-Array", [0, 3, 10] ],

    emptyNumArray : [ "Num-Array", [] ], //初期値が空配列のnum-array
    "nullableNumArray?" : [ "Num-Array" ], //nullable配列の初期値をnullにする場合
    numArray2 : [ "Num-Array2", [[2,3], [3,1]] ], //二次元のnumarray
    numArray2_2 : [[2,3], [3,1]], //型を指定しないバージョン


	strArray : [ "Str-Array", ["hoge", "hogehoge"] ], //文字列配列
	//strArray : ["hoge", "hogehoge"], //直接文字列配列を入れるのはややこしいので認めない。Str-Arrayと書く必要がある

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
      {hegoNew : "hego"} //名前を変更する
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
//let item = itemList3[0]; //item は hogehoge と hegohego を継承している
//item.RefIDs.hoge.mem <これは"a"である
//item.hoge.mem <直接メンバとして呼び出すことも可能。同名のメンバがある場合はそちらが優先
//item.hoge.mem = "b" //参照越しに書き換えてもいいのかなあ・・・？
//item.memOverride = "c" //nullableなメンバを書き換える
//overrideしたいケースがあるかもしれないが、直接overrideするのではなく、nullableなメンバを上書きしたら実質overrideのようにした方が良かろうと思う
//継承の仕組みは人間には複雑過ぎ、事故のもと。
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