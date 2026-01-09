use crate::{name_map::{IdentNameMap}, tokens::Token};

pub fn to_bytecode(tokens: Vec<Vec<Token>>, ident_name_map: &IdentNameMap) -> Result<Vec<(Vec<i32>, i32)>, String> {
    let mut bytecode = Vec::new();
    
    for token_line in tokens {
        let (code, l_n ) = process_line(token_line, ident_name_map)?;
        
        if !code.is_empty() {
            bytecode.push((code, l_n));
        }
    }
    
    Ok(bytecode)
}

fn process_line(tokens: Vec<Token>, ident_name_map: &IdentNameMap) -> Result<(Vec<i32>, i32), String> {
    match tokens.as_slice() {
        
        // E
        [Token::Keyword(s, l_n)] if s == "E" => Ok((vec![50], *l_n)),

        // 10 T | 10 F
        [Token::Number(n, l_n), Token::Bool(b, l_n2)] => {
            if *b { Ok((vec![101, *n], *l_n)) }
            else { Ok((vec![100, *n], *l_n)) }
        }

        // 10 10
        [Token::Number(n, l_n), Token::Number(n2, l_n2)] => Ok((vec![150, *n, *n2], *l_n)),

        // P.10 | P1
        [Token::LabelP(n, l_n)] => Ok((vec![200, *n], *l_n)),

        // PD.10 | PD1
        [Token::LabelPD(n, l_n)] => Ok((vec![201, *n], *l_n)),

        // G P.10 | G P1
        [Token::Keyword(s, l_n), Token::LabelP(n, l_n2)] if s == "G" => Ok((vec![230, *n], *l_n)),

        // G PD.10 | G PD1
        [Token::Keyword(s, l_n), Token::LabelPD(n, l_n2)] if s == "G" => Ok((vec![231, *n], *l_n)),

        // PD.10 P.10
        [Token::LabelPD(n, l_n), Token::LabelP(n2, l_n2)] => Ok((vec![260, *n, *n2], *l_n)),

        // PD.10 PD.10
        [Token::LabelPD(n, l_n), Token::LabelPD(n2, l_n2)] => Ok((vec![261, *n, *n2], *l_n)),

        // I 10
        [Token::Keyword(s, l_n), Token::Number(n, l_n2)] if s == "I" => Ok((vec![300, *n], *l_n)),

        // IG 10 P.10
        [Token::Keyword(s, l_n), Token::Number(n, l_n2), Token::LabelP(n2, l_n3)] if s == "IG" => Ok((vec![302, *n, *n2], *l_n)),

        // IG 10 PD.10
        [Token::Keyword(s, l_n), Token::Number(n, l_n2), Token::LabelPD(n2, l_n3)] if s == "IG" => Ok((vec![303, *n, *n2], *l_n)),

        // P T | P F
        [Token::Keyword(s, l_n), Token::Bool(b, l_n2)] if s == "P" => {
            if *b { Ok((vec![400], *l_n)) }
            else { Ok((vec![401], *l_n)) }
        }
        
        // P 10
        [Token::Keyword(s, l_n), Token::Number(n, l_n2)] if s == "P" => Ok((vec![402, *n], *l_n)),
        
        // P N
        [Token::Keyword(s, l_n), Token::Keyword(s2, l_n2)] if s == "P" && s2 == "N" => Ok((vec![403], *l_n)),

        // P S
        [Token::Keyword(s, l_n), Token::Keyword(s2, l_n2)] if s == "P" && s2 == "S" => Ok((vec![404], *l_n)),

        // P S
        [Token::Keyword(s, l_n), Token::Keyword(s2, l_n2), Token::Number(n, l_n3s)] if s == "P" && s2 == "U" => Ok((vec![405, *n], *l_n)),

        // 10 N 10
        [Token::Number(n, l_n), Token::Keyword(s, l_n2), Token::Number(n2, l_n3)] if s == "N" => Ok((vec![500, *n, *n2], *l_n)),

        // 10 O 10 10
        [Token::Number(n, l_n), Token::Keyword(s, l_n2), Token::Number(n2, l_n3), Token::Number(n3, l_n4)] 
            if s == "O" => Ok((vec![550, *n, *n2, *n3], *l_n)),
        
        // 10 A 10 10
        [Token::Number(n, l_n), Token::Keyword(s, l_n2), Token::Number(n2, l_n3), Token::Number(n3, l_n4)] 
            if s == "A" => Ok((vec![551, *n, *n2, *n3], *l_n)),

        // 10 X 10 10
        [Token::Number(n, l_n), Token::Keyword(s, l_n2), Token::Number(n2, l_n3), Token::Number(n3, l_n4)] 
            if s == "X" => Ok((vec![552, *n, *n2, *n3], *l_n)),

        // Пустая строка
        [] => Ok((vec![], -1)),

        _ => {
            let mut line: Vec<String> = Vec::new();
            let mut line_n = -1;
            for i in &tokens {
                match i {
                    Token::Number(n, l_n) => {
                        line_n = *l_n; 
                        line.push(format!("{}", ident_name_map.get_name(*n)));
                    }
                    Token::LabelP(n, l_n) => {
                        line_n = *l_n;
                        line.push(format!("P.{}", ident_name_map.get_name(*n)));
                    }
                    Token::LabelPD(n, l_n) => {
                        line_n = *l_n;
                        line.push(format!("PD.{}", ident_name_map.get_name(*n)));
                    }
                    Token::Bool(b, l_n) => {line.push(format!("{}", if *b {"T"} else {"F"})); line_n = *l_n;}
                    Token::Keyword(b, l_n) => {line.push(format!("{}", *b)); line_n = *l_n;}
                }
            }
            Err(format!("\n   >>  ! Ошибка в обработке последовательности токенов {:?}  ({})\n", line.join(" "), line_n))
        }
    }
}