use serde_json::{Result, Value};

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
    "hegoNumber?" : null,

    //配列はnumber配列、string配列、bool配列、それぞれ中身がnullableであるかの6通りとなる。最初に型を示し、その後初期値をいれる。
    numArray : [ "Num-Array", 0, 3, 10 ],
    nullableNumArray : [ "Num?-Array", 0, null, 10 ], //nullableの場合num?のようになる
    emptyNumArray : [ "Num-Array" ], //初期値が空配列のnum-array
    emptyNullableNumArray : [ "Num?-Array" ], //初期値が空配列のnullableなnumのarray

    "numArray?" : [ "Num-Array?" ] //初期値がnullの場合、array?とする
    "numArray2?" : [ "Num?-Array?" ] //num?-array?もある。nullable整数配列であり初期値はnullである
	"strArray" : [ "Str-Array", "hoge", "hogehoge" ], //文字列配列
	"boolArray" : [ "Bool-Array", true, false ], //bool配列(必要か？）

	hogeList : [
		"List", //listは配列とは違う。オブジェクトのコレクションを作るためにはlistを使う必要がある。
		["ListID", "hogehoge"], //任意でlist-IDを与えることが出来る。list-IDは全データの中で一意である必要がある。
		["Dummy", "dummy"], //dummyのIDを設定。デフォルト値を設定でき、実際のリストには加わらない。
		{
			ID : "dummy",
			hogeNumber : 0,
			hogeString : "hoge"
		},
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

	"Rename" : { prevName : "currentName",
	                prevName2 : "currentName2"... }, //メンバ名の変更をした場合、これを書いておくことで自動でメンバ名の対応表を作ってくれる。

	"member?" : 3,
	"member2?" : 4,
	"IfUndefined" : {
      member : null,  //スキーマ変換時にmemberが元のスキーマに定義されていなかった場合、初期値をnullにする。
	                  //プログラム側ではnullを発見したら、特定の計算式を使ってmemberに初期値を入れることになるだろう。
	  member2 : null, //多分この機能はnullを入れる以外に使わないと思う・・・
	}

	hogeList2 : [
	  "List",
	  {
	    ID : "first",
	    member1 : "hoge"
	  },
	  {
	    ID : "second",
	    IsDefault : "true", //デフォルトのメンバにisDefaultを設定することでデフォルトオブジェクトを最初のものから変えることができる。
	    member2 : "this_is_default", //これがデフォルト値となり、差分が取られる。
	  }
	]

	commonType : {
	  "TypeName" : "thisTypesName",
	  "IsDefault" : true, //typeNameが同じものは同じ型になる。型のデフォルトが存在する。
	  memberName : "commonMemberName",
	}
  }

  commonType2 : {
    TypeName : "typeName", //同じ型名にすれば同じ型と認識される。これは主にプログラム側から使う設定
    memberName : "this is common",
  }

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
  ]

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
  ]

  itemList : [
    "List",
    ["AutoID"],
    ["RefListID", "weapon"],
    {
      RefID : "doutanuki",
      atk : 8 //上書きしてみる
    }
  ]

  itemList2 : [
    "List",
    ["AutoID"],
    ["RefListIDs", "weapons", "usables"], //RefListIDsを設定すると、アイテムにはRefListIDが絶対に必要。
    {
      RefListID : "weapon",
      RefID : "doutanuki",
    }
    {
      RefListID : "usable",
      RefID : "yakusou",
    }
  ]

  hogeList : [
    "List",
    ["ListID", "hoge"],
    {
      ID : "hogehoge"
    }
  ],

  hugaList: [
    "List",
    ["ListID", "huga"],
    {
      ID : "hugahuga"
    }
  ],

  itemList3 : [
    "List",
    ["AutoID"],
    ["MultipleRefListIDs", {
      impl1 : ["weapons", "usables"], /
      hoge : ["hoge"],
      "huga?" : ["huga"] //nullableにも出来る
    }]
  ]

}


差分生成システムとは直接関係しないが、俺の考えるデータモデルでは、データの継承ができる。
{
	refID : "first",
	hogeNumber : 21 //オーバーライド
}

listIDとIDをrefとして指定することで、そのアイテムを参照し、さらに自分のところでオーバーライドできる。オーバーライドしても元データは変化しない。
Ref-list-IDはいちいち指定しなくても、このアイテムの親リストにRef-list-IDが一つだけあればそれを使う

itemList : [
	"list",
	["listID", "itemList"], //これは参照可能にするべきではないのでlistIDをつけるのは間違いだな・・・参照可能にするとアイテムを削除できない縛りを設けるつもりだ
	["refListID", "usables"], //このリスト内のアイテムはrefする時にこのlistから探す
	{
		ID : "1",
		refID : "yakusou",
		hogeString : "huga",
		mishikibetsu : true,
		unknown_name : "緑の草",
	}
]

つまりあるリストのアイテムを継承するアイテムのリストというものが自然に作られる設計になっている。
たとえば風来のシレンのアイテムリストがある時に、それぞれのアイテムはそれがどういうものであるのか（薬草とかどうたぬきとか）を継承し、
さらに+1したり、名前が未識別だったりといった付加情報も追加する必要がある。そういうのを自然に表現できる。
この継承システムをプログラム側に組み込みたいと思っているが、差分生成システムと全然関係がないので勝手に組み込んでいいものでもなさそうだ。
でも組み込むつもりだが。

また、refを使えば、不定型を実装できる。list内のitemが一つのlistからrefするなら同じ型にできるが、
薬草とどうたぬきのデータ型が同じとは思えない。どちらもrefしたいのでそうなるのが自然かなという気がする。
Rustでいえばenumにしてパターンマッチすることになるので、どれが来る可能性があるのか予め列挙しておくべきだろう。

["refListID", "weapons", "usables" ], //このリスト内のオブジェクトはrefする時にこれらのリストのうちどれかを使う

{
	refListID : "weapons",
	refID : "doutanuki"
}

単一継承では済まない場合も考え、多重継承も可能にしておくべきだろう
["multipleRefListIDs",
  {
    impl1 : //どのリスト群を使うのかを指定するための名前
    ["weapons", "usables"], //このやり方だと武器でありかつ使用可能であるものを記述できないので不適切と思われる。 nullableな weapon と usableを別個で作るべきか
    hoge : ["hoge"],
    "hego?" : ["hego"] //nullableにも出来る
  },
]

{
  refIDs : {
    impl1 : ["weapons", "doutanuki"],//リストとIDを指定する。
    hoge : ["hogeItem"], //一つしかlistがない場合は１つ目を省略できる
    //hoge : "hogeItem", //角カッコをなくしてしまってもいい
    hego : null //nullableの場合nullにしても良い。nullableの場合はnullと明示しなくてもデフォルトでnullになる。
  }
}
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(v)
}