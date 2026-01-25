//use std::{fmt::format, i32};
use crate::preprocessors::parentheses_process;

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
            if tokens.is_empty() {println!("\nТокенов нет\n")}
            //else {println!("\nТокены: {:?} \n", tokens);}
            Ok(tokens)
        }
        Err(e) => {
            Err(e)
        }
    }
}


pub fn is_range_token(s: &str) -> bool {
    s.contains("{") && s.ends_with("}")
}

pub fn parse_range_token(s: &str) -> Option<(String, i32, i32)> {
    if !is_range_token(s) {
        return None;
    }
    
    if let Some((base, range_part)) = s.split_once("{") {
        let range_part = range_part.strip_suffix('}')?;
        
        if !base.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') || base.is_empty() {
            return None;
        }
        
        if range_part.contains("..") {
            let parts: Vec<&str> = range_part.split("..").collect();
            if parts.len() == 2 {
                match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    (Ok(start), Ok(end)) => {
                        // end - ВКЛЮЧИТЕЛЬНЫЙ (inclusive)
                        // 0..7 = 0,1,2,3,4,5,6,7 (8 элементов)
                        // 8..1 = 8,7,6,5,4,3,2,1 (8 элементов)
                        
                        if start == end {
                            // Одно значение
                            return Some((base.to_string(), start, end));
                        }
                        
                        return Some((base.to_string(), start, end));
                    }
                    _ => return None,
                }
            }
        } else {
            match range_part.parse::<i32>() {
                Ok(num) => return Some((base.to_string(), num, num)),
                _ => return None,
            }
        }
    }
    
    None
}

fn tokenize(input: &str) -> Result<Vec<Vec<RawToken>>, (String, i32)> {
    let mut all_tokens = Vec::new();
    let mut current_line = 1;
    let mut in_single_comment = false;
    let mut in_multi_comment = false;
    let mut current_instruction = String::new();
    let mut instruction_line = 1;
    let mut in_parentheses = false;
    
    let mut in_parentheses_vec: Vec<char> = Vec::new();

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
                    let n = parentheses_process(&in_parentheses_vec);
                    
                    match parse_instruction(&current_instruction, instruction_line) {
                        Ok(tokens) => {
                            if n == 1 {
                                all_tokens.push(tokens);
                            } else { for _ in 0..n {
                                all_tokens.push(tokens.clone());
                            }} 
                            
                        }
                        Err(e) => return Err((e, instruction_line)),
                    }
                }
                current_instruction.clear();
                in_parentheses_vec.clear();
                instruction_line = current_line; // СЛЕДУЮЩАЯ инструкция начнётся с текущей строки
            }
            '(' => {in_parentheses = true}
            ')' => {in_parentheses = false}
            _ => {
                if in_parentheses {in_parentheses_vec.push(ch);}
                // Если инструкция пустая (только что начали), запоминаем строку
                else if current_instruction.trim().is_empty() && !ch.is_whitespace() {
                    instruction_line = current_line;
                }
                if !in_parentheses {current_instruction.push(ch);}
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


fn parse_token(s: &str, line_n: &i32) -> Result<RawToken, String> {
    // Булевы: "T", "F"
    if s == "T" || s == "F" {
        return Ok(RawToken::Bool(s == "T", *line_n));
    }

    // Односимвольные ключевые слова: "X", "A", "N", "I", "G", "P", "E", "L", "S"
    if s.len() == 1 {
        let c = s.chars().next().unwrap();
        if matches!(c, 'X' | 'A' | 'O' | 'N' | 'I' | 'G' | 'P' | 'E' | 'L' | 'S' | 'U') {
            return Ok(RawToken::Keyword(c.to_string(), *line_n));
        }
    }

    if s.len() == 2 {
        if matches!(s, "IN" | "IG" | "BP") {
            return Ok(RawToken::Keyword(s.to_string(), *line_n));
        }
    }

    if s.len() == 3 {
        if matches!(s, "INB" | "RET" | "LOG") {
            return Ok(RawToken::Keyword(s.to_string(), *line_n));
        }
    }

    if s.len() == 4 {
        if matches!(s, "INBC" | "CALL" | "FUNC" | "STEP" | "ELSE") {
            return Ok(RawToken::Keyword(s.to_string(), *line_n));
        }
    }
    
    if s.len() == 5 {
        if matches!(s, "DEBUG") {
            return  Ok(RawToken::Keyword(s.to_string(), *line_n));
        }
    }

    if s.len() == 6 {
        if s == "IMPORT" {
            return  Ok(RawToken::Keyword(s.to_string(), *line_n));
        }
    }

    if matches!(s, "DEBUG_ON" | "DEBUG_OFF") {
        return  Ok(RawToken::Keyword(s.to_string(), *line_n));
    }


    
    
    if s.len() >= 3 { 
        // Метки: P.10 P.test PD.10 PD.test
        if s.contains('.') {
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() == 2 {
                return match parts[0] {
                    "P" => Ok(RawToken::LabelP(parts[1].to_string(), *line_n)),
                    "PD" => Ok(RawToken::LabelPD(parts[1].to_string(), *line_n)),
                    _ => Err(format!("   >>  ! не удальсь обработать указатель {}  ({})\n\n", s, line_n))                         
                }   
            }
        }
    }

        // Числа: 10 test test_10
    if s.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_alphanumeric() || c == '_' || c == '{' || c == '.' || c == '}') {
        //print!("{s} ");
        return Ok(RawToken::Number(s.to_string(), *line_n));
    }

    Err(format!("   >>  ! не получилось обработать слово: {s}  ({})\n\n", line_n))
}