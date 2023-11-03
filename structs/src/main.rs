fn main() {

    let user1 = User {
        email: String::from("amine@example.com"),
        username: String::from("monad123"),
        active: true,
        sign_in_count: 1,
    };


    let user2 = build_user("user2".to_string(), "user2@example.com".to_string());

    // Struct update syntax 

    let user2_bis = User {
        email: String::from("my_email@email.com"),
        username: user1.username,
        ..user2
    };
    println!("Hello {} !", user2_bis.username);


    let black = Color(0,0,0);
}

// we used String instead of &str so that the Struct User owns all of its data
// if we want to reference data we need "lifetimes"
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active:bool,
}

// field init shorthand

fn build_user(username: String, email: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count:23,
    }
}

//tuple struct

struct Color(i32, i32, i32);