struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Cordinates(i32, i32, i32);



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


    let user2 = User {
        email: String::from("user2@something.com"),
        ..user1
    };

    user1.username = String::from("user1Updated");

    // println!("User 1 email: {}", user1.email);
    // println!("User 1 username: {}", user1.username);
    // println!("User 1 active: {}", user1.active);
    // println!("User 1 sign in count: {}", user1.sign_in_count);

    println!("User 2 email: {}", user2.email);
    println!("User 2 username: {}", user2.username);
    println!("User 2 active: {}", user2.active);
    println!("User 2 sign in count: {}", user2.sign_in_count);

    let white = Color(255, 255, 255);
    let center = Cordinates(0, 0, 0);

    let Color(r, g, b) = white;
    let Cordinates(x, y, z) = center;

    println!("\nRGB values for white: {r}, {g}, {b}");
    println!("Cordinates for center: {x}, {y}, {z}");

}