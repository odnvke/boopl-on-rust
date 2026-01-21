use crate::tokens::RawToken;

pub fn else_processing(tokens: Vec<Vec<RawToken>>) -> Vec<Vec<RawToken>> {
    let mut result = Vec::new();
    let mut else_blocks: Vec<(usize, String, usize)> = Vec::new(); // (позиция G, метка, позиция для метки)
    let mut label_counter = 0;
    
    // Первый проход: находим все IF/E/ELSE
    let mut if_stack: Vec<usize> = Vec::new();
    let mut last_e_positions: Vec<usize> = Vec::new();
    
    for i in 0..tokens.len() {
        match tokens[i].as_slice() {
            [RawToken::Keyword(s, _), RawToken::Number(_, _)] if s == "I" => {
                if_stack.push(i);
            }
            [RawToken::Keyword(s, _)] if s == "E" => {
                last_e_positions.push(i);
                if_stack.pop();
            }
            _ => {}
        }
    }
    
    // Второй проход: находим ELSE и планируем вставку меток
    for i in 0..tokens.len() {
        let line = &tokens[i];
        
        // Если это E и следующий токен - ELSE
        if let [RawToken::Keyword(s, _l_n)] = line.as_slice() {
            if s == "E" && i < tokens.len() - 1 {
                if let [RawToken::Keyword(next_s, _)] = tokens[i + 1].as_slice() {
                    if next_s == "ELSE" {
                        let label = format!("__else_close_{}", label_counter);
                        
                        // Нужно найти позицию для метки (следующий E после ELSE блока)
                        let mut j = i + 2; // Пропускаем E и ELSE
                        let mut nested_count = 0;
                        
                        while j < tokens.len() {
                            match tokens[j].as_slice() {
                                [RawToken::Keyword(ss, _), RawToken::Number(_, _)] if ss == "I" => {
                                    nested_count += 1;
                                }
                                [RawToken::Keyword(ss, label_l_n)] if ss == "E" => {
                                    if nested_count == 0 {
                                        // Нашли нужный E
                                        else_blocks.push((i, label, j));
                                        break;
                                    }
                                    nested_count -= 1;
                                }
                                _ => {}
                            }
                            j += 1;
                        }
                        
                        label_counter += 1;
                    }
                }
            }
        }
    }
    
    // Третий проход: строим результат
    for i in 0..tokens.len() {
        let line = &tokens[i];
        
        // Пропускаем ELSE
        if matches!(line.as_slice(), [RawToken::Keyword(s, _)] if s == "ELSE") {
            continue;
        }
        
        // Проверяем, нужно ли вставить G перед этим E
        let mut inserted_g = false;
        for &(e_pos, ref label, _) in &else_blocks {
            if i == e_pos {
                if let [RawToken::Keyword(_, l_n)] = line.as_slice() {
                    result.push(vec![
                        RawToken::Keyword("G".to_string(), *l_n),
                        RawToken::LabelP(label.clone(), *l_n)
                    ]);
                    inserted_g = true;
                }
            }
        }
        
        // Проверяем, нужно ли вставить метку перед этим E
        let mut inserted_label = false;
        for &(_, ref label, label_pos) in &else_blocks {
            if i == label_pos {
                if let [RawToken::Keyword(_, l_n)] = line.as_slice() {
                    result.push(vec![
                        RawToken::LabelP(label.clone(), *l_n)
                    ]);
                    inserted_label = true;
                }
            }
        }
        
        // Добавляем исходную строку
        if !inserted_g && !inserted_label {
            result.push(line.clone());
        } else if inserted_g {
            // Если вставили G, нужно добавить и исходный E
            result.push(line.clone());
        }
    }
    
    result
}