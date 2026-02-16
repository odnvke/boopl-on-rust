use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::error::Error;

mod preprocessors;
mod vm;
mod tokens;
mod to_bytecode;
mod namezator;
mod name_map;
mod importer;
mod error;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // читаем аргументы
    let (filename, debug_mode) = parse_args()?;
    
    // читаем название файла
    if !Path::new(&filename).exists() {
        return Err(format!("Файл '{}' не найден", filename).into());
    }

    if debug_mode {
        println!("=== ЗАПУСК С ОТЛАДКОЙ ===\nФайл: {}", filename);
    }

    // Читаем файл
    let content = fs::read_to_string(&filename)?;
    let base_path = Path::new(&filename).parent().unwrap_or(Path::new("."));
    


    // =======================
    //     токенизация
    // =======================
    let raw_tokens = tokens::start(content)?;

    
    if raw_tokens.is_empty() {
        println!("Файл пуст");
        return Ok(());
    }

    // Debug: сохраняем исходные токены
    if debug_mode {
        save_tokens_to_file(&filename, "tokens.txt", &raw_tokens);
    }



    // ================================================================
    //     Конвейер препроцессинга (каждый шаг может вернуть ошибку)
    // ================================================================
    let tokens = preprocessors::expand_ranges(raw_tokens)?;
    let tokens = importer::importing(tokens, base_path)?;
    let tokens = preprocessors::else_processing(tokens)?;
    let expanded = preprocessors::expand(tokens)?;

    if debug_mode {
        save_tokens_to_file(&filename, "expanded.txt", &expanded);
    }


    // ====================
    //      змена имён
    // ====================
    let (processed_tokens, ident_map) = namezator::namezating(expanded, debug_mode);
    
    if debug_mode {
        // Таблица имён уже печатается внутри namezating
    }


    // ============================
    //      Генерация байткода
    // ============================
    let bytecode = to_bytecode::to_bytecode(processed_tokens, &ident_map)?;
    
    if bytecode.is_empty() {
        println!("Байткод пуст");
        return Ok(());
    }

    if debug_mode {
        // save_bytecode_to_file(&filename, "bytecode.txt", &bytecode);
    }



    // =======================
    //       Иполнение
    // =======================
    vm::start(bytecode, ident_map)?;
    
    Ok(())
}





    // ===========================
    //          Утилиты
    // ===========================
fn parse_args() -> Result<(String, bool), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return Err("Использование: boopl [--debug] <файл>".into());
    }

    let mut debug = false;
    let mut file = None;

    for arg in &args[1..] {
        if arg == "--debug" || arg == "-debug" {
            debug = true;
        } else if !arg.starts_with('-') {
            file = Some(arg.clone());
        }
    }

    match file {
        Some(f) => Ok((f, debug)),
        None => Err("Ошибка: не указано имя файла".into()),
    }
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