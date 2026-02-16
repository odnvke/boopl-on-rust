// range_expander.rs
use crate::tokens::{RawToken, is_range_token, parse_range_token};
use crate::error::{BooplError, Result};

pub fn expand_ranges(tokens: Vec<Vec<RawToken>>) -> Result<Vec<Vec<RawToken>>> {
    let mut expanded_tokens = Vec::new();
    
    for line in tokens {
        let mut has_ranges = false;
        let mut ranges_info = Vec::new();
        
        for token in &line {
                if let RawToken::Number(s, l_num) = token {
                    if is_range_token(s) {
                        has_ranges = true;
                        match parse_range_token(s) {
                            Some((base, start, end)) => {
                                ranges_info.push((base, start, end, *l_num));
                            }
                            None => {
                                return Err(BooplError::new(
                                    format!("Некорректный range токен '{}'", s),
                                    *l_num
                                ));
                            }
                        }
                    }
                }
            }
        
        if !has_ranges {
            expanded_tokens.push(line);
            continue;
        }
        
        if ranges_info.len() > 1 {
            let first_len = (ranges_info[0].2 - ranges_info[0].1).abs() + 1;
            for (_base, start, end, l_num) in &ranges_info[1..] {
                let current_len = (end - start).abs() + 1;
                if current_len != first_len {
                    return Err(BooplError::new(
                        "Разные длины в range expansion",
                        *l_num
                    ));
                }
            }
        }
        
        // Количество итераций
        let max_iterations = if let Some((_, start, end, _)) = ranges_info.first() {
            ((end - start).abs() + 1) as usize  // +1 т.к. inclusive
        } else {
            1
        };
        
        // Генерируем
        for i in 0..max_iterations {
            let mut new_line = Vec::new();
            
            for token in &line {
                match token {
                    RawToken::Number(s, line_num) => {
                        if is_range_token(s) {
                            match parse_range_token(s) {
                                Some((base, start, end)) => {
                                    let actual_index = if start < end {
                                        start + i as i32
                                    } else {
                                        start - i as i32
                                    };
                                    new_line.push(RawToken::Number(
                                        format!("{}{}", base, actual_index), 
                                        *line_num
                                    ));
                                }
                                None => new_line.push(token.clone()),
                            }
                        } else {
                            new_line.push(token.clone());
                        }
                    }
                    _ => new_line.push(token.clone()),
                }
            }
            
            expanded_tokens.push(new_line);
        }
    }
    
    Ok(expanded_tokens)
}