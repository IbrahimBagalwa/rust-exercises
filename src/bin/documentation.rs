/// A favorite color.
enum Color {
    Red,
    Green,
    Blue,
}

/// A piece of email.
struct Email {
    /// sender address.
    from: String,
    /// recipient address.
    to: String,
    /// subject of the email.
    subject: String,
    /// body of the email.
    body: String,
}

/// Add two numbers together.
fn add(a: i32, b: i32) -> i32 {
    a + b
}
/// you can generate a documentation using cargo tool by typing this command cargo doc --open

fn main() {}