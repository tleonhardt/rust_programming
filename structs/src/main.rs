struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    user1.email = String::from("anotheremail@example.com");

    // The syntax .. specifies the remaining fields not explicitly set should have same values as user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

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
