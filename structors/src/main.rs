struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    user1.email = String::from("anyEmail@example.com");

    println!("User 1 email: {}", user1.email);
    println!("User 1 username: {}", user1.username);
    println!("User 1 active: {}", user1.active);
    println!("User 1 sign in count: {}", user1.sign_in_count);
}