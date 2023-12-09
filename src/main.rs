pub mod todo_args;
pub mod todo_engine;
pub mod todo_command;

use clap::Parser;
use crate::todo_args::TodoArgs;


fn main() {
    let args = TodoArgs::parse();
    let engine = todo_engine::todo_engine::new(args);

    // process command
    let output = engine.process_command();

    println!("{}", output);
}
