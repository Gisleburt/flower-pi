fn main() {
    let user = std::env::var("USER").unwrap();
    println!("Hello, {}!", user);
}
