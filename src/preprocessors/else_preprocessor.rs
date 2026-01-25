use crate::tokens::RawToken;

pub fn else_processing(tokens: Vec<Vec<RawToken>>) -> Vec<Vec<RawToken>> {
    let mut label_counter = 0;
    process_tokens(&tokens, &mut label_counter)
}

fn process_tokens(tokens: &[Vec<RawToken>], label_id: &mut i32) -> Vec<Vec<RawToken>> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        if is_if_with_else(tokens, i) {
            let (block, new_i) = process_chain(tokens, i, label_id);
            result.extend(block);
            i = new_i;
        } else {
            result.push(tokens[i].clone());
            i += 1;
        }
    }
    result
}

fn is_if(line: &[RawToken]) -> bool {
    matches!(line, [RawToken::Keyword(s, _), RawToken::Number(_, _)] if s == "IF")
}

fn is_else_if(line: &[RawToken]) -> bool {
    matches!(line, [RawToken::Keyword(s1, _), RawToken::Keyword(s2, _), RawToken::Number(_, _)] 
        if s1 == "ELSE" && s2 == "IF")
}

fn is_else(line: &[RawToken]) -> bool {
    matches!(line, [RawToken::Keyword(s, _)] if s == "ELSE")
}

fn is_end(line: &[RawToken]) -> bool {
    matches!(line, [RawToken::Keyword(s, _)] if s == "E")
}

fn is_if_with_else(tokens: &[Vec<RawToken>], pos: usize) -> bool {
    if !is_if(&tokens[pos]) {
        return false;
    }
    let mut depth = 0;
    for i in (pos + 1)..tokens.len() {
        let line = &tokens[i];
        if is_if(line) {
            depth += 1;
        } else if is_end(line) {
            if depth == 0 { return false; }
            depth -= 1;
        } else if depth == 0 && (is_else_if(line) || is_else(line)) {
            return true;
        }
    }
    false
}

fn get_line_num(line: &[RawToken]) -> i32 {
    line.first().map(|t| match t {
        RawToken::Bool(_, n) => *n,
        RawToken::Keyword(_, n) => *n,
        RawToken::LabelP(_, n) => *n,
        RawToken::LabelPD(_, n) => *n,
        RawToken::Number(_, n) => *n,
    }).unwrap_or(0)
}

fn process_chain(tokens: &[Vec<RawToken>], start: usize, label_id: &mut i32) -> (Vec<Vec<RawToken>>, usize) {
    let current_end_label_id = *label_id;
    *label_id += 1;
    let end_label = format!("__close_else_{}", current_end_label_id);
    
    let mut else_labels: Vec<String> = Vec::new();
    let mut blocks: Vec<(Option<String>, usize, usize)> = Vec::new();
    let mut i = start;
    
    // Собираем все блоки
    loop {
        let line = &tokens[i];
        let cond = if is_if(line) {
            if let [RawToken::Keyword(_, _), RawToken::Number(c, _)] = &line[..] { 
                Some(c.clone()) 
            } else { 
                None 
            }
        } else if is_else_if(line) {
            if let [RawToken::Keyword(_, _), RawToken::Keyword(_, _), RawToken::Number(c, _)] = &line[..] { 
                Some(c.clone()) 
            } else { 
                None 
            }
        } else {
            None
        };
        
        i += 1;
        let body_start = i;
        let mut depth = 0;
        
        while i < tokens.len() {
            let line = &tokens[i];
            if is_if(line) {
                depth += 1;
            } else if is_end(line) {
                if depth == 0 {
                    blocks.push((cond, body_start, i));
                    i += 1;
                    break;
                }
                depth -= 1;
            } else if depth == 0 && (is_else_if(line) || is_else(line)) {
                blocks.push((cond, body_start, i));
                break;
            }
            i += 1;
        }
        
        if i >= tokens.len() || (!is_else_if(&tokens[i]) && !is_else(&tokens[i])) {
            break;
        }
        
        // Генерируем уникальную метку для следующего блока (ELSE IF)
        else_labels.push(format!("__else_{}_{}", current_end_label_id, else_labels.len()));
    }
    
    // Генерируем результат
    let mut result = Vec::new();
    for (idx, (cond, body_start, body_end)) in blocks.iter().enumerate() {
        let is_last = idx == blocks.len() - 1;
        let l_n = get_line_num(&tokens[*body_start]);
        
        // Метка для перехода (для всех кроме первого блока)
        if idx > 0 {
            let label = &else_labels[idx - 1];
            result.push(vec![RawToken::LabelP(label.clone(), l_n)]);
        }
        
        // Условие IF/ELSE IF
        if let Some(c) = cond {
            result.push(vec![
                RawToken::Keyword("IF".to_string(), l_n),
                RawToken::Number(c.clone(), l_n)
            ]);
        }
        
        // Рекурсивно обрабатываем тело блока для вложенных условий
        let body_slice = &tokens[*body_start..*body_end];
        let processed_body = process_tokens(body_slice, label_id);
        result.extend(processed_body);
        
        if !is_last {
            // G на конец конструкции
            result.push(vec![
                RawToken::Keyword("G".to_string(), l_n),
                RawToken::LabelP(end_label.clone(), l_n)
            ]);
            // E
            result.push(vec![RawToken::Keyword("E".to_string(), l_n)]);
        } else {
            // Последний блок - метка конца вместо E
            let end_l_n = if *body_end < tokens.len() { 
                get_line_num(&tokens[*body_end]) 
            } else { 
                l_n 
            };
            result.push(vec![RawToken::LabelP(end_label.clone(), end_l_n)]);
        }
    }
    
    (result, i)
}