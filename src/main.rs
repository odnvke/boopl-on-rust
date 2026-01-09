//use std::char;
use std::env;
use std::fs;
use std::process;
//use std::io;
// use std::collections::HashMap;
//use std::str::Chars;

mod vm;
mod tokens;
mod to_bytecode;
mod namezator;
mod name_map;

fn main() {
    // Получаем аргументы
    let args: Vec<String> = env::args().collect();
    
    // Проверяем, что передано имя файла
    if args.len() < 2 {
        eprintln!(" ! Использование: {} <имя_файла>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    // Читаем файл
    match fs::read_to_string(filename) {
        Ok(content) => {
            let tokens = tokens::start(content);
            match tokens {
                Ok(tokens) => {
                    let (tokens, ident_name_map) = namezator::namezating(tokens);

                    let bytecode = to_bytecode::to_bytecode(tokens, &ident_name_map);
                    match bytecode {
                        Ok(bytecode) => {
                            if bytecode.is_empty() {println!("байткод пустой")}
                            else {println!("перевод в байткод успешен: \n{:?}\n", bytecode)}
                            vm::start(bytecode, ident_name_map);

                        }
                        Err(e) => {
                            eprintln!("\n ! перевод в байткод: \n{}", e)
                        }
                    }
                }

                Err((e, _)) => {
                    eprintln!("\n ! токенизация: \n{}", e);
                }
            }
        }

        Err(e) => {
            eprintln!("   >>  ! Ошибка при чтении файла '{}': {}", filename, e);
            process::exit(1);
        }
    }

    //let name = String::new();
    
    //io::stdin().read_line(&mut name);
}



