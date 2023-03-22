use std::env;

fn main() {
    like_dotenv::config().unwrap();

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
}
