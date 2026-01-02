#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(i32),      // 0, 10, 20, 21...
    Bool(bool),       // T, F (парсим сразу в true/false)
    Keyword(String),    // X, A, N, I, G, P, E, L, S
    Label(String),    // P1, P2, P3
    Ident(String),    // Всё остальное
}


pub fn start(content: String) {
    match tokenize(&content) {
        Ok(tokens) => {
            println!("Токены: {:#?}", tokens);
        }
        Err(e) => eprintln!("   >>  ! Ошибка токенизации: {}", e),
    }
}
// fn is_num(s: &str) -> bool {
//     s.chars().all(|c| c.is_ascii_digit())
// }

fn tokenize(input: &str) -> Result<Vec<Vec<Token>>, String> {
    let mut all_tokens = Vec::new();
    
    // Разбиваем входной текст на строки по ';'
    // filter() убирает пустые строки после последней ';'
    for line in input.split(';').filter(|s| !s.is_empty()) {
        let mut tokens = Vec::new();
        let mut buffer = String::new();
        
        for ch in line.chars() {
            if ch.is_whitespace() {
                if !buffer.is_empty() {
                    tokens.push(parse_token(&buffer)?);
                    buffer.clear();
                }
            } else {
                buffer.push(ch);
            }
        }
        
        // Обрабатываем последний токен в строке
        if !buffer.is_empty() {
            tokens.push(parse_token(&buffer)?);
        }
        
        all_tokens.push(tokens);
    }
    
    Ok(all_tokens)
}

fn parse_token(s: &str) -> Result<Token, String> {
    // Число: "123", "0", "10"
    if s.chars().all(|c| c.is_ascii_digit()) {
        return s.parse::<i32>()
            .map(Token::Number)
            .map_err(|e| format!("   >>  ! Невалидное число '{}': {}", s, e));
    }

    // Булевы: "T", "F"
    if s == "T" || s == "F" {
        return Ok(Token::Bool(s == "T"));
    }

    // Односимвольные ключевые слова: "X", "A", "N", "I", "G", "P", "E", "L", "S"
    if s.len() == 1 {
        let c = s.chars().next().unwrap();
        if matches!(c, 'X' | 'A' | 'N' | 'I' | 'G' | 'P' | 'E' | 'L' | 'S' | ';') {
            return Ok(Token::Keyword(c.to_string()));
        }
    }
    
    
    if s.len() >= 2 { 
        // Метки: "P1", "P2", "P3" (буква + цифры)
        if s.starts_with(|c: char| c.is_alphabetic()) && s[1..].chars().all(|c| c.is_ascii_digit()) {
            return Ok(Token::Label(s.to_string()));
        }
        if matches!(s, "IC") {
            return Ok(Token::Keyword(s.to_string()));
        }
    }

    // Идентификатор: всё остальное "valid_name"
    if s.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Ok(Token::Ident(s.to_string()));
    }

    Err(format!("   >>  ! Невалидный токен: '{}'", s))
}