extern crate mal_rust;

use mal_rust::readline::Readline;
use mal_rust::reader::read_str;
use mal_rust::printer::pr_str;
use mal_rust::types::{MalType, MalErr};

fn main() {
    let mut readline = Readline::new("user> ");
    loop {
        match readline.get() {
            Some(line) => {
                if line.len() > 0 {
                    rep(&line)
                }
            }
            None => break,
        }
    }
    readline.save_history();
}

fn rep(input: &str) {
    match read_str(input) {
        Ok(out) => println!("{}", pr_str(out)),
        Err(MalErr::UNEXPECTED(e)) => println!("Unexpected {}", e),
        Err(MalErr::EOF) => println!("EOF"),
    }
    //let out = eval(out);

}

