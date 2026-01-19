pub fn parentheses_process(str: &Vec<char>) -> i32 {
    let s: String = str.iter().collect();
    let cleaned: String = s.chars().filter(|ch| *ch != '_').collect();

    if cleaned.is_empty() {return 1;}

    // Исправляем: s.iter() -> s.chars()
    // is_ascii_alphanumeric() проверяет буквы И цифры, возможно вам нужно is_ascii_digit()
    if cleaned.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
        // Используем match или expect/unwrap для обработки Result
        match cleaned.parse::<i32>() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Ошибка парсинга '{}' в число: {}", s, e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Ошибка: '{}' содержит не ASCII-буквенно-цифровые символы", s);
        std::process::exit(1);
    }
}