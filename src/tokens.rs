//use std::{fmt::format, i32};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Bool(bool),
    Keyword(String),
    LabelP(i32),     // P.10
    LabelPD(i32),    // PD.10
    Number(i32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum RawToken {
    Bool(bool),
    Keyword(String),
    LabelP(String),     // P.10
    LabelPD(String),    // PD.10
    Number(String)
}

pub fn start(content: String) -> Result<Vec<Vec<RawToken>>, String> {
    let content = remove_comments(content);
    
    match tokenize(&content) {
        Ok(tokens) => {
            if tokens[0].len() == 0 {println!("\nТокенов нет")}
            else {println!("\nТокены: {:?} \n", tokens);}
            Ok(tokens)
        }
        Err(e) => {
            eprintln!("   >>  ! Ошибка токенизации: {} \n", e);
            Err(e)
        }
    }
}
// fn is_num(s: &str) -> bool {
//     s.chars().all(|c| c.is_ascii_digit())
// }

fn tokenize(input: &str) -> Result<Vec<Vec<RawToken>>, String> {
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
        if tokens.len() > 4 {
            return Err(format!("   >>  ! Слишком много токенов в строке: '{}'", line));
        }
        
        all_tokens.push(tokens);
    }
    
    Ok(all_tokens)
}

fn parse_token(s: &str) -> Result<RawToken, String> {
    // Булевы: "T", "F"
    if s == "T" || s == "F" {
        return Ok(RawToken::Bool(s == "T"));
    }

    // Односимвольные ключевые слова: "X", "A", "N", "I", "G", "P", "E", "L", "S"
    if s.len() == 1 {
        let c = s.chars().next().unwrap();
        if matches!(c, 'X' | 'A' | 'O' | 'N' | 'I' | 'G' | 'P' | 'E' | 'L' | 'S' | 'U' | ';') {
            return Ok(RawToken::Keyword(c.to_string()));
        }
    }
    
    
    if s.len() >= 2 { 
        // Метки: "P1", "P2", "P3" (буква + цифры)
        if s.contains('.') {
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() == 2 {
                return match parts[0] {
                    "P" => Ok(RawToken::LabelP(parts[1].to_string())),
                    "PD" => Ok(RawToken::LabelPD(parts[1].to_string())),
                    _ => Err(format!("не удальсь обработать тип указателя {}", parts[0]))                         
                }
                
            }
        }
        if matches!(s, "IC") {
            return Ok(RawToken::Keyword(s.to_string()));
        }
    }

        // Число: "123", "0", "10"
    if true {
        return s.parse::<String>()
            .map(RawToken::Number)
            .map_err(|e| format!("   >>  ! Невалидное число '{}': {}", s, e));
    }

    Err(format!("   >>  ! Невалидный токен: '{}'", s))
}

fn remove_comments(string: String) -> String {
    let mut new_string = "".to_string();
    let mut flag_multi = false;
    let mut flag_single = false;
    let mut _flag = false;

    let mut string = string.replace("\r\n", "\n");

    string = string.replace("/*", " /* ").replace("*/", " */ ").replace("//", " // ").replace("\n", " | ").replace(";", " ; ");

    for i in string.split(' ') {
        let mut flag = false;

        if i == "//" {
            // print!("удалён комментарий: ");
            flag_single = true
        }
        if flag_single || i == "|" {
            flag = true;
        }
        
        if i == "|" {
            flag_single = false;
            //println!()
        }
        
        if flag_single {
            // print!("{}", i)
        }
        

        if i == "/*" {
            flag_multi = true;
            print!("  >>  замечено начало мульти строчного комментария: '{}' удаляется: ", i)
        }

        if flag_multi {
            flag = true;
        }

        if i.contains("*/") {
            println!(" '{}'", i);
            flag_multi = false;
            println!("  >>  замечен конец мульти строчного комментария: '{}'", i)
        }
        if flag_multi {
            //print!(" {}", i)
        }

        if !flag {
            new_string.push(' ');
            new_string.push_str(i);
            //println!("add: '{}'", i)
        } else { 
            //println!("not add: '{}'", i)
        }
    }
    new_string
}