//use std::char;
use std::env;
use std::fs;
use std::process;
//use std::io;
// use std::collections::HashMap;
//use std::str::Chars;

mod tokens;

fn main() {
    // Получаем аргументы
    let args: Vec<String> = env::args().collect();
    
    // Проверяем, что передано имя файла
    if args.len() < 2 {
        eprintln!("Использование: {} <имя_файла>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    // Читаем файл
    match fs::read_to_string(filename) {
        Ok(content) => tokens::start(content),
        Err(e) => {
            eprintln!("Ошибка при чтении файла '{}': {}", filename, e);
            process::exit(1);
        }
    }

    //let name = String::new();
    
    //io::stdin().read_line(&mut name);
}



