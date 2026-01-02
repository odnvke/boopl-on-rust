use std::{collections::HashMap, env, fs, process};

type Mem = HashMap<i32, i32>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} file", args[0]);
        process::exit(1);
    }

    let src = fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("cannot read {}: {}", args[1], e);
        process::exit(1);
    });

    if let Err(e) = run(&src) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}

fn run(src: &str) -> Result<(), String> {
    let tokens: Vec<&str> = src
        .split_whitespace()
        .collect();

    let mut mem = Mem::new();
    let mut i = 0;

    while i < tokens.len() {
        if tokens[i] == "I" {
            // I a b
            if i + 2 >= tokens.len() {
                return Err("I needs two numbers".into());
            }
            let a = parse(tokens[i + 1])?;
            let b = parse(tokens[i + 2])?;
            i += 3;

            if *mem.get(&a).unwrap_or(&0) == 1 {
                println!("{}", b);
            }
        } else if is_num(tokens[i]) {
            // addr val
            if i + 1 >= tokens.len() {
                return Err("pair needs two numbers".into());
            }
            let addr = parse(tokens[i])?;
            let val  = parse(tokens[i + 1])?;
            i += 2;
            mem.insert(addr, val);
        } else {
            return Err(format!("unknown token: {}", tokens[i]));
        }
    }
    Ok(())
}

fn is_num(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

fn parse(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| format!("'{}' is not a number", s))
}