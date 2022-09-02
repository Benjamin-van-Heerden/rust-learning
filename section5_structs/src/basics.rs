fn main() {
    let user1 = User {
        active: true,
        email: String::from("mail@domain.com"),
        sign_in_count: 1,
        username: String::from("my_name"),
    };

    // use dot notation to get a specific value from a struct
    println!("{}", user1.username);

    // if the instance is mutable we can also update with dot notation
    let mut user2 = User {
        active: true,
        email: String::from("mail@domain.com"),
        sign_in_count: 1,
        username: String::from("my_name"),
    };

    user2.username = String::from("another_name");

    // struct update syntax makes defining new from old easy
    let user3 = User {
        email: String::from("another@a.com"),
        ..user1
    };
    // this means that email is set explicitly, but the rest of the properties assume those as in user1
    // the downside here is that we can no longer use user1, user2 has been declared to "be" user1 (at least in part)
    // therefore user1 is no longer valid
    // println!("{}", user1.username); - won't work!

    // tuple structs can also be useful - access by .index
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!("{}", black.0);
    let origin = Point(0, 0, 0);

    // unit-like structs - should be useful for pattern matching
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
