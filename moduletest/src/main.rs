fn example() -> i32 {
    let x = 42;
    // Rust の三項式
    let v = if x < 42 { -1 } else { 1 };
    println!("if より: {}", v);
    //1がvに入るはず。

    let food = "ハンバーガー";
    let result = match food {
        "ホットドッグ" => "ホットドッグです",
        // 単一の式で値を返す場合、中括弧は省略可能
        _ => "ホットドッグではありません",
    };
    println!("食品の識別: {}", result);
    //_ =>で、「それ以外の時」を表すので
    //ホットドッグではありませんが出力されるはず

    let v = {
        // ブロックのスコープは関数のスコープから分離されている
        let a = 1;
        let b = 2;
        a + b
    };
    println!("ブロックより: {}", v);
    //3が出力されるはず
    // Rust で関数の最後から値を返す慣用的な方法
    v + 4
}

fn main() {
    println!("関数より: {}", example());
}
//1,