use crate::tokens::RawToken;

pub fn else_processing(tokens: Vec<Vec<RawToken>>) -> Vec<Vec<RawToken>> {
    let mut name_ident: i32 = 0;

    let mut process_vec = tokens.clone();

    let name_else_p = "__else_close_{}".to_string();

    loop {
        let mut out: Vec<Vec<RawToken>> = Vec::new();

        let mut if_couter = 0;
        let mut lock_ = false;

        let mut _complite = true;

        for i in 0..process_vec.len() {
            let line = &process_vec[i];
            let next_line: Option<Vec<RawToken>> = if i < process_vec.len()-1 {
                Some(process_vec[i+1].clone())
            } else {
                None
            };

            //println!("{:?}", line);

            match line.as_slice() {
                [RawToken::Keyword(s, _), RawToken::Number(_, _)] if s == "I" => {
                    if_couter += 1;
                    out.push(line.clone());
                }

                [RawToken::Keyword(s, l_n2)] if s == "E" => {
                    if_couter -= 1;
                    if !lock_ {
                        match next_line {
                            Some(_next_line) => {
                                
                                match _next_line.as_slice() {
                                    [RawToken::Keyword(s, l_n)] if s == "ELSE" && if_couter == 0 => {
                                        println!("!!!!");
                                        
                                        // G P.__else_close_{} // переход к end close`у
                                        out.push(vec![RawToken::Keyword("G".to_string(), *l_n), 
                                                RawToken::LabelP(name_else_p.replace("{}", &name_ident.to_string()).to_string(), *l_n)]);

                                        lock_ = true;

                                        _complite = false;
                                    }
                                    
                                    _ => {}
                                }
                            }
                            None => {}
                        }

                        
                        // E; // конец if
                        out.push(line.clone());

                    } else {
                        if if_couter == -1 {
                            // P.__else_close_{} // метка end close`а
                            out.push(vec![ 
                                    RawToken::LabelP(name_else_p.replace("{}", &name_ident.to_string()).to_string(), *l_n2)]);
                            print!("$$$$");
                            lock_ = false;
                            name_ident += 1;
                        } else {
                            out.push(line.clone());
                        }
                    }
                }

                [RawToken::Keyword(s, l_n)] if s == "ELSE" => {}

                _ => {out.push(process_vec[i].clone());}
            }
        }
        process_vec = out.clone();
        if _complite {return out;}
    }
}