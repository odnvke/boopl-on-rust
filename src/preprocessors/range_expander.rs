// range_expander.rs
use crate::tokens::{RawToken, is_range_token, parse_range_token};

pub fn expand_ranges(tokens: Vec<Vec<RawToken>>) -> Vec<Vec<RawToken>> {
    let mut expanded_tokens = Vec::new();
    
    for line in tokens {
        // Проверяем, есть ли в строке токены с диапазонами
        let mut has_ranges = false;
        let mut ranges_info = Vec::new();
        let mut line_num ;
        
        for token in &line {
            if let RawToken::Number(s, l_num) = token {
                line_num = *l_num;
                if is_range_token(s) {
                    has_ranges = true;
                    match parse_range_token(s) {
                        Some((base, start, end)) => {
                            // Проверка корректности диапазона
                            // start уже не может быть > end, так как parse_range_token проверила
                            // Но оставим для безопасности
                            if start > end {
                                eprintln!("Ошибка: неверный диапазон в '{}' (строка {}): начало {} > конца {}", 
                                    s, line_num, start, end);
                                std::process::exit(1);
                            }
                            ranges_info.push((base, start, end, *l_num));
                        }
                        None => {
                            eprintln!("Ошибка: некорректный формат диапазона в '{}' (строка {})", 
                                s, line_num);
                            eprintln!("Ожидается формат: name_{{start..end}} или name_{{num}}");
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
        
        if !has_ranges {
            // Нет диапазонов - добавляем строку как есть
            expanded_tokens.push(line);
            continue;
        }
        
        // Проверяем, что все диапазоны имеют одинаковую длину
        if ranges_info.len() > 1 {
            let first_len = (ranges_info[0].2 - ranges_info[0].1 + 1) as usize;
            for (base, start, end, l_num) in &ranges_info[1..] {
                let current_len = (end - start + 1) as usize;
                if current_len != first_len {
                    eprintln!("Ошибка: разные длины диапазонов в строке {}", l_num);
                    eprintln!("  '{}' имеет {} элементов ({}..{})", 
                        base, current_len, start, end);
                    eprintln!("  а должно быть {} элементов, как у первого диапазона", first_len);
                    std::process::exit(1);
                }
            }
        }
        
        // Определяем количество итераций
        let max_iterations = if let Some((_, start, end, _)) = ranges_info.first() {
            (end - start + 1) as usize
        } else {
            1
        };
        
        // Генерируем строки для каждого индекса
        for i in 0..max_iterations {
            let mut new_line = Vec::new();
            let current_index = i as i32;
            
            for token in &line {
                match token {
                    RawToken::Number(s, line_num) => {
                        if is_range_token(s) {
                            match parse_range_token(s) {
                                Some((base, start, _end)) => {
                                    let actual_index = start + current_index;
                                    // Теперь это всегда true, так как длины одинаковы
                                    let new_name = format!("{}{}", base, actual_index);
                                    new_line.push(RawToken::Number(new_name, *line_num));
                                }
                                None => {
                                    new_line.push(token.clone());
                                }
                            }
                        } else {
                            // Обычное число/идентификатор
                            new_line.push(token.clone());
                        }
                    }
                    _ => {
                        // Все остальные токены добавляем как есть
                        new_line.push(token.clone());
                    }
                }
            }
            
            if !new_line.is_empty() {
                expanded_tokens.push(new_line);
            }
        }
    }
    
    expanded_tokens
}