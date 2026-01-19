use std::{collections::{HashMap, VecDeque}};

use crate::name_map::IdentNameMap;

use super::pre_run;

struct VM {
    memory: HashMap<i32, u8>,
    memory_pd: HashMap<i32, i32>,
    bytecode: Vec<Vec<i32>>,
    input_buffer: VecDeque<char>,
}

impl VM {
    fn new() -> Self {
        VM {
            memory: HashMap::new(),
            memory_pd: HashMap::new(),
            bytecode: Vec::new(),
            input_buffer: VecDeque::new(),
        }
    }
    
    fn run(&mut self, program: Vec<Vec<i32>>, ident_name_map: IdentNameMap, lines_n: Vec<i32>) {
        self.bytecode = program;
        let mut pc = 0;
        let mut instr_couter = 0;
        let mut log_mode = false;
        let mut step_mode = false;
        loop {
            if pc >= self.bytecode.len() {break;}
            let line: &Vec<i32> = &self.bytecode[pc]; 
            let opcode = line[0];
            let line = &line[1..];
            let line_n = lines_n[pc];

            if log_mode {print!("\n  [pc: {}; instr: {}; line: {}]  ", pc, instr_couter, line_n)}
            match opcode {
                0 => {}

                50 => {break}

                51 => {}

                // 10 F
                100 => {
                    self.memory.insert(line[0],0);
                    if log_mode {print!("{} <= False", ident_name_map.get_name(line[0]))}
                }
                // 10 T
                101 => {
                    self.memory.insert(line[0],1);
                    if log_mode {print!("{} <= False", ident_name_map.get_name(line[0]))}
                }
                // 10 10
                150 => {
                    if self.memory.contains_key(&line[1]) {
                        self.memory.insert(line[0], *self.memory.get(&line[1]).unwrap_or(&0));
                        if log_mode {print!("{} <= {} ({})", ident_name_map.get_name(line[0]), ident_name_map.get_name(line[1]), line[1] == 1)}
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[1])), line_n)
                    }
                }
                // PD.10      ! P.10 небудет
                201 => {
                    self.memory_pd.insert(line[0], pc as i32);
                    if log_mode {print!("PD.{} <= {}", ident_name_map.get_name(line[0]), pc)}
                }

                // G P.10
                230 => {
                    if log_mode {print!("GO TO: {}(instr) (from: {})", line[0], pc)};
                    pc = line[0] as usize;
                }
                // G PD.10
                231 => { if self.memory_pd.contains_key(&line[0]) {
                        if log_mode {print!("GO TO: PD.{} (from: {})", ident_name_map.get_name(line[0]), pc)}
                        pc = self.memory_pd[&line[0]] as usize
                    } else {
                        error_print(format!("   >>  ! несушествующий динамический указатель PD.{}", ident_name_map.get_name(line[0])), line_n)
                    }
                }

                // PD.10 P.10
                260 => {
                    if log_mode {print!("PD.{} <= {}(instr)", ident_name_map.get_name(line[0]), line[1])}
                    self.memory_pd.insert(line[0], line[1] );
                }
                // PD.10 PD.10
                261 => { 
                    if self.memory_pd.contains_key(&line[1]) {
                        if log_mode {print!("PD.{} <= PD.{} ({})", ident_name_map.get_name(line[0]), ident_name_map.get_name(line[1]), line[1])}
                        self.memory_pd.insert(line[0], *self.memory_pd.get(&line[1]).unwrap_or(&0));
                    } else {
                        error_print(format!("   >>  ! несушествующий динамический указатель PD.{}", ident_name_map.get_name(line[1])), line_n)
                    }
                }

                // I 10 6(указатель на иначе)
                300 => {
                    if self.memory.contains_key(&line[0]) {
                        if log_mode {print!("IF {} (end on {}) : ", ident_name_map.get_name(line[0]), line[1])}
                        if self.memory[&line[0]] == 1 {
                            if log_mode {print!("Run IF Block")}
                        }
                        else {
                            pc = line[1] as usize; 
                            if log_mode {print!("GO TO End IF")}
                        }
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[0])), line_n)
                    }
                }
                // IG 10 P.10
                302 => {
                    if self.memory.contains_key(&line[0]) {
                        if log_mode {print!("IF {} GO TO {}(instr) : ", ident_name_map.get_name(line[0]), line[1])}
                        if self.memory[&line[0]] == 1 {pc = line[1] as usize; if log_mode {print!("GO TO")}}
                        else {if log_mode {print!("next instr")}}
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[0])), line_n)
                    }
                }
                // IG 10 PD.10
                303 => {
                    if self.memory.contains_key(&line[0]) {
                            if self.memory_pd.contains_key(&line[1]) {
                            if log_mode {print!("IF {} GOTO {} ({}) :", ident_name_map.get_name(line[0]), 
                                    ident_name_map.get_name(line[1]), self.memory_pd[&line[1]])}
                            if self.memory[&line[0]] == 1 {pc = self.memory_pd[&line[1]] as usize; if log_mode {print!("GO TO")}}
                            else {if log_mode {print!("next instr")}}
                        }
                        else {
                            error_print(format!("   >>  ! несушествующий динамический указатель PD.{}", ident_name_map.get_name(line[1])), line_n)
                        }
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[0])), line_n)
                    }
                }

                // P T
                400 => {
                    if log_mode {print!("PRINT TRUE: ")}
                    print!("#")
                }
                // P F
                401 => {
                    if log_mode {print!("PRINT FALSE: ")}
                    print!(".")
                }
                // P 10
                402 => {if self.memory.contains_key(&line[0]) {
                        if log_mode {print!("PRINT {}: ", ident_name_map.get_name(line[0]))}
                        if self.memory[&line[0]] == 1 {print!("#")} else {print!(".")}
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[0])), line_n)
                    }
                }
                // P N
                403 => {
                    if log_mode {print!("NEW LINE: ")}
                    println!()
                }
                // P S
                404 => {
                    if log_mode {print!("PRINT SPACE: ")}
                    print!(" ")
                }

                // P U 10
                405 => {
                    if log_mode {print!("PRINT UTF-8 (from: {}): ", ident_name_map.get_name(line[0]))}
                    let start_addr = line[0];
                    let mut bytes = Vec::new();
                    
                    // Читаем первый байт
                    let first_byte = {
                        let mut byte_value: u8 = 0;
                        for i in 0..8 {
                            let bit = if self.memory.contains_key(&(start_addr + i)) {
                                *self.memory.get(&(start_addr + i)).unwrap_or(&0)
                            } else {
                                error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(start_addr+i)), line_n);
                                panic!();
                            };
                            byte_value = (byte_value << 1) | (bit as u8)
                        }
                        byte_value
                    };
                    bytes.push(first_byte);
                    
                    // Определяем сколько еще байтов нужно прочитать
                    let additional_bytes = if (first_byte & 0b10000000) == 0 {
                        0  // ASCII
                    } else if (first_byte & 0b11100000) == 0b11000000 {
                        1  // 2 байта
                    } else if (first_byte & 0b11110000) == 0b11100000 {
                        2  // 3 байта  
                    } else if (first_byte & 0b11111000) == 0b11110000 {
                        3  // 4 байта
                    } else {
                        0  // Некорректный
                    };
                    
                    // Читаем дополнительные байты
                    for i in 0..additional_bytes {
                        let byte_addr = start_addr + 8 + (i * 8); // каждый байт через 8 ячеек
                        let next_byte = {
                            let mut byte_value: u8 = 0;
                            for i in 0..8 {
                                let bit = if self.memory.contains_key(&(byte_addr + i)) {
                                    *self.memory.get(&(byte_addr + i)).unwrap_or(&0)
                                } else {
                                    error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(start_addr+i)), line_n);
                                    panic!()
                                };
                                byte_value = (byte_value << 1) | (bit as u8)
                            }
                            byte_value
                        };
                        bytes.push(next_byte);
                    }
                    
                    // Проверяем валидность и выводим
                    if let Ok(s) = String::from_utf8(bytes) {
                        print!("{}", s);
                    } else {
                        print!("�");
                    }
                }

                // 10 N 10
                500 => {
                    if self.memory.contains_key(&line[1]) {
                        if log_mode {print!("{} <= NOT {}", ident_name_map.get_name(line[0]), ident_name_map.get_name(line[1]))}
                        self.memory.insert(line[0], if *self.memory.get(&line[1]).unwrap_or(&0) == 1 {0} else {1});
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[1])), line_n)
                    }
                }

                // 10 O 10 10
                550 => {
                    if log_mode {print!("{} <= {} OR {}", ident_name_map.get_name(line[0]), 
                            ident_name_map.get_name(line[1]), ident_name_map.get_name(line[2]))}
                    if self.memory.contains_key(&line[1]) && self.memory.contains_key(&line[2]) {
                        if *self.memory.get(&line[1]).unwrap_or(&0) == 1 || *self.memory.get(&line[2]).unwrap_or(&0) == 1 {
                            self.memory.insert(line[0], 1);
                        } else {
                            self.memory.insert(line[0], 0);
                        }
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {} или {}", 
                            ident_name_map.get_name(line[1]), ident_name_map.get_name(line[2])), line_n)
                    }
                }
                // 10 A 10 10
                551 => {
                    if log_mode {print!("{} <= {} AND {}", ident_name_map.get_name(line[0]), 
                            ident_name_map.get_name(line[1]), ident_name_map.get_name(line[2]))}
                    if self.memory.contains_key(&line[1]) && self.memory.contains_key(&line[2]) {
                        if *self.memory.get(&line[1]).unwrap_or(&0) == 1 && *self.memory.get(&line[2]).unwrap_or(&0) == 1 {
                            self.memory.insert(line[0], 1);
                        } else {
                            self.memory.insert(line[0], 0);
                        }
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {} или {}", 
                            ident_name_map.get_name(line[1]), ident_name_map.get_name(line[2])), line_n)
                    }
                }
                // 10 X 10 10
                552 => {
                        if log_mode {print!("{} <= {} XOR {}", ident_name_map.get_name(line[0]), 
                                ident_name_map.get_name(line[1]), ident_name_map.get_name(line[2]))}
                    if self.memory.contains_key(&line[1]) && self.memory.contains_key(&line[2]) {
                        let a = if *self.memory.get(&line[1]).unwrap_or(&0) == 1 {1} else {0};
                        let b = if *self.memory.get(&line[2]).unwrap_or(&0) == 1 {1} else {0};
                        if a + b == 1 {
                            self.memory.insert(line[0], 1);
                        } else {
                            self.memory.insert(line[0], 0);
                        }
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {} или {}", 
                            ident_name_map.get_name(line[1]), ident_name_map.get_name(line[2])), line_n);
                    }
                }

                // 600: IN добавляет строку в буффер
                600 => {
                    if log_mode {print!("INPUT Bool: ")}

                    use std::io::{self, Write};
                    
                    io::stdout().flush().unwrap();
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    
                    // Каждый символ строки добавляем в буфер
                    for ch in input.chars() {
                        match ch {
                            'T' | 't' | '1' | '#' | 'F' | 'f' | '0' | '.'=> {
                                // Добавляем только валидные символы
                                self.input_buffer.push_back(ch.to_ascii_uppercase());
                            }
                            _ => {}
                        }
                         
                    }
                    if log_mode {print!("\n  INPUT Bool Closed; INPUT Buffer: {}", self.input_buffer.iter().collect::<String>())}
                }

                // 601: IN U
                601 => {
                    if log_mode {print!("INPUT UTF-8: ")}
                    use std::io::{self, Write};
                    
                    io::stdout().flush().unwrap();
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    
                    input.chars().for_each(|ch| self.input_buffer.push_back(ch));
                    if log_mode {print!("\n  INPUT UTF-8 Closed; INPUT Buffer: {}", self.input_buffer.iter().collect::<String>())}
                }

                // INBC   очистить буффер
                625 => {
                    if log_mode {print!("INPUT Buffer cleared")}
                    self.input_buffer.clear();
                }
                
                // 10 INBC   проверка пустоты
                650 => {
                    if log_mode {print!("{} <= IF INPUT Buffer clear ({})", ident_name_map.get_name(line[0]), self.input_buffer.is_empty())}
                    let result = if self.input_buffer.is_empty() { 1 } else { 0 };
                    self.memory.insert(line[0], result);
                }
                
                // 10 INB    берёт следующий символ
                675 => {
                    if let Some(ch) = self.input_buffer.pop_front() {
                        if log_mode {print!("{} <= First Char From INPUT Buffer ({})", ident_name_map.get_name(line[0]), ch)}
                        // Преобразуем char в T/F (1 или 0)
                        let value = match ch {
                            'T' | 't' | '1' | '#' => 1,
                            'F' | 'f' | '0' | '.' => 0,
                            _ => {error_print(format!("   >> ! символ неподходяший для ячейки памяти {}", ch), line_n); std::process::exit(1)}
                        };
                        self.memory.insert(line[0], value);
                    } else {
                        error_print(format!("   >>  ! буффур ввода пустой"), line_n);
                    }
                }


                // 676: 10 U INB - взять первый символ из буфера
                676 => {
                    if let Some(ch) = self.input_buffer.pop_front() {
                        if log_mode {print!("{} <= First Char UTF-8 From INPUT Buffer ({})", ident_name_map.get_name(line[0]), ch)}
                        self.store_char_to_memory(line[0], ch);
                    } else {
                        error_print("   >>  ! буффур ввода пустой (utf-8) ".to_string(), line_n);
                    }
                }

                700 => {
                    println!("\n\n   >>   BreakPoint on {}\n\n", line_n);
                    break;
                }

                730 => {
                    println!("\n   >>   StepPoint on {}: press Enter", line_n);
                    use std::io::{self, Write};
                    
                    io::stdout().flush().unwrap();
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                }

                760 => {
                    if log_mode {print!("log_mode: {}; step_mode: {}", log_mode, step_mode)}
                    if line[0] == 0 {step_mode = true} else {log_mode = true}
                }

                761 => {
                    if log_mode {print!("log_mode: {}; step_mode: {}", log_mode, step_mode)}
                    if line[0] == 0 {step_mode = false} else {log_mode = false}
                }
                _ => {panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAAA!!!")}
            }

            if step_mode {
                    use std::io::{self, Write};
                    
                    io::stdout().flush().unwrap();
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
            }

            pc += 1;
            instr_couter += 1;
        }
    }



    fn store_char_to_memory(&mut self, start_addr: i32, ch: char) {
        let mut bytes = [0u8; 4];
        let utf8_bytes = ch.encode_utf8(&mut bytes);
        
        // Очистить
        for i in 0..32 {
            self.memory.insert(start_addr + i, 0);
        }
        
        // Записать байты
        for (byte_idx, &byte) in utf8_bytes.as_bytes().iter().enumerate() {
            for bit_pos in 0..8 {
                let addr = start_addr + (byte_idx as i32 * 8) + bit_pos;
                // bit_pos=0 -> бит 7, bit_pos=1 -> бит 6, ..., bit_pos=7 -> бит 0
                let bit_shift = 7 - bit_pos;  // 7,6,5,4,3,2,1,0
                let bit = (byte >> bit_shift) & 1;
                self.memory.insert(addr, bit);
            }
        }
    }
}






fn error_print(s: String, line_n: i32) {
    eprintln!("\n ! ран-тайм\n\n{}  ({})\n\n", s, line_n); std::process::exit(1)
}

pub fn start(bytecode: Vec<(Vec<i32>, i32)>, ident_name_map: IdentNameMap) {
    let mut vm = VM::new();
    //println!("перед пре-ран{:?}", bytecode);
    let (program, lines_n) = pre_run::pre_run(bytecode, &ident_name_map);
    //println!("после пре-ран{:?}", program);
    if program.len() != lines_n.len() {error_print(format!("АААААА!!! байткод длина: {}; лайн_н длина: {}", 
                                                    program.len(), lines_n.len()), -1);}
    
    vm.run(program, ident_name_map, lines_n);
}