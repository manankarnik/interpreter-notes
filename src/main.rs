use std::{
    env,
    fmt::{self, Display},
    io::{self, Write},
    path, process,
};

#[rustfmt::skip]
#[derive(Debug)]
enum TokenType<'a> {
    LParen, RParen,
    LCurly, RCurly,
    Semicolon, Comma, Dot,
    Plus, Minus, Star,

    Equal, EqualEqual,
    Not, NotEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Ident(&'a str), String(&'a str), Number(&'a str),

    And, Class, Else, False, Fn, For, If, Let,
    Null, Or, Print, Return, Super, True, While,

    EOF
}

struct Token<'a> {
    token_type: TokenType<'a>,
    lexeme: &'a str,
    line: usize,
    column: usize,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{type: {:?}, lexeme: '{}', line: {}, column: {}}}",
            self.token_type, self.lexeme, self.line, self.column
        )
    }
}

struct Scanner<'a> {
    file_name: Option<&'a str>,
    source: &'a str,
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str, file_name: Option<&'a str>) -> Self {
        Self {
            file_name,
            source,
            tokens: vec![],
        }
    }

    fn scan_tokens(&mut self) {
        let mut current = 0;
        let mut start = 0;
        let mut line = 1;
        let mut column = 0;

        for c in self.source.chars() {
            current += 1;
            column += 1;
            let literal = self.source.get(start..current).unwrap();
            start = current;

            #[rustfmt::skip]
            let token_type: Option<TokenType> = match c {
                '\n' => {
                    line += 1;
                    column = 0;
                    None
                },

                '(' => Some(TokenType::LParen),
                ')' => Some(TokenType::RParen),
                '{' => Some(TokenType::LCurly),
                '}' => Some(TokenType::RCurly),

                ',' => Some(TokenType::Comma),
                '.' => Some(TokenType::Dot),
                '+' => Some(TokenType::Plus),
                '-' => Some(TokenType::Minus),
                '*' => Some(TokenType::Star),
                '=' => Some(TokenType::Equal),

                ';' => Some(TokenType::Semicolon),

                c => {
                    report_error(self.file_name, line, column, format!("Unexpected character '{c}'").as_str());
                    None
                },
            };

            if let Some(token_type) = token_type {
                self.tokens.push(Token {
                    token_type,
                    lexeme: literal,
                    line,
                    column,
                });
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "",
            line,
            column,
        });
    }
}

fn report_error(file_name: Option<&str>, line: usize, column: usize, message: &str) {
    if let Some(file) = file_name {
        eprintln!("{file}:{line}:{column}: Error - {message}");
    } else {
        eprintln!("{line}:{column}: Error - {message}");
    }
}

fn interpret(source: &str, file_name: Option<&str>) {
    let mut scanner = Scanner::new(&source, file_name);
    scanner.scan_tokens();
    for token in scanner.tokens.iter() {
        println!("{token}");
    }
}

fn interpret_file(file_name: &str) -> io::Result<()> {
    let lines = std::fs::read_to_string(file_name)?;
    interpret(&lines, Some(file_name));
    Ok(())
}

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => interpret(&input, None),
            Err(e) => {
                eprintln!("Error reading input: {e}");
                break;
            }
        }
    }
}

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
