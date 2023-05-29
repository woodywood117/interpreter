use std::io::{stdin, stdout};

fn main() {
    println!("Monkey REPL");
    println!("Feel free to type in commands.");
    repl::start(stdin(), stdout());
}
