fn main() {
    // 変数と可変性
    let mut x = 5;
    println!("The value onf x is: {}", x);
    x = 6;
    println!("The value onf x is: {}", x);

    // 変数と定数の違い
    const MAX_POINTS: u32 = 100_000;
    println!("The value onf MAX_POINTS is: {}", MAX_POINTS);

    // シャドーイング
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value onf x is: {}", x);
    }
    println!("The value onf x is: {}", x);

    // 関数
    test();
    test2(6);
    five();
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // 制御フロー
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn test() {
    println!("Call, test.");
}

fn test2(i: i32) {
    println!("Call, test2.{}", i);
}

fn five()-> i32 {
    5
}
