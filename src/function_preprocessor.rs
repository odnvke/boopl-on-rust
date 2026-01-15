// function_preprocessor.rs
use std::collections::{HashMap, HashSet};

pub fn expand(source: &str) -> String {
    let mut expander = FunctionExpander::new();
    expander.expand(source)
}

struct FunctionInfo {
    original_body: String,
    processed_body: String,
    declared_at_line: usize,
    calls: HashSet<String>, // Какие функции вызывает эта функция
}

struct FunctionExpander {
    functions: HashMap<String, FunctionInfo>,
    all_calls: Vec<(String, usize)>, // (имя_функции, строка_вызова)
    next_id: usize,
}

impl FunctionExpander {
    fn new() -> Self {
        Self {
            functions: HashMap::new(),
            all_calls: Vec::new(),
            next_id: 0,
        }
    }
    
    fn expand(&mut self, source: &str) -> String {
        // Сбрасываем next_id
        self.next_id = 0;
        
        // Валидируем исходный код (требуем ; везде)
        match self.validate(source) {
            Ok(_) => {}, // всё OK
            Err(e) => {
                eprintln!("! препроцессор функций:\n\n{}\n\n", e);
                std::process::exit(1);
            }
        }

        // Обрабатываем тела функций (преобразуем CALL)
        let mut processed_bodies = Vec::new();
        for (func_name, info) in &self.functions {
            let processed_body = self.process_function_body(&info.original_body, func_name);
            processed_bodies.push((func_name.clone(), processed_body));
        }
        
        // Обновляем processed_body в функциях
        for (func_name, processed_body) in processed_bodies {
            if let Some(info) = self.functions.get_mut(&func_name) {
                info.processed_body = processed_body;
            }
        }

        // Теперь обрабатываем для расширения
        let lines: Vec<&str> = source.lines().collect();
        let mut result_lines = Vec::new();
        let mut i = 0;
        
        // Собираем основной код (все, что не является функциями)
        while i < lines.len() {
            let line = lines[i].trim();
            
            if line.starts_with("FUNC") {
                // Пропускаем всю функцию
                i += 1;
                while i < lines.len() {
                    let body_line = lines[i].trim();
                    if body_line == "RET;" {
                        break;
                    }
                    i += 1;
                }
                i += 1; // Пропускаем RET;
                continue;
            }
            
            result_lines.push(lines[i]);
            i += 1;
        }
        
        // 2. Обрабатываем оставшиеся строки
        let mut final_result = String::new();
        
        // Глобальные переменные
        final_result.push_str("PD.__ret;\n\n");
        
        for line in result_lines {
            let trimmed = line.trim();
            let indent = Self::get_indent(line);
            
            // Пропускаем пустые строки
            if trimmed.is_empty() {
                final_result.push_str("\n");
                continue;
            }
            
            // Пропускаем комментарии
            if trimmed.starts_with("//") {
                final_result.push_str(line);
                final_result.push_str("\n");
                continue;
            }
            
            // Обработка CALL
            if trimmed.starts_with("CALL ") {
                let rest = trimmed[5..].trim();
                let func_name = rest.split_whitespace().next().unwrap_or("");
                let func_name = func_name.trim_end_matches(';');
                
                if self.functions.contains_key(func_name) {
                    let call_id = self.next_id;
                    self.next_id += 1;
                    
                    final_result.push_str(&format!("{}// CALL {}\n", indent, func_name));
                    final_result.push_str(&format!("{}PD.__ret P.__ret_{};\n", indent, call_id));
                    final_result.push_str(&format!("{}G P.__func_{}_body;\n", indent, func_name));
                    final_result.push_str(&format!("{}P.__ret_{};\n", indent, call_id));
                } else {
                    // Функция не найдена
                    final_result.push_str(line);
                    final_result.push_str("\n");
                }
            }
            // Обработка E
            else if trimmed == "E;" {
                final_result.push_str(&format!("{}E;\n", indent));
            }
            // Всё остальное
            else {
                final_result.push_str(line);
                final_result.push_str("\n");
            }
        }
        
        // 3. Добавляем тела функций
        if !self.functions.is_empty() {
            final_result.push_str("\n// === Тела функций ===\n");
            
            for (func_name, info) in &self.functions {
                final_result.push_str(&format!("\nP.__func_{}_body;\n", func_name));
                final_result.push_str(&format!("  PD.__local_ret_{} PD.__ret;\n", func_name));
                final_result.push_str(&info.processed_body);
                final_result.push_str(&format!("  G PD.__local_ret_{};\n", func_name));
            }
        }
        
        final_result
    }
    
    fn process_function_body(&self, body: &str, current_func_name: &str) -> String {
        let lines: Vec<&str> = body.lines().collect();
        let mut result = String::new();
        let mut local_next_id = 0;
        
        for line in lines {
            let trimmed = line.trim();
            
            // Пропускаем пустые строки и комментарии
            if trimmed.is_empty() || trimmed.starts_with("//") {
                result.push_str(line);
                result.push_str("\n");
                continue;
            }
            
            let indent = Self::get_indent(line);
            
            if trimmed.starts_with("CALL ") {
                let rest = trimmed[5..].trim();
                let called_func = rest.split_whitespace().next().unwrap_or("");
                let called_func = called_func.trim_end_matches(';');
                
                // Проверяем, объявлена ли вызываемая функция
                if self.functions.contains_key(called_func) {
                    result.push_str(&format!("{}// CALL {}\n", indent, called_func));
                    result.push_str(&format!("{}PD.__ret P.__ret_{}_{};\n", 
                        indent, current_func_name, local_next_id));
                    result.push_str(&format!("{}G P.__func_{}_body;\n",
                        indent, called_func));
                    result.push_str(&format!("{}P.__ret_{}_{};\n", 
                        indent, current_func_name, local_next_id));
                    
                    local_next_id += 1;
                } else {
                    // Если функция не найдена, оставляем как есть
                    result.push_str(line);
                    result.push_str("\n");
                }
            } else {
                result.push_str(line);
                result.push_str("\n");
            }
        }
        
        result
    }
    
fn validate(&mut self, source: &str) -> Result<(), String> {
    let lines: Vec<&str> = source.lines().collect();
    
    // 1. ПЕРВЫЙ ПРОХОД: собираем ВСЕ объявления функций
    let mut i = 0;
    let mut function_declarations: Vec<(String, usize, usize, usize)> = Vec::new(); // (имя, строка_начала, строка_RET, уровень_вложенности)
    let mut current_func_stack: Vec<(String, usize)> = Vec::new(); // Стек функций
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        if line.starts_with("FUNC") {
            let func_name = Self::extract_func_name(line);
            let declared_at_line = i + 1;
            
            // Проверка переопределения
            if self.functions.contains_key(&func_name) {
                let first_line = self.functions[&func_name].declared_at_line;
                return Err(format!(
                    "Переопределение функции '{}'\nПервое объявление: строка {}\nПовторное: строка {}",
                    func_name, first_line, declared_at_line
                ));
            }
            
            // Запоминаем объявление
            function_declarations.push((func_name.clone(), declared_at_line, 0, current_func_stack.len()));
            
            // Добавляем в стек
            current_func_stack.push((func_name.clone(), declared_at_line));
            
            // Ищем конец функции (RET;)
            let mut j = i + 1;
            let mut found_ret = false;
            
            while j < lines.len() {
                let body_line = lines[j].trim();
                
                if body_line == "RET;" {
                    found_ret = true;
                    // Обновляем информацию о конце функции
                    if let Some(last) = function_declarations.last_mut() {
                        last.2 = j + 1;
                    }
                    current_func_stack.pop();
                    break;
                }
                
                // Если встретили новую функцию внутри - ошибка
                if body_line.starts_with("FUNC") {
                    let nested_func = Self::extract_func_name(body_line);
                    return Err(format!(
                        "Строка {}: Функция '{}' объявлена внутри функции '{}' (вложенные функции не разрешены)",
                        j + 1, nested_func, func_name
                    ));
                }
                
                j += 1;
            }
            
            if !found_ret {
                return Err(format!(
                    "Функция '{}' (строка {}) не завершена RET;",
                    func_name, declared_at_line
                ));
            }
            
            i = j + 1; // Переходим к строке после RET;
            continue;
        }
        
        i += 1;
    }
    
    // 2. СОЗДАЕМ функции в HashMap
    for (func_name, start_line, _ret_line, _nesting_level) in &function_declarations {
        self.functions.insert(func_name.clone(), FunctionInfo {
            original_body: String::new(),
            processed_body: String::new(),
            declared_at_line: start_line.clone(),
            calls: HashSet::new(),
        });
    }
    
    // 3. ВТОРОЙ ПРОХОД: собираем тела функций и вызовы
    self.all_calls.clear();
    
    for (func_name, start_line, ret_line, _) in function_declarations.iter() {
        let start_idx = start_line - 1;
        let ret_idx = ret_line - 1;
        
        if let Some(func_info) = self.functions.get_mut(func_name) {
            let mut original_body = String::new();
            let mut calls_in_func = HashSet::new();
            
            // Собираем тело функции
            for line_idx in (start_idx + 1)..ret_idx {
                let line = lines[line_idx];
                original_body.push_str(line);
                original_body.push_str("\n");
                
                // Проверяем синтаксис в теле функции
                let trimmed = line.trim();
                if !trimmed.is_empty() && 
                   !trimmed.starts_with("//") &&
                   !trimmed.ends_with(';') &&
                   !trimmed.ends_with(':') {
                    return Err(format!(
                        "Строка {}: Инструкция должна заканчиваться точкой с запятой: '{}'",
                        line_idx + 1, trimmed
                    ));
                }
                
                // Собираем CALL внутри функции
                if trimmed.starts_with("CALL ") {
                    let called = Self::extract_called_func(trimmed);
                    if !called.is_empty() {
                        calls_in_func.insert(called.clone());
                        self.all_calls.push((called.clone(), line_idx + 1));
                    }
                }
            }
            
            func_info.original_body = original_body;
            func_info.calls = calls_in_func;
        }
    }
    
    // 4. Собираем CALL вне функций (в основном коде)
    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Пропускаем пустые строки, комментарии
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        
        // Пропускаем строки внутри функций
        let mut is_inside_function = false;
        for (_, start_line, ret_line, _) in &function_declarations {
            let start_idx = *start_line - 1;
            let ret_idx = *ret_line - 1;
            if line_idx > start_idx && line_idx < ret_idx {
                is_inside_function = true;
                break;
            }
        }
        
        if is_inside_function {
            continue;
        }
        
        // Проверяем синтаксис в основном коде
        if !trimmed.starts_with("FUNC") && 
           trimmed != "RET;" &&
           !trimmed.ends_with(';') && 
           !trimmed.ends_with(':') {
            return Err(format!(
                "Строка {}: Инструкция должна заканчиваться точкой с запятой: '{}'",
                line_idx + 1, trimmed
            ));
        }
        
        // Сбор CALL вне функций
        if trimmed.starts_with("CALL ") {
            let called = Self::extract_called_func(trimmed);
            if !called.is_empty() {
                self.all_calls.push((called, line_idx + 1));
            }
        }
    }
    
    // 5. Проверка необъявленных функций
    let mut undefined_calls = Vec::new();
    for (called_func, line_num) in &self.all_calls {
        if !self.functions.contains_key(called_func) {
            undefined_calls.push((called_func.clone(), *line_num));
        }
    }
    
    if !undefined_calls.is_empty() {
        let mut error_msg = String::from("Вызовы необъявленных функций:\n");
        for (func, line) in undefined_calls {
            error_msg.push_str(&format!("  - '{}' в строке {}\n", func, line));
        }
        return Err(error_msg);
    }
    
    // 6. Проверка циклических зависимостей
    if let Some(cycle) = self.find_cycles() {
        return Err(format!(
            "Обнаружена циклическая зависимость:\n  {}",
            cycle.join(" -> ")
        ));
    }
    
    Ok(())
}
    
    fn find_cycles(&self) -> Option<Vec<String>> {
        let _visited: HashSet<String> = HashSet::new();
        
        for func_name in self.functions.keys() {
            let mut visiting: HashSet<String> = HashSet::new();
            let mut path: Vec<String> = Vec::new();
            
            if let Some(cycle) = self.detect_cycle(func_name, &mut visiting, &mut path) {
                return Some(cycle);
            }
        }
        
        None
    }

    fn detect_cycle(&self, current: &str, visiting: &mut HashSet<String>, 
                   path: &mut Vec<String>) -> Option<Vec<String>> {
        if visiting.contains(current) {
            // Нашли цикл
            let start = path.iter().position(|x| x == current).unwrap();
            let cycle = path[start..].to_vec();
            return Some(cycle);
        }
        
        visiting.insert(current.to_string());
        path.push(current.to_string());
        
        if let Some(func_info) = self.functions.get(current) {
            for callee in &func_info.calls {
                if let Some(cycle) = self.detect_cycle(callee, visiting, path) {
                    return Some(cycle);
                }
            }
        }
        
        path.pop();
        visiting.remove(current);
        None
    }
    
    fn extract_func_name(line: &str) -> String {
        let line = line.trim();
        let line = line.trim_start_matches("FUNC").trim_start();
        
        // Убираем комментарии
        let line = match line.find("//") {
            Some(pos) => &line[..pos],
            None => line,
        };
        
        let end = line.find(|c: char| c == ' ' || c == '(' || c == ';')
            .unwrap_or(line.len());
        
        let func_name = line[..end].trim();
        
        if func_name.is_empty() {
            eprintln!("Предупреждение: функция без имени");
        }
        
        func_name.to_string()
    }
    
    fn extract_called_func(line: &str) -> String {
        let line = line.trim_start_matches("CALL ").trim();
        // Убираем комментарии после вызова
        let line = match line.find("//") {
            Some(pos) => &line[..pos],
            None => line,
        };
        
        // Ищем конец имени функции
        let end = line.find(|c: char| c == ' ' || c == ';')
            .unwrap_or(line.len());
        
        let func_name = line[..end].trim();
        
        // Проверяем, не пустое ли имя
        if func_name.is_empty() {
            eprintln!("Предупреждение: пустой вызов CALL в строке");
        }
        
        func_name.to_string()
    }
    
    fn get_indent(line: &str) -> String {
        line.chars()
            .take_while(|c| c.is_whitespace())
            .collect()
    }
}