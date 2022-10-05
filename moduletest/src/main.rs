struct SeaCreature{
    //Stringは構造体らしい。
    animal_type: String,
    name: String,
    arms: i32,
    legs: f32,
    weapon: String,
    //フィールドは、データ構造とキーワードを紐づける
    //値はプリミティブ型もしくはデータ構造を指定可能。
    //メモリ上で隣り合うデータの配置をコンパイラに伝えるが、メモリとコンパイルに関して
    //知識が浅いので用復習
}