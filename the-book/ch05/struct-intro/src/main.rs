#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 3,
    };
    user1.email = String::from("hogehoge@yahoo.co.jp");
    let user2 = build_user(
        String::from("aaaa@gmail.com"),
        String::from("aaaa"),
    );

    println!("{user1:?}");
    println!("{user2:?}");
    println!("Hello, world!");
}
