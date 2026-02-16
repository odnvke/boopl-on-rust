use crate::error::{BooplError, Result};

pub fn parentheses_process(str: &Vec<char>, line_n: i32) -> Result<i32> {
    let s: String = str.iter().collect();
    let cleaned: String = s.chars().filter(|ch| *ch != '_').collect();

    if cleaned.is_empty() {
        // 1 исли скобки пустые
        return Ok(1);
    }

    if cleaned.chars().all(|ch| ch.is_ascii_alphanumeric()) {
        match cleaned.parse::<i32>() {
            
            // возврощяем цифру
            Ok(num) => Ok(num),
            
            Err(e) => Err(BooplError::new(
                format!("Ошибка парсинга '{}' в число: {}", s, e), 
                line_n
            )),
        }
    } else {
        Err(BooplError::new(
            format!("'{}' содержит недопустимые символы", s),
            line_n
        ))
    }
}