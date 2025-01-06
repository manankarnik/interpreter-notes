use std::{
    env,
    io::{self, Write},
    path, process,
};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        interpret_file(&args[1])?;
    } else {
        let program = path::Path::new(args[0].as_str())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        eprintln!("Usage: {program} [file]");
        process::exit(64);
    }
    Ok(())
}

fn interpret(line: &str) {
    let tokens: Vec<&str> = line.split(" ").collect();
    for token in tokens {
        println!("{token}");
    }
}

fn interpret_file(file_name: &str) -> io::Result<()> {
    let lines = std::fs::read_to_string(file_name)?;
    let lines: Vec<&str> = lines.split("\n").collect();
    for line in lines {
        interpret(&line);
    }
    Ok(())
}

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => interpret(&input),
            Err(e) => {
                eprintln!("Error reading input: {e}");
                break;
            }
        }
    }
}
