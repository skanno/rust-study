struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };
    println!("user1.username = {}", user1.username);
    println!("user1.email = {}", user1.email);
    println!("user1.sign_in_count = {}", user1.sign_in_count);
    println!("user1.active = {}", user1.active);

    user1.email = String::from("anotheremail@example.com");
    println!("user1.email = {}", user1.email);

    let user2 = build_user(String::from("someone@example.com"), String::from("someusername123"));

    let user3 = User {
        username: String::from("user3@example.com"),
        email: String::from("user3"),
        sign_in_count: 1,
        active: true,
    };
    let user4 = User {
        username: String::from("user4@example.com"),
        email: String::from("user4"),
        ..user3
    };
    println!("user4.username = {}", user4.username);
    println!("user4.email = {}", user4.email);
    println!("user4.sign_in_count = {}", user4.sign_in_count);
    println!("user4.active = {}", user4.active);

    let color = Color(0 ,2, 4);
    let point = Point(0 ,0, 0);
    println!("color.1 = {}", color.1);
}

fn build_user(email: String, username: String) -> User {
    User {
        username, // フィールド名と変数名が同じ場合は省略可
        email,
        sign_in_count: 1,
        active: true,
    }
}