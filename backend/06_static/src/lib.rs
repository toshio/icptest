use std::include_str;

const MESSAGE:&str = include_str!("../assets/message.txt"); // "こんにちは"

#[ic_cdk::query]
fn greet(name: String) -> String {
  format!("{}, {}", MESSAGE, name)
}
