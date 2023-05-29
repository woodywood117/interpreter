use lexer::Lexer;
use std::io::{Stdin, Stdout, Write, BufRead, BufReader};

const PROMPT: &str = ">> ";

pub fn start(input: Stdin, mut output: Stdout) {
    let mut scanner = BufReader::new(input);

    loop {
        print!("{}", PROMPT);
        output.flush().unwrap();
        let mut line = String::new();
        scanner.read_line(&mut line).unwrap();
        let lexer = Lexer::new(line);

        for token in lexer {
            println!("{:?}", token);
        }
    }
}