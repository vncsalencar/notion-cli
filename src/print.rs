use owo_colors::OwoColorize;
use std::fmt::Display;

pub fn print<S: AsRef<str> + Display>(text: S) {
    let header = "[notion-cli]".green();
    println!("{header} {text}");
}
