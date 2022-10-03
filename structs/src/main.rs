struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;


fn main() {

    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    user1.email = String::from("test@example.com");

    println!("{}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);

    let user2 = User {
        email: String::from("new@example.com"),
        ..user1
    };

    println!("{}", user2.email);
    println!("{}", user2.username);
    println!("{}", user2.active);
    println!("{}", user2.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
