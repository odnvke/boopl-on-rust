use crate::tokens::Token;

pub fn to_bytecode(tokens: Vec<Vec<Token>>) -> Result<Vec<Vec<i32>>, String> {
    let mut bytecode = Vec::new();
    
    for token_line in tokens {
        let code = process_line(token_line)?; // ? автоматически распаковывает Result
        
        if !code.is_empty() {
            bytecode.push(code);
        }
    }
    
    Ok(bytecode)
}

fn process_line(tokens: Vec<Token>) -> Result<Vec<i32>, String> {
    match tokens.as_slice() {
        
        // E
        [Token::Keyword(s)] if s == "E" => Ok(vec![50]),

        // 10 T | 10 F
        [Token::Number(n), Token::Bool(b)] => {
            if *b { Ok(vec![101, *n]) }
            else { Ok(vec![100, *n]) }
        }

        // 10 10
        [Token::Number(n), Token::Number(n2)] => Ok(vec![150, *n, *n2]),

        // P.10 | P1
        [Token::LabelP(n)] => Ok(vec![200, *n]),

        // PD.10 | PD1
        [Token::LabelPD(n)] => Ok(vec![201, *n]),

        // G P.10 | G P1
        [Token::Keyword(s), Token::LabelP(n)] if s == "G" => Ok(vec![230, *n]),

        // G PD.10 | G PD1
        [Token::Keyword(s), Token::LabelPD(n)] if s == "G" => Ok(vec![231, *n]),

        // PD.10 P.10
        [Token::LabelPD(n), Token::LabelP(n2)] => Ok(vec![260, *n, *n2]),

        // PD.10 PD.10
        [Token::LabelPD(n), Token::LabelPD(n2)] => Ok(vec![261, *n, *n2]),

        // I 10
        [Token::Keyword(s), Token::Number(n)] if s == "I" => Ok(vec![300, *n]),

        // IC 10
        [Token::Keyword(s), Token::Number(n)] if s == "IC" => Ok(vec![301, *n]),

        // IG 10 P.10
        [Token::Keyword(s), Token::Number(n), Token::LabelP(n2)] if s == "IG" => Ok(vec![302, *n, *n2]),

        // IG 10 PD.10
        [Token::Keyword(s), Token::Number(n), Token::LabelPD(n2)] if s == "IG" => Ok(vec![303, *n, *n2]),

        // P T | P F
        [Token::Keyword(s), Token::Bool(b)] if s == "P" => {
            if *b { Ok(vec![400]) }
            else { Ok(vec![401]) }
        }
        
        // P 10
        [Token::Keyword(s), Token::Number(n)] if s == "P" => Ok(vec![402, *n]),
        
        // P N
        [Token::Keyword(s), Token::Keyword(s2)] if s == "P" && s2 == "N" => Ok(vec![403]),

        // P S
        [Token::Keyword(s), Token::Keyword(s2)] if s == "P" && s2 == "S" => Ok(vec![404]),

        // 10 N 10
        [Token::Number(n), Token::Keyword(s), Token::Number(n2)] if s == "N" => Ok(vec![500, *n, *n2]),

        // 10 O 10 10
        [Token::Number(n), Token::Keyword(s), Token::Number(n2), Token::Number(n3)] 
            if s == "O" => Ok(vec![550, *n, *n2, *n3]),
        
        // 10 A 10 10
        [Token::Number(n), Token::Keyword(s), Token::Number(n2), Token::Number(n3)] 
            if s == "A" => Ok(vec![551, *n, *n2, *n3]),

        // 10 X 10 10
        [Token::Number(n), Token::Keyword(s), Token::Number(n2), Token::Number(n3)] 
            if s == "X" => Ok(vec![552, *n, *n2, *n3]),

        // Пустая строка
        [] => Ok(vec![]),

        _ => Err(format!("Ошибка в обработке последовательности токенов {:?}", tokens)),
    }
}