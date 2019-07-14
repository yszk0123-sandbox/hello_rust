struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

struct Point(u32, u32, u32);

fn main() {
    let user = build_user(String::from("foo@example.com"), String::from("foo"));

    println!("Hello, {}!", user.username);
    println!(
        "email: {}, active: {}, sign_in_count: {}",
        user.email, user.active, user.sign_in_count
    );

    let user = logout(user);
    println!("Bye, {}", user.username);
    println!("active: {}", user.active);

    let point = Point(1, 2, 3);
    println!("point: ({}, {}, {})", point.0, point.1, point.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn logout(user: User) -> User {
    User {
        active: false,
        ..user
    }
}
