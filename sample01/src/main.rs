fn main() {
    // /_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/
    // /_ シャドーイングの実験
    // /_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/

    // シャドーイングは、別の型でもOK
    let num = String::from("123");
    println!("num type is {}", type_of(num));
    let num = 123;
    println!("num type is {}", type_of(num));

    // シャドーイングのスコープ
    let y = 1;
    println!("y = {}", y); // y = 1
    let y = 2;
    println!("y = {}", y); // y = 2
    {
        println!("y = {}", y); // y = 2
        let y = 3;
        println!("y = {}", y); // y = 3
        let y = 4;
        println!("y = {}", y); // y = 4
    }
    println!("y = {}", y); // y = 2

    // /_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/
    // /_ 所有権の実験
    // /_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/

    // str1はstr型でスタック領域に保存されているので、
    // str1の値はstr2にコピーされる。
    let str1 = "abc";
    let str2 = str1;
    println!("str1 = {}", str1);
    println!("str2 = {}", str2);

    // str3の型はString型でヒープ領域に保存されているので、
    // str3はstr4にmoveされたのでstr3を使うことはできない。
    let str3 = String::from("abc");
    let str4 = str3;
    // println!("str3 = {}", str3); // ← コンパイルでエラーになる。
    println!("str4 = {}", str4);

    // str5の型はString型でヒープ領域に保存されているが、
    // clone()でstr6へヒープ領域の値をコピーしているので、
    // str5の値はそのまま使える。
    let str5 = String::from("abc");
    let str6 = str5.clone();
    println!("str5 = {}", str5);
    println!("str6 = {}", str6);

    // 関数に変数を渡すとmoveされる。
    let str7 = String::from("abc");
    hoge(str7); // moveされた。
    // println!("str7 = {}", str7); // moveされたため変数は使用できない。

    // 関数に参照を渡すだけならmoveされない。
    let str8 = String::from("abc");
    fuga(&str8); // moveされていない。
    println!("str8 = {}", str8); // moveされていないため使用できる。

    // 同時に２つの変数をミュータブルにすることはできない。
    let mut str9 = String::from("abc");
    let str9_1 = &str9;
    // let str9_2 = &mut str9;
    // println!("{}, {}", str9, str9_1, str9_2);
    println!("{}, {}", str9, str9_1);

    // /_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/
    // /_ スライスの実験
    // /_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/_/

    let str10 = String::from("hello world"); // String型
    let str10_1 = &str10[..5]; // str型
    println!("str10_1 = {}", str10_1);
    println!("str10_1 type of {}", type_of(str10_1));
}

// 型を返す
fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

fn hoge (s :String) {
    println!("s = {}", s);
}

fn fuga (s :&String) {
    println!("s = {}", s);
}