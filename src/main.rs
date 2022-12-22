use std::{env, fs::read_to_string};

use interpreter::interpret;

use crate::lexer::learn_to_read;
mod interpreter;
mod lexer;
mod value;

fn main() {
    let Some(filename) = env::args().nth(1) else {
        println!("Provide a filename as an argument, weirdo");
        return
    };
    let Ok(input) = read_to_string(filename) else {
        println!("Could not read file. Idk, bye?");
        return
    };
    let tokens = learn_to_read(&input);
    interpret(tokens);
}
