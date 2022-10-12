//お決まりのインポート的な奴
use std::io::{self,Read};

fn main() {
    //入力された全てのバイトに対して処理を行う
    //io::stdin()は、input関数みたいなやつ
    for b in io::stdin().bytes(){
        let c = b.unwrap() as char;
        println!("{}", c);
    }
}
