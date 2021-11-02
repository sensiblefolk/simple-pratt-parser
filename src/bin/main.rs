use std::io;
use std::io::BufRead;
use simple_pratt_parser::expr;

fn main() {
    for line in io::stdin().lock().lines() {
        let line  = line.unwrap();
        let s = expr(&line);
        println!("{}", s)
    }
}