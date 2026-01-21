// main.rs
use std::env;
use std::fs;
use std::process;
use std::path::Path;

mod preprocessors;
mod vm;
mod tokens;
mod to_bytecode;
mod namezator;
mod name_map;
mod importer;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        show_usage(&args[0]);
        process::exit(1);
    }
    
    // Парсим аргументы
    let mut debug_mode = false;
    let mut filename_arg = None;
    
    // Пропускаем первый аргумент (имя программы)
    for i in 1..args.len() {
        let arg = &args[i];
        
        // Если аргумент начинается с '-', это флаг
        if arg.starts_with('-') {
            if arg == "--debug" || arg == "-debug" {
                debug_mode = true;
            }
            // Игнорируем другие флаги
        } else {
            // Это не флаг, значит это имя файла
            filename_arg = Some(arg.clone());
            // После имени файла не должно быть других аргументов (кроме флагов)
        }
    }
    
    let filename = match filename_arg {
        Some(f) => f,
        None => {
            eprintln!("Ошибка: не указано имя файла");
            show_usage(&args[0]);
            process::exit(1);
        }
    };
    
    // Проверяем существование файла
    if !Path::new(&filename).exists() {
        eprintln!("Файл '{}' не найден", filename);
        eprintln!("Текущая директория: {:?}", std::env::current_dir().unwrap());
        process::exit(1);
    }
    
    if debug_mode {
        println!("=== ЗАПУСК С ОТЛАДКОЙ ===");
        println!("Файл: {}", filename);
    }
    
    let base_path = Path::new(&filename).parent().unwrap_or(Path::new("."));
    
    match fs::read_to_string(&filename) {
        Ok(content) => {
            let tokens = tokens::start(content);
            match tokens {
                Ok(tokens) => {
                    if tokens.is_empty() { 
                        println!("Файл пуст");
                        return; 
                    }
                    
                    // Сохраняем исходные токены
                    if debug_mode {
                        save_tokens_to_file(&filename, "tokens.txt", &tokens);
                    }
                    let tokens = preprocessors::expand_ranges(tokens);
                    let tokens = importer::importing(tokens, base_path).expect("Ошибка импорта");
                    let tokens = preprocessors::else_processing(tokens);
                    let expanded_tokens = preprocessors::expand(tokens);
                    
                    
                    match expanded_tokens {
                        tokens => {
                            // Сохраняем расширенные токены
                            if debug_mode {
                                save_tokens_to_file(&filename, "expanded.txt", &tokens);
                            }
                            
                            let (tokens, ident_name_map) = namezator::namezating(tokens, debug_mode);
                            
                            let bytecode = to_bytecode::to_bytecode(tokens, &ident_name_map);
                            match bytecode {
                                Ok(bytecode) => {
                                    if bytecode.is_empty() {
                                        println!("Байткод пуст");
                                    } else {
                                        // Сохраняем байткод
                                        // if debug_mode {
                                        //     save_bytecode_to_file(&filename, "bytecode.txt", &bytecode);
                                        // }
                                        
                                        vm::start(bytecode, ident_name_map);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Ошибка байткода: {}", e);
                                    process::exit(1);
                                }
                            }
                        }
                    }
                }
                Err((e, _)) => {
                    eprintln!("Ошибка токенизации: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Ошибка чтения файла '{}': {}", filename, e);
            process::exit(1);
        }
    }
}

fn show_usage(program_name: &str) {
    eprintln!("Использование: {} [--debug] <файл>", program_name);
    eprintln!();
    eprintln!("Примеры:");
    eprintln!("  {} program.bpl", program_name);
    eprintln!("  {} --debug program.bpl", program_name);
}

// Функции для сохранения отладочной информации:

// Форматирование одного токена
fn format_token(token: &tokens::RawToken) -> String {
    match token {
        tokens::RawToken::Bool(b, _) => format!("{}", b),
        tokens::RawToken::Number(n, _) => format!("{}", n),
        tokens::RawToken::Keyword(k, _) => format!("{}", k),
        tokens::RawToken::LabelP(s, _) => format!("P.{}", s),
        tokens::RawToken::LabelPD(s, _) => format!("PD.{}", s),
    }
}

// Функция для сохранения токенов в файл
fn save_tokens_to_file(original_name: &str, suffix: &str, tokens: &[Vec<tokens::RawToken>]) {
    let base_name = Path::new(original_name)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    
    let debug_name = format!("{}_{}", base_name, suffix);
    
    let mut content = String::new();
    content.push_str("=== ТОКЕНЫ ===\n\n");
    for (i, line) in tokens.iter().enumerate() {
        content.push_str(&format!("{}: ", i + 1));
        for token in line {
            content.push_str(&format!("{} ", format_token(token)));
        }
        content.push('\n');
    }
    
    if let Err(e) = fs::write(&debug_name, content) {
        eprintln!("Не удалось сохранить файл {}: {}", debug_name, e);
    } else {
        println!("Сохранено: {}", debug_name);
    }
}