
struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("someone@example.com"),
        user_name: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotherone@example.com");
    print_user(&user1);

    let user2 = build_user(String::from("user2@example.com"), String::from("user2"), &user1);
    print_user(&user2);
    print_user(&user1);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email: String, user_name: String, user: &User) -> User {
    User {
        email,
        user_name,
        ..*user
    }
}

fn print_user(user: &User) {
    println!("{} {} {} {}", user.user_name, user.email, user.sign_in_count, user.active);
}
