// function_preprocessor.rs
use crate::tokens::RawToken;

pub fn expand(tokens: Vec<Vec<RawToken>>) -> Vec<Vec<RawToken>> {
    let mut fp = Data::new();
    fp.expand(tokens)
}

struct Data {
    ident_index: i32,
    local_p_index: i32,
    is_need_label_p: bool,

    is_in_func: bool,
    func_name: Option<String>,
    return_point_name_t: String,
    local_rp_template: String,
    return_point_name: String,
    close_func_t: String,
}

impl Data {
    pub fn new() -> Self {
        Self {
            ident_index: 0,
            local_p_index: 0,
            is_need_label_p: false,

            is_in_func: false,
            func_name: None,
            return_point_name_t: "__func_return_{}".to_string(),
            local_rp_template: "__local_func_{}".to_string(),
            return_point_name: "__return".to_string(),
            close_func_t: "__func_close_{}".to_string(),
        }
    }

    fn expand(&mut self, tokens: Vec<Vec<RawToken>>) -> Vec<Vec<RawToken>> {
        let mut out_vec: Vec<Vec<RawToken>> = Vec::new();

        out_vec.push(vec![RawToken::LabelPD(self.return_point_name.to_string(), 0)]);

        for line in tokens {
            match line.as_slice() {

                // FUNC func_a;
                [RawToken::Keyword(s, l_n), RawToken::Number(n, l_n2)] if s == "FUNC" => {
                    if !self.is_in_func && self.func_name.is_some() {
                        eprintln!("функция '{}' без RET", self.func_name.as_ref().unwrap_or(&"не определена".to_string()));
                        std::process::exit(1);
                    }
                    self.func_name = Some(n.to_string());
                    self.is_in_func = true;
                    self.local_p_index += 1;
                    self.is_need_label_p = false;

                    out_vec.push(self._go_func_close(*l_n));
                    out_vec.push(vec![RawToken::LabelP(self.func_name.clone().unwrap_or("не определена".to_string()), *l_n)]);
                }

                // CALL func_a;
                [RawToken::Keyword(s, l_n), RawToken::Number(n, l_n2)] if s == "CALL" => {
                    if !self.is_need_label_p {
                        out_vec.push(self._add_write_pointer(*l_n));
                        self.is_need_label_p = true;
                    }
                    self.ident_index += 1;
                    
                    out_vec.push(self._add_pre_func_l(*l_n));
                    out_vec.push(self._add_goto(n, *l_n));
                    out_vec.push(self._add_post_func_l(*l_n));
                }

                [RawToken::Keyword(s, l_n), RawToken::Keyword(s2, l_n2)] if s == "RET" && s2 == "E" => {
                    out_vec.push(self._add_return_func(*l_n));
                    out_vec.push(self._func_close(*l_n));
                    self.is_in_func = true;
                    self.func_name = None;
                }

                [RawToken::Keyword(s, l_n)] if s == "RET" => {
                    if self.is_in_func {
                        out_vec.push(self._add_return_func(*l_n));
                    }
                }

                

                _ => {
                    for token in &line {
                        if let RawToken::Keyword(s, _) = token {
                            if matches!(s.as_str(), "CALL" | "FUNC" | "RET") {
                                eprintln!("\n ! препроцессор функций:\n\n   >>  ! неудалось обработать {:?}", line);
                                std::process::exit(1);
                            }
                        }
                    }
                    out_vec.push(line);
                }
            }
        }

        out_vec
    }

    // сохроняем перед вызовом внутри функции
    // PD.local PD.R 
    fn _add_write_pointer(&self, line_n: i32) -> Vec<RawToken> {
        vec![RawToken::LabelPD(self.return_point_name_t.replace("{}", &self.local_p_index.to_string()), line_n),
             RawToken::LabelPD(self.return_point_name.to_string(), line_n),]
    }   

    // перед вызовом внутри функции
    // PD.R P.local_new
    fn _add_pre_func_l(&self, line_n: i32) -> Vec<RawToken> {
        vec![RawToken::LabelPD(self.return_point_name.to_string(), line_n),
             RawToken::LabelP(self.local_rp_template.replace("{}", &self.ident_index.to_string()), line_n)]
    }

    // G P.func_a
    fn _add_goto(&self, func_name: &String, line_n: i32) -> Vec<RawToken> {
        vec![RawToken::Keyword("G".to_string(), line_n),
             RawToken::LabelP(func_name.to_string(), line_n)]
    }

    
    // после вызова внутри функции
    // P.local_new
    fn _add_post_func_l(&self, line_n: i32) -> Vec<RawToken> {
        vec![RawToken::LabelP(self.local_rp_template.replace("{}", &self.ident_index.to_string()), line_n)]
    }


    // G PD.local
    fn _add_return_func(&self, line_n: i32) -> Vec<RawToken> {
        if self.is_need_label_p {
            vec![RawToken::Keyword("G".to_string(), line_n),
            RawToken::LabelPD(self.return_point_name_t.replace("{}", &self.local_p_index.to_string()), line_n)]
        } else {
            vec![RawToken::Keyword("G".to_string(), line_n),
            RawToken::LabelPD(self.return_point_name.to_string(), line_n)]
        }
    }

    fn _func_close(&self, line_n: i32) -> Vec<RawToken> {
        vec![RawToken::LabelP(self.close_func_t.replace("{}", &self.local_p_index.to_string()), line_n)]
    }

    fn _go_func_close(&self, line_n: i32) -> Vec<RawToken> {
        vec![RawToken::Keyword("G".to_string(), line_n),
             RawToken::LabelP(self.close_func_t.replace("{}", &self.local_p_index.to_string()), line_n)]
    }
}