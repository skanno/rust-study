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
}

// 型を返す
fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}
