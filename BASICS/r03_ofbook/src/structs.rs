// https://doc.rust-lang.org/book/ch05-01-defining-structs.html


// Normal struct
#[derive(Debug)]  // Makes this struct printable with println("{:?}")
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}  // No semicolon in contrast to C++


// Methods
// Static methods are named 'associated functions' in Rust and don't take '&self' as first parameter.
// Like in Javascript, there may be multiple 'impl' blocks.
impl User {

    // 'self' keyword lets Rust know that this is a regular (non-static) method
    fn to_string(&self) -> String {
        let active = if self.active { "active"} else { "not active" };
        String::from(format!("{{ User: {}, e-mail: {}, sign_in_count: {}, {} }}", 
                    self.username, self.email, self.sign_in_count, active))
    }

    // An associated function (static method)
    fn create(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}



fn user_struct_usage() {

    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("  * u1: {:?}", u1);  // possible due to 'Debug' annotation
    println!("  * Using self-defined 'toString()' method: {}", u1.to_string());

    let u2 = User::create("u2@gmail.com".to_string(), "u2".to_string());
    println!("  * u2 username: {}", u2.username);
    println!("  * u2 email: {}", u2.email);
}


// Tuple Structs without Named Fields
struct Color(i32, i32, i32);

// Empty (unit-like) structs
// struct MyEmptyStruct {}

fn tuple_struct_usage() {
    let c1 = Color(1, 1, 1);
    println!("  * Color c1: ({}, {}, {})", c1.0, c1.1, c1.2);

}



pub fn demo() {

    println!("== structs demo begin ==");

    user_struct_usage();
    tuple_struct_usage();

    println!("== structs demo end ==");
    println!();
}
