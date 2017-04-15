extern crate itertools;
extern crate termion;

mod interpreter;
mod interactive;

use std::io;
use std::io::prelude::*;

fn main() {
    let mut interp = interpreter::Interpreter::new();
     /* TODO if non- interactive

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", interp.execute_line(&line.unwrap()));
    }
    */
    interactive::run(interp);
}
