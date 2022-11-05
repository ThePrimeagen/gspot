use std::io::BufRead;

use anyhow::Result;
use interpreterbook::repl::Repl;

const PROMPT: &'static str = ">>";

fn main() -> Result<()> {
    let repl = Repl::new();

    let stdin = std::io::stdin();

    loop {
        println!("{}", PROMPT);
        if let Some(Ok(ref line)) = stdin.lock().lines().next() {
            for item in repl.line(line).iter() {
                println!("{:?}", item);
            }
        }
    }
}

