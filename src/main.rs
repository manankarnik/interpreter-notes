use std::{io, io::Write, env};

fn main() {
    let args = env::args();
    if args.len() == 1 {
        repl();
    } else {
        todo!()
    }
}

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.as_str() {
            "exit\n" => return,
            value    => print!("{value}"),
        }
    }
}
