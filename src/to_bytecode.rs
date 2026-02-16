use crate::{name_map::IdentNameMap, tokens::Token};

pub fn to_bytecode(tokens: Vec<Vec<Token>>, ident_name_map: &IdentNameMap) -> Result<Vec<(Vec<i32>, i32)>, String> {
    let mut bytecode = Vec::new();
    bytecode.push((vec![0], 0));
    for token_line in tokens {
        let (code, line_num) = process_line(token_line, ident_name_map)?;
        
        if !code.is_empty() {
            bytecode.push((code, line_num));
        }
    }
    Ok(bytecode)
}

fn process_line(tokens: Vec<Token>, ident_name_map: &IdentNameMap) -> Result<(Vec<i32>, i32), String> {
    match tokens.as_slice() {
        
        // E
        [Token::Keyword(s, line_num)] if s == "E" => Ok((vec![50], *line_num)),

        // 10 T | 10 F
        [Token::Number(n, line_num), Token::Bool(b, _)] => {
            if *b { Ok((vec![101, *n], *line_num)) }
            else { Ok((vec![100, *n], *line_num)) }
        }

        // 10 10
        [Token::Number(n, line_num), Token::Number(n2, _)] => Ok((vec![150, *n, *n2], *line_num)),

        // P.10 | P1
        [Token::LabelP(n, line_num)] => Ok((vec![200, *n], *line_num)),

        // PD.10 | PD1
        [Token::LabelPD(n, line_num)] => Ok((vec![201, *n], *line_num)),

        // G P.10 | G P1
        [Token::Keyword(s, line_num), Token::LabelP(n, _)] if s == "G" => Ok((vec![230, *n], *line_num)),

        // G PD.10 | G PD1
        [Token::Keyword(s, line_num), Token::LabelPD(n, _)] if s == "G" => Ok((vec![231, *n], *line_num)),

        // PD.10 P.10
        [Token::LabelPD(n, line_num), Token::LabelP(n2, _)] => Ok((vec![260, *n, *n2], *line_num)),

        // PD.10 PD.10
        [Token::LabelPD(n, line_num), Token::LabelPD(n2, _)] => Ok((vec![261, *n, *n2], *line_num)),

        // IF 10
        [Token::Keyword(s, line_num), Token::Number(n, _)] if s == "IF" => Ok((vec![300, *n], *line_num)),

        // IFG 10 P.10
        [Token::Keyword(s, line_num), Token::Number(n, _), Token::LabelP(n2, _)] if s == "IFG" => Ok((vec![302, *n, *n2], *line_num)),

        // IFG 10 PD.10
        [Token::Keyword(s, line_num), Token::Number(n, _), Token::LabelPD(n2, _)] if s == "IFG" => Ok((vec![303, *n, *n2], *line_num)),

        // P T | P F
        [Token::Keyword(s, line_num), Token::Bool(b, _)] if s == "P" => {
            if *b { Ok((vec![400], *line_num)) }
            else { Ok((vec![401], *line_num)) }
        }
        
        // P 10
        [Token::Keyword(s, line_num), Token::Number(n, _)] if s == "P" => Ok((vec![402, *n], *line_num)),
        
        // P N
        [Token::Keyword(s, line_num), Token::Keyword(s2, _)] if s == "P" && s2 == "N" => Ok((vec![403], *line_num)),

        // P S
        [Token::Keyword(s, line_num), Token::Keyword(s2, _)] if s == "P" && s2 == "S" => Ok((vec![404], *line_num)),

        // P U 10
        [Token::Keyword(s, line_num), Token::Keyword(s2, _), Token::Number(n, _)] if s == "P" && s2 == "U" => Ok((vec![405, *n], *line_num)),

        // 10 N 10
        [Token::Number(n, line_num), Token::Keyword(s, _), Token::Number(n2, _)] if s == "N" => Ok((vec![500, *n, *n2], *line_num)),

        // 10 O 10 10
        [Token::Number(n, line_num), Token::Keyword(s, _), Token::Number(n2, _), Token::Number(n3, _)] 
            if s == "O" => Ok((vec![550, *n, *n2, *n3], *line_num)),
        
        // 10 A 10 10
        [Token::Number(n, line_num), Token::Keyword(s, _), Token::Number(n2, _), Token::Number(n3, _)] 
            if s == "A" => Ok((vec![551, *n, *n2, *n3], *line_num)),

        // 10 X 10 10
        [Token::Number(n, line_num), Token::Keyword(s, _), Token::Number(n2, _), Token::Number(n3, _)] 
            if s == "X" => Ok((vec![552, *n, *n2, *n3], *line_num)),

        // IN
        [Token::Keyword(s, line_num)] if s == "IN" => Ok((vec![600], *line_num)),

        // IN U
        [Token::Keyword(s, line_num), Token::Keyword(s2, _)] 
            if s == "IN" && s2 == "U" => Ok((vec![601], *line_num)),

        // INBC
        [Token::Keyword(s, line_num)] if s == "INBC" => Ok((vec![625], *line_num)),

        // 10 INBC
        [Token::Number(n, line_num), Token::Keyword(s, _)] if s == "INBC" => Ok((vec![650, *n], *line_num)),

        // 10 INB
        [Token::Number(n, line_num), Token::Keyword(s, _)] if s == "INB" => Ok((vec![675, *n], *line_num)),

        // 10 U INB
        [Token::Number(n, line_num), Token::Keyword(s, _), Token::Keyword(s2, _)]
              if s == "U" && s2 == "INB" => Ok((vec![676, *n], *line_num)),

        // DEBUG BP;
        [Token::Keyword(s, line_num), Token::Keyword(s2, _)] 
                if s == "DEBUG" && s2 == "BP" => Ok((vec![700], *line_num)),

        // DEBUG STOP;        
        [Token::Keyword(s, line_num), Token::Keyword(s2, _)] 
                if s == "DEBUG" && s2 == "STOP" => Ok((vec![730], *line_num)),

        // DEBUG_ON STEP;        
        [Token::Keyword(s, line_num), Token::Keyword(s2, _)] 
                if s == "DEBUG_ON" && s2 == "STEP" => Ok((vec![760, 0], *line_num)),
        
        // DEBUG_ON LOG;        
        [Token::Keyword(s, line_num), Token::Keyword(s2, _)] 
                if s == "DEBUG_ON" && s2 == "LOG" => Ok((vec![760, 1], *line_num)),

        // DEBUG_ON STEP LOG;        
        [Token::Keyword(s, line_num), Token::Keyword(s2, _), Token::Keyword(s3, _)] 
                if s == "DEBUG_ON" && ((s2 == "LOG" && s3 == "STEP") || (s2 == "STEP" && s3 == "LOG")) => Ok((vec![760, 2], *line_num)),
        
        // DEBUG_OFF STEP;        
        [Token::Keyword(s, line_num), Token::Keyword(s2, _)] 
                if s == "DEBUG_OFF" && s2 == "STEP" => Ok((vec![761, 0], *line_num)),
        
        // DEBUG_OFF LOG;        
        [Token::Keyword(s, line_num), Token::Keyword(s2, _)] 
                if s == "DEBUG_OFF" && s2 == "LOG" => Ok((vec![761, 1], *line_num)),

        // DEBUG_OFF STEP LOG;        
        [Token::Keyword(s, line_num), Token::Keyword(s2, _), Token::Keyword(s3, _)] 
                if s == "DEBUG_OFF" && ((s2 == "LOG" && s3 == "STEP") || (s2 == "STEP" && s3 == "LOG")) => Ok((vec![761, 2], *line_num)),
        
        // Пустая строка
        [] => Ok((vec![], -1)),

        _ => {
            let mut line: Vec<String> = Vec::new();
            let mut line_num = -1;
            for token in &tokens {
                match token {
                    Token::Number(n, ln) => {
                        line_num = *ln; 
                        line.push(format!("{}", ident_name_map.get_name_n(*n)));
                    }
                    Token::LabelP(n, ln) => {
                        line_num = *ln;
                        line.push(format!("P.{}", ident_name_map.get_name_p(*n)));
                    }
                    Token::LabelPD(n, ln) => {
                        line_num = *ln;
                        line.push(format!("PD.{}", ident_name_map.get_name_pd(*n)));
                    }
                    Token::Bool(b, ln) => {
                        line.push(format!("{}", if *b {"T"} else {"F"})); 
                        line_num = *ln;
                    }
                    Token::Keyword(k, ln) => {
                        line.push(format!("{}", *k)); 
                        line_num = *ln;
                    }
                }
            }
            Err(format!("\n   >>  ! Ошибка в обработке последовательности токенов {:?}  ({})\n\n", line.join(" "), line_num))
        }
    }
}