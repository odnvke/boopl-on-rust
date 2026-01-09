//use std::{fmt::format, i32};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Bool(bool, i32),
    Keyword(String, i32),
    LabelP(i32, i32),     // P.10
    LabelPD(i32, i32),    // PD.10
    Number(i32, i32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum RawToken {
    Bool(bool, i32),
    Keyword(String, i32),
    LabelP(String, i32),     // P.10
    LabelPD(String, i32),    // PD.10
    Number(String, i32)
}

pub fn start(content: String) -> Result<Vec<Vec<RawToken>>, (String, i32)> {
    //let content = remove_comments(content);
    
    match tokenize(&content) {
        Ok(tokens) => {
            if tokens[0].len() == 0 {println!("\nТокенов нет")}
            else {println!("\nТокены: {:?} \n", tokens);}
            Ok(tokens)
        }
        Err(e) => {
            Err(e)
        }
    }
}


fn tokenize(input: &str) -> Result<Vec<Vec<RawToken>>, (String, i32)> {
    let mut all_tokens = Vec::new();
    let mut current_line = 1;
    let mut in_single_comment = false;
    let mut in_multi_comment = false;
    let mut current_instruction = String::new();
    let mut instruction_line = 1;
    
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        // Считаем строки
        if ch == '\n' {
            current_line += 1;
            in_single_comment = false;
        }
        
        // Пропускаем комментарии
        if in_single_comment {
            continue;
        }
        
        if in_multi_comment {
            if ch == '*' && chars.peek() == Some(&'/') {
                chars.next(); // пропускаем '/'
                in_multi_comment = false;
            }
            continue;
        }
        
        // Проверяем начало комментариев
        if ch == '/' {
            match chars.peek() {
                Some(&'/') => {
                    chars.next(); // пропускаем второй '/'
                    in_single_comment = true;
                    continue;
                }
                Some(&'*') => {
                    chars.next(); // пропускаем '*'
                    in_multi_comment = true;
                    continue;
                }
                _ => {}
            }
        }
        
        // Обрабатываем инструкции
        match ch {
            ';' => {
                // Конец инструкции
                if !current_instruction.trim().is_empty() {
                    match parse_instruction(&current_instruction, instruction_line) {
                        Ok(tokens) => {
                            all_tokens.push(tokens);
                        }
                        Err(e) => return Err((e, instruction_line)),
                    }
                }
                current_instruction.clear();
                instruction_line = current_line; // СЛЕДУЮЩАЯ инструкция начнётся с текущей строки
            }
            _ => {
                // Если инструкция пустая (только что начали), запоминаем строку
                if current_instruction.trim().is_empty() && !ch.is_whitespace() {
                    instruction_line = current_line;
                }
                current_instruction.push(ch);
            }
        }
    }
    
    // Последняя инструкция (если нет ';' в конце)
    if !current_instruction.trim().is_empty() {
        match parse_instruction(&current_instruction, instruction_line) {
            Ok(tokens) => {
                all_tokens.push(tokens);
            }
            Err(e) => return Err((e, instruction_line)),
        }
    }
    
    Ok(all_tokens)
}

fn parse_instruction(instruction: &str, line_num: i32) -> Result<Vec<RawToken>, String> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    
    for ch in instruction.chars() {
        if ch.is_whitespace() {
            if !buffer.is_empty() {
                tokens.push(parse_token(&buffer, &line_num)?);
                buffer.clear();
            }
        } else {
            buffer.push(ch);
        }
    }
    
    if !buffer.is_empty() {
        tokens.push(parse_token(&buffer, &line_num)?);
    }
    
    if tokens.len() > 4 {
        return Err(format!("Слишком много токенов: '{}'", instruction));
    }
    
    Ok(tokens)
}

// parse_token оставить как был, но он получает реальный line_num

fn parse_token(s: &str, line_n: &i32) -> Result<RawToken, String> {
    // Булевы: "T", "F"
    if s == "T" || s == "F" {
        return Ok(RawToken::Bool(s == "T", *line_n));
    }

    // Односимвольные ключевые слова: "X", "A", "N", "I", "G", "P", "E", "L", "S"
    if s.len() == 1 {
        let c = s.chars().next().unwrap();
        if matches!(c, 'X' | 'A' | 'O' | 'N' | 'I' | 'G' | 'P' | 'E' | 'L' | 'S' | 'U' | ';') {
            return Ok(RawToken::Keyword(c.to_string(), *line_n));
        }
    }
    
    
    if s.len() >= 2 { 
        // Метки: "P1", "P2", "P3" (буква + цифры)
        if s.contains('.') {
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() == 2 {
                return match parts[0] {
                    "P" => Ok(RawToken::LabelP(parts[1].to_string(), *line_n)),
                    "PD" => Ok(RawToken::LabelPD(parts[1].to_string(), *line_n)),
                    _ => Err(format!("   >>  ! не удальсь обработать указатель {}  ({})", s, line_n))                         
                }   
            }
        }
    }

        // Число: "123", "0", "10"
    if s.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_alphanumeric() || c == '_') {
        return Ok(RawToken::Number(s.to_string(), *line_n));
    }

    Err(format!("   >>  ! не получилось обработать слово: {s}  ({})", line_n))
}

















// fn remove_comments(string: String) -> String {
//     let mut new_string = "".to_string();
//     let mut flag_multi = false;
//     let mut flag_single = false;
//     let mut _flag = false;

//     let mut string = string.replace("\r\n", "\n");

//     string = string.replace("/*", " /* ").replace("*/", " */ ").replace("//", " // ").replace("\n", " \n ").replace(";", " ; ");

//     for i in string.split(' ') {
//         let mut flag = false;

//         if i == "//" {
//             // print!("удалён комментарий: ");
//             flag_single = true
//         }
//         if flag_single || i == "\n" {
//             flag = true;
//         }
        
//         if i == "\n" {
//             flag_single = false;
//             //println!()
//         }
        
//         if flag_single {
//             // print!("{}", i)
//         }
        

//         if i == "/*" {
//             flag_multi = true;
//             print!("  >>  замечено начало мульти строчного комментария: '{}' удаляется: ", i)
//         }

//         if flag_multi {
//             flag = true;
//         }

//         if i.contains("*/") {
//             println!(" '{}'", i);
//             flag_multi = false;
//             println!("  >>  замечен конец мульти строчного комментария: '{}'", i)
//         }
//         if flag_multi {
//             //print!(" {}", i)
//         }

//         if !flag {
//             new_string.push(' ');
//             new_string.push_str(i);
//             //println!("add: '{}'", i)
//         } else { 
//             //println!("not add: '{}'", i)
//         }
//     }
//     new_string
// }