fn main() {
    let s = String::from("abc def");
    println!("{}", first_word(&s));
    println!("{:?}", "aaa");
    println!("{:?}", String::from("abc"));
    println!("{:?}", [1, 2]);

    let s2 = "bbb";
    println!("{:?}", s2);
    let s3 = s2.clone();
    println!("{:?}", s3);

    let mut s4 = "aaaaaaa";
    println!("{:?}", s4);
    let &s5 = &s4;
    s4 = "bbbbbbb";
    println!("{:?}", s4.as_ptr());
    println!("{:?}", s5.as_ptr());

    let mut s6 = String::from("qqqq");
    let s7 = &s6;
    println!("{:?}", s6.as_ptr());
    println!("{:?}", s7.as_ptr());

    s6 = String::from("ddddd");
    println!("{:?}", s6.as_ptr());
    println!("{:?}", s7.as_ptr());
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}