// function_preprocessor.rs
use std::collections::{HashMap, HashSet};
use crate::tokens::RawToken;

// ==================== ТОКЕН-ПРЕПРОЦЕССОР ====================

pub fn expand_tokens(tokens: Vec<Vec<RawToken>>) -> Result<Vec<Vec<RawToken>>, String> {
    let mut expander = TokenFunctionExpander::new();
    expander.expand(tokens)
}

struct TokenFunctionInfo {
    body_tokens: Vec<Vec<RawToken>>,
    declared_at_line: i32,
    calls: HashSet<String>,
}

struct TokenFunctionExpander {
    functions: HashMap<String, TokenFunctionInfo>,
    call_counter: usize,
}

impl TokenFunctionExpander {
    fn new() -> Self {
        Self {
            functions: HashMap::new(),
            call_counter: 0,
        }
    }
    
    fn expand(&mut self, tokens: Vec<Vec<RawToken>>) -> Result<Vec<Vec<RawToken>>, String> {
        // 1. Собрать информацию о функциях
        self.collect_functions(&tokens)?;
        
        // 2. Проверить циклические зависимости
        if let Some(cycle) = self.find_cycles() {
            return Err(format!("Обнаружена циклическая зависимость: {}", cycle.join(" -> ")));
        }
        
        // 3. Расширить основной код
        let mut result = Vec::new();
        
        for line in tokens {
            if self.is_function_declaration(&line) {
                continue; // Пропускаем объявления функций
            }
            
            // Пропускаем RET в основном коде
            if let [RawToken::Keyword(k, _)] = line.as_slice() {
                if k == "RET" {
                    continue;
                }
            }
            
            if let Some(expanded) = self.expand_main_line(&line)? {
                for exp_line in expanded {
                    result.push(exp_line);
                }
            } else {
                result.push(line);
            }
        }
        
        // 4. Добавить тела функций
        if !self.functions.is_empty() {
            let bodies = self.create_function_bodies();
            result.extend(bodies);
        }
        
        Ok(result)
    }
    
fn collect_functions(&mut self, tokens: &[Vec<RawToken>]) -> Result<(), String> {
    let mut current_func: Option<(String, i32, Vec<Vec<RawToken>>)> = None;
    
    for line in tokens {
        // Проверяем начало новой функции
        if let [RawToken::Keyword(k, line_num), RawToken::Number(name, _), ..] = &line[..] {
            if k == "FUNC" {
                // Если уже есть активная функция, завершаем её
                if let Some((prev_name, prev_line_num, body)) = current_func.take() {
                    let calls = self.extract_calls_from_body(&body);
                    self.functions.insert(prev_name.clone(), TokenFunctionInfo {
                        body_tokens: body,
                        declared_at_line: prev_line_num,
                        calls,
                    });
                }
                
                if self.functions.contains_key(name) {
                    return Err(format!("Строка {}: Переопределение функции '{}'", line_num, name));
                }
                
                current_func = Some((name.clone(), *line_num, Vec::new()));
                continue;
            }
        }
        
        // Добавляем ВСЕ строки в тело текущей функции (включая RET!)
        if let Some((_, _, body)) = &mut current_func {
            body.push(line.clone());
        }
    }
    
    // Не забываем завершить последнюю функцию
    if let Some((name, line_num, body)) = current_func.take() {
        let calls = self.extract_calls_from_body(&body);
        self.functions.insert(name.clone(), TokenFunctionInfo {
            body_tokens: body,
            declared_at_line: line_num,
            calls,
        });
    }
    
    Ok(())
}
    
    fn create_function_bodies(&mut self) -> Vec<Vec<RawToken>> {
    let mut result = Vec::new();
    
    // Глобальная переменная для возврата
    result.push(vec![RawToken::LabelPD("__ret".to_string(), 0)]);
    result.push(vec![]); // Пустая строка
    
    // Создаем копию информации о функциях перед обработкой
    let funcs_to_process: Vec<(String, Vec<Vec<RawToken>>)> = self.functions
        .iter()
        .map(|(name, info)| (name.clone(), info.body_tokens.clone()))
        .collect();
    
    for (func_name, body_tokens) in funcs_to_process {
        // Метка начала функции
        result.push(vec![
            RawToken::LabelP(format!("__func_{}_body", func_name), 0),
        ]);
        
        // Сохраняем возвратный адрес
        result.push(vec![
            RawToken::LabelPD(format!("__local_ret_{}", func_name), 0),
            RawToken::LabelPD("__ret".to_string(), 0),
        ]);
        
        // Тело функции ВКЛЮЧАЯ все RET
        for line in &body_tokens {
            if let Some(expanded) = self.expand_function_line(line, &func_name) {
                for exp_line in expanded {
                    result.push(exp_line);
                }
            } else {
                result.push(line.clone());
            }
        }
        
        // Автоматический возврат в конце функции (на случай если нет явного RET)
        result.push(vec![
            RawToken::Keyword("G".to_string(), 0),
            RawToken::LabelPD(format!("__local_ret_{}", func_name), 0),
        ]);
        
        result.push(vec![]); // Пустая строка между функциями
    }
    
    result
}
    
    fn expand_main_line(&mut self, line: &[RawToken]) -> Result<Option<Vec<Vec<RawToken>>>, String> {
        if let [RawToken::Keyword(k, line_num), RawToken::Number(func_name, _), ..] = line {
            if k == "CALL" {
                if !self.functions.contains_key(func_name) {
                    return Err(format!("Строка {}: Функция '{}' не объявлена", line_num, func_name));
                }
                
                let call_id = self.call_counter;
                self.call_counter += 1;
                
                let mut result = Vec::new();
                
                // PD.__ret P.__ret_{call_id}
                result.push(vec![
                    RawToken::LabelPD("__ret".to_string(), *line_num),
                    RawToken::LabelP(format!("__ret_{}", call_id), *line_num),
                ]);
                
                // G P.__func_{func_name}_body
                result.push(vec![
                    RawToken::Keyword("G".to_string(), *line_num),
                    RawToken::LabelP(format!("__func_{}_body", func_name), *line_num),
                ]);
                
                // P.__ret_{call_id}
                result.push(vec![
                    RawToken::LabelP(format!("__ret_{}", call_id), *line_num),
                ]);
                
                return Ok(Some(result));
            }
        }
        
        Ok(None)
    }
    
fn expand_function_line(&mut self, line: &[RawToken], current_func: &str) -> Option<Vec<Vec<RawToken>>> {
    // ДОСТАТОЧНО: проверяем только первый токен для RET
    if let [RawToken::Keyword(k, line_num), ..] = line {
        match k.as_str() {
            "CALL" => {
                // Для CALL нужен второй токен
                if let [_, RawToken::Number(called_func, line_num), ..] = line {
                    if self.functions.contains_key(called_func) {
                        let call_id = self.call_counter;
                        self.call_counter += 1;
                        
                        let mut result = Vec::new();
                        
                        result.push(vec![
                            RawToken::LabelPD("__ret".to_string(), *line_num),
                            RawToken::LabelP(format!("__ret_{}_{}", current_func, call_id), *line_num),
                        ]);
                        
                        result.push(vec![
                            RawToken::Keyword("G".to_string(), *line_num),
                            RawToken::LabelP(format!("__func_{}_body", called_func), *line_num),
                        ]);
                        
                        result.push(vec![
                            RawToken::LabelP(format!("__ret_{}_{}", current_func, call_id), *line_num),
                        ]);
                        
                        return Some(result);
                    }
                }
            }
            "RET" => {
                // RET заменяем на G к локальному возврату
                return Some(vec![vec![
                    RawToken::Keyword("G".to_string(), *line_num),
                    RawToken::LabelPD(format!("__local_ret_{}", current_func), *line_num),
                ]]);
            }
            _ => {}
        }
    }
    None
}
    
    // Вспомогательные методы
    fn is_function_declaration(&self, line: &[RawToken]) -> bool {
        matches!(line.get(0), Some(RawToken::Keyword(k, _)) if k == "FUNC")
    }
    
    fn extract_calls_from_body(&self, body: &[Vec<RawToken>]) -> HashSet<String> {
        let mut calls = HashSet::new();
        for line in body {
            if let [RawToken::Keyword(k, _), RawToken::Number(func_name, _), ..] = line.as_slice() {
                if k == "CALL" {
                    calls.insert(func_name.clone());
                }
            }
        }
        calls
    }
    
    fn find_cycles(&self) -> Option<Vec<String>> {
        for func in self.functions.keys() {
            let mut visited = HashSet::new();
            let mut stack = Vec::new();
            
            if self.detect_cycle(func, &mut visited, &mut stack) {
                return Some(stack);
            }
        }
        None
    }
    
    fn detect_cycle(&self, current: &str, visited: &mut HashSet<String>, stack: &mut Vec<String>) -> bool {
        if visited.contains(current) {
            return true;
        }
        
        visited.insert(current.to_string());
        stack.push(current.to_string());
        
        if let Some(info) = self.functions.get(current) {
            for callee in &info.calls {
                if self.detect_cycle(callee, visited, stack) {
                    return true;
                }
            }
        }
        
        stack.pop();
        visited.remove(current);
        false
    }
}