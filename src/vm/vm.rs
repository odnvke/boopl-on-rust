use std::collections::{HashMap};

use crate::name_map::IdentNameMap;

use super::pre_run;

struct VM {
    memory: HashMap<i32, u8>,
    memory_pd: HashMap<i32, i32>,
    bytecode: Vec<Vec<i32>>,
}

impl VM {
    fn new() -> Self {
        VM {
            memory: HashMap::new(),
            memory_pd: HashMap::new(),
            bytecode: Vec::new()
        }
    }
    
    fn run(&mut self, program: Vec<Vec<i32>>, ident_name_map: IdentNameMap, lines_n: Vec<i32>) {
        self.bytecode = program;
        let mut pc = 0;
        loop {
            if pc >= self.bytecode.len() {break;}
            
            let line: &Vec<i32> = &self.bytecode[pc]; 
            let opcode = line[0];
            let line = &line[1..];
            let line_n = lines_n[pc];
            match opcode {
                50 => {break}

                51 => {}

                // 10 F
                100 => {self.memory.insert(line[0],0);}
                // 10 T
                101 => {self.memory.insert(line[0],1);}
                // 10 10
                150 => {
                    if self.memory.contains_key(&line[1]) {
                        self.memory.insert(line[0], *self.memory.get(&line[1]).unwrap_or(&0));
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[1])), line_n)
                    }
                }
                // PD.10      ! P.10 небудет
                201 => {self.memory_pd.insert(line[0], pc as i32);}

                // G P.10
                230 => {pc = line[0] as usize}
                // G PD.10
                231 => { if self.memory_pd.contains_key(&line[0]) {
                        pc = self.memory_pd[&line[0]] as usize
                    } else {
                        error_print(format!("   >>  ! несушествующий динамический указатель PD.{}", ident_name_map.get_name(line[0])), line_n)
                    }
                }

                // PD.10 P.10
                260 => {self.memory_pd.insert(line[0], line[1] );}
                // PD.10 PD.10
                261 => { 
                    if self.memory_pd.contains_key(&line[1]) {
                        self.memory_pd.insert(line[0], line[1] );
                    } else {
                        error_print(format!("   >>  ! несушествующий динамический указатель PD.{}", ident_name_map.get_name(line[1])), line_n)
                    }
                }

                // I 10 6(указатель на иначе)
                300 => {
                    if self.memory.contains_key(&line[0]) {
                        if self.memory[&line[0]] == 1 {}
                        else {pc = line[1] as usize}
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[0])), line_n)
                    }
                }
                // IG 10 P.10
                302 => {
                    if self.memory.contains_key(&line[0]) { 
                        if self.memory[&line[0]] == 1 {pc = line[1] as usize}
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[0])), line_n)
                    }
                }
                // IG 10 PD.10
                303 => {
                    if self.memory.contains_key(&line[0]) {
                            if self.memory_pd.contains_key(&line[1]) {
                            if self.memory[&line[0]] == 1 {pc = self.memory_pd[&line[1]] as usize}
                        }
                        else {
                            error_print(format!("   >>  ! несушествующий динамический указатель PD.{}", ident_name_map.get_name(line[1])), line_n)
                        }
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[0])), line_n)
                    }
                }

                // P T
                400 => {print!("#")}
                // P F
                401 => {print!(".")}
                // P 10
                402 => {if self.memory.contains_key(&line[0]) {
                        if self.memory[&line[0]] == 1 {print!("#")} else {print!(".")}
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[0])), line_n)
                    }
                }
                // P N
                403 => {println!()}
                // P S
                404 => {print!(" ")}

                // P U 10
                405 => {
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
                        self.memory.insert(line[0], if *self.memory.get(&line[1]).unwrap_or(&0) == 1 {0} else {1});
                    } else {
                        error_print(format!("   >>  ! несушествующая ячейка {}", ident_name_map.get_name(line[1])), line_n)
                    }
                }

                // 10 O 10 10
                550 => {
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

                _ => {panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAAA!!!")}
            }
            pc += 1;
        }
    }
}

fn error_print(s: String, line_n: i32) {
    panic!("\n ! ран-тайм\n\n{}  ({})\n\n", s, line_n)
}

pub fn start(bytecode: Vec<(Vec<i32>, i32)>, ident_name_map: IdentNameMap) {
    let mut vm = VM::new();

    let (program, lines_n) = pre_run::pre_run(bytecode, &ident_name_map);

    if program.len() != lines_n.len() {error_print(format!("АААААА!!! байткод длина: {}; лайн_н длина: {}", 
                                                    program.len(), lines_n.len()), -1);}
    
    vm.run(program, ident_name_map, lines_n);
}