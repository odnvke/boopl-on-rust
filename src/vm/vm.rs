use std::{collections::VecDeque, io::{self, Write}};
use crate::error::{BooplError, Result};
use crate::name_map::IdentNameMap;
use super::pre_run;

struct VM {
    memory: Vec<u8>,
    memory_pd: Vec<i32>,
    bytecode: Vec<Vec<i32>>,
    input_buffer: VecDeque<char>,
}

impl VM {
    fn new() -> Self {
        VM {
            memory: Vec::new(),
            memory_pd: Vec::new(),
            bytecode: Vec::new(),
            input_buffer: VecDeque::new(),
        }
    }

    fn run(&mut self, program: Vec<Vec<i32>>, ident_name_map: IdentNameMap, lines_n: Vec<i32>) -> Result<()> {
        self.bytecode = program;
        let mut pc = 0;
        let mut instr_counter = 0;
        let mut log_mode = false;
        let mut step_mode = false;
        
        loop {
            if pc >= self.bytecode.len() { 
                break; 
            }
            
            let line = &self.bytecode[pc];
            
            if line.is_empty() {
                return Err(vm_error("Пустая инструкция".to_string(), lines_n.get(pc).copied().unwrap_or(-1)));
            }

            let opcode = line[0];
            let line = &line[1..];
            let line_num = lines_n[pc];

            if log_mode {
                print!("\n  [pc: {}; instr: {}; line: {}]  ", pc, instr_counter, line_num);
            }
            
            match opcode {
                0 => {}

                50 => break,

                51 => {}

                // 10 F
                100 => {
                    if (line[0] as usize) >= self.memory.len() {
                        self.memory.resize(line[0] as usize + 1, 0);
                    }
                    self.memory[line[0] as usize] = 0;
                    if log_mode {
                        print!("{} <= False", ident_name_map.get_name_n(line[0]));
                    }
                }
                // 10 T
                101 => {
                    if (line[0] as usize) >= self.memory.len() {
                        self.memory.resize(line[0] as usize + 1, 0);
                    }
                    self.memory[line[0] as usize] = 1;
                    if log_mode {
                        print!("{} <= True", ident_name_map.get_name_n(line[0]));
                    }
                }
                // 10 10
                150 => {
                    if (line[1] as usize) >= self.memory.len() {
                        return Err(vm_error(format!("   >>  ! несуществующая ячейка {}", ident_name_map.get_name_n(line[1])), line_num));
                    }
                    let val = self.memory[line[1] as usize];
                    if (line[0] as usize) >= self.memory.len() {
                        self.memory.resize(line[0] as usize + 1, 0);
                    }
                    self.memory[line[0] as usize] = val;
                    if log_mode {
                        print!("{} <= {} ({})", ident_name_map.get_name_n(line[0]), 
                               ident_name_map.get_name_n(line[1]), val == 1);
                    }
                }

                // PD.10
                201 => {
                    if (line[0] as usize) >= self.memory_pd.len() {
                        self.memory_pd.resize(line[0] as usize + 1, -1);
                    }
                    self.memory_pd[line[0] as usize] = pc as i32;
                    if log_mode {
                        print!("PD.{} <= {}", ident_name_map.get_name_pd(line[0]), pc);
                    }
                }

                // G P.10
                230 => {
                    if log_mode {
                        print!("GO TO: {}(instr) (from: {})", line[0], pc);
                    }
                    pc = line[0] as usize;
                }
                // G PD.10
                231 => { 
                    if (line[0] as usize) < self.memory_pd.len() && self.memory_pd[line[0] as usize] >= 0 {
                        if log_mode {
                            print!("GO TO: PD.{} (from: {})", ident_name_map.get_name_pd(line[0]), pc);
                        }
                        pc = self.memory_pd[line[0] as usize] as usize;
                    } else {
                        return Err(vm_error(format!("   >>  ! несуществующий динамический указатель PD.{}", 
                                                     ident_name_map.get_name_pd(line[0])), line_num));
                    }
                }

                // PD.10 P.10
                260 => {
                    if (line[0] as usize) >= self.memory_pd.len() {
                        self.memory_pd.resize(line[0] as usize + 1, -1);
                    }
                    self.memory_pd[line[0] as usize] = line[1];
                    if log_mode {
                        print!("PD.{} <= {}(instr)", ident_name_map.get_name_pd(line[0]), line[1]);
                    }
                }
                // PD.10 PD.10
                261 => { 
                    if (line[1] as usize) < self.memory_pd.len() && self.memory_pd[line[1] as usize] >= 0 {
                        let val = self.memory_pd[line[1] as usize];
                        if (line[0] as usize) >= self.memory_pd.len() {
                            self.memory_pd.resize(line[0] as usize + 1, -1);
                        }
                        self.memory_pd[line[0] as usize] = val;
                        if log_mode {
                            print!("PD.{} <= PD.{} ({})", ident_name_map.get_name_pd(line[0]), 
                                   ident_name_map.get_name_pd(line[1]), val);
                        }
                    } else {
                        return Err(vm_error(format!("   >>  ! несуществующий динамический указатель PD.{}", 
                                                     ident_name_map.get_name_pd(line[1])), line_num));
                    }
                }

                // IF 10 6(указатель на иначе)
                300 => {
                    if (line[0] as usize) < self.memory.len() {
                        if log_mode {
                            print!("IF {} (end on {}) : ", ident_name_map.get_name_n(line[0]), line[1]);
                        }
                        if self.memory[line[0] as usize] == 1 {
                            if log_mode { print!("Run IF Block"); }
                        } else {
                            pc = line[1] as usize; 
                            if log_mode { print!("GO TO End IF"); }
                        }
                    } else {
                        return Err(vm_error(format!("   >>  ! несуществующая ячейка {}", ident_name_map.get_name_n(line[0])), line_num));
                    }
                }
                // IFG 10 P.10
                302 => {
                    if (line[0] as usize) < self.memory.len() {
                        if log_mode {
                            print!("IF {} GO TO {}(instr) : ", ident_name_map.get_name_n(line[0]), line[1]);
                        }
                        if self.memory[line[0] as usize] == 1 {
                            pc = line[1] as usize;
                            if log_mode { print!("GO TO"); }
                        } else {
                            if log_mode { print!("next instr"); }
                        }
                    } else {
                        return Err(vm_error(format!("   >>  ! несуществующая ячейка {}", ident_name_map.get_name_n(line[0])), line_num));
                    }
                }
                // IFG 10 PD.10
                303 => {
                    if (line[0] as usize) < self.memory.len() {
                        if (line[1] as usize) < self.memory_pd.len() && self.memory_pd[line[1] as usize] >= 0 {
                            if log_mode {
                                print!("IF {} GOTO {} ({}) :", ident_name_map.get_name_n(line[0]), 
                                      ident_name_map.get_name_pd(line[1]), self.memory_pd[line[1] as usize]);
                            }
                            if self.memory[line[0] as usize] == 1 {
                                pc = self.memory_pd[line[1] as usize] as usize;
                                if log_mode { print!("GO TO"); }
                            } else {
                                if log_mode { print!("next instr"); }
                            }
                        } else {
                            return Err(vm_error(format!("   >>  ! несуществующий динамический указатель PD.{}", 
                                                         ident_name_map.get_name_pd(line[1])), line_num));
                        }
                    } else {
                        return Err(vm_error(format!("   >>  ! несуществующая ячейка {}", ident_name_map.get_name_n(line[0])), line_num));
                    }
                }

                // P T
                400 => {
                    if log_mode { print!("PRINT TRUE: "); }
                    print!("#");
                }
                // P F
                401 => {
                    if log_mode { print!("PRINT FALSE: "); }
                    print!(".");
                }
                // P 10
                402 => {
                    if (line[0] as usize) < self.memory.len() {
                        if log_mode { 
                            print!("PRINT {}: ", ident_name_map.get_name_n(line[0])); 
                        }
                        if self.memory[line[0] as usize] == 1 { 
                            print!("#"); 
                        } else { 
                            print!("."); 
                        }
                    } else {
                        return Err(vm_error(format!("   >>  ! несуществующая ячейка {}", ident_name_map.get_name_n(line[0])), line_num));
                    }
                }
                // P N
                403 => {
                    if log_mode { print!("NEW LINE: "); }
                    println!();
                }
                // P S
                404 => {
                    if log_mode { print!("PRINT SPACE: "); }
                    print!(" ");
                }

                // P U 10
                405 => {
                    if log_mode { 
                        print!("PRINT UTF-8 (from: {}): ", ident_name_map.get_name_n(line[0])); 
                    }
                    let start_addr = line[0] as usize;
                    let mut bytes = Vec::new();

                    // Читаем первый байт
                    let first_byte = {
                        let mut byte_value: u8 = 0;
                        for i in 0..8 {
                            if start_addr + i >= self.memory.len() {
                                return Err(vm_error(format!("   >>  ! несуществующая ячейка {}", 
                                                             ident_name_map.get_name_n((start_addr + i) as i32)), line_num));
                            }
                            let bit = self.memory[start_addr + i];
                            byte_value = (byte_value << 1) | (bit as u8);
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
                        let byte_addr = start_addr + 8 + (i * 8);
                        if byte_addr + 7 >= self.memory.len() {
                            return Err(vm_error(format!("   >>  ! несуществующая ячейка {}", 
                                                         ident_name_map.get_name_n(byte_addr as i32)), line_num));
                        }
                        let next_byte = {
                            let mut byte_value: u8 = 0;
                            for j in 0..8 {
                                let bit = self.memory[byte_addr + j];
                                byte_value = (byte_value << 1) | (bit as u8);
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
                    if (line[1] as usize) < self.memory.len() {
                        let val = self.memory[line[1] as usize];
                        if (line[0] as usize) >= self.memory.len() {
                            self.memory.resize(line[0] as usize + 1, 0);
                        }
                        self.memory[line[0] as usize] = if val == 1 {0} else {1};
                        if log_mode {
                            print!("{} <= NOT {}", ident_name_map.get_name_n(line[0]), 
                                   ident_name_map.get_name_n(line[1]));
                        }
                    } else {
                        return Err(vm_error(format!("   >>  ! несуществующая ячейка {}", 
                                                     ident_name_map.get_name_n(line[1])), line_num));
                    }
                }

                // 10 O 10 10
                550 => {
                    if (line[1] as usize) < self.memory.len() && (line[2] as usize) < self.memory.len() {
                        let a = self.memory[line[1] as usize];
                        let b = self.memory[line[2] as usize];
                        if (line[0] as usize) >= self.memory.len() {
                            self.memory.resize(line[0] as usize + 1, 0);
                        }
                        self.memory[line[0] as usize] = if a == 1 || b == 1 {1} else {0};
                        if log_mode {
                            print!("{} <= {} OR {}", ident_name_map.get_name_n(line[0]), 
                                   ident_name_map.get_name_n(line[1]), ident_name_map.get_name_n(line[2]));
                        }
                    } else {
                        return Err(vm_error(format!("   >>  ! несуществующая ячейка {} или {}", 
                            ident_name_map.get_name_n(line[1]), ident_name_map.get_name_n(line[2])), line_num));
                    }
                }
                // 10 A 10 10
                551 => {
                    if (line[1] as usize) < self.memory.len() && (line[2] as usize) < self.memory.len() {
                        let a = self.memory[line[1] as usize];
                        let b = self.memory[line[2] as usize];
                        if (line[0] as usize) >= self.memory.len() {
                            self.memory.resize(line[0] as usize + 1, 0);
                        }
                        self.memory[line[0] as usize] = if a == 1 && b == 1 {1} else {0};
                        if log_mode {
                            print!("{} <= {} AND {}", ident_name_map.get_name_n(line[0]), 
                                   ident_name_map.get_name_n(line[1]), ident_name_map.get_name_n(line[2]));
                        }
                    } else {
                        return Err(vm_error(format!("   >>  ! несуществующая ячейка {} или {}", 
                            ident_name_map.get_name_n(line[1]), ident_name_map.get_name_n(line[2])), line_num));
                    }
                }
                // 10 X 10 10
                552 => {
                    if (line[1] as usize) < self.memory.len() && (line[2] as usize) < self.memory.len() {
                        let a = self.memory[line[1] as usize];
                        let b = self.memory[line[2] as usize];
                        if (line[0] as usize) >= self.memory.len() {
                            self.memory.resize(line[0] as usize + 1, 0);
                        }
                        self.memory[line[0] as usize] = if (a == 1) != (b == 1) {1} else {0};
                        if log_mode {
                            print!("{} <= {} XOR {}", ident_name_map.get_name_n(line[0]), 
                                   ident_name_map.get_name_n(line[1]), ident_name_map.get_name_n(line[2]));
                        }
                    } else {
                        return Err(vm_error(format!("   >>  ! несуществующая ячейка {} или {}", 
                            ident_name_map.get_name_n(line[1]), ident_name_map.get_name_n(line[2])), line_num));
                    }
                }

                // 600: IN добавляет строку в буффер
                600 => {
                    if log_mode { print!("INPUT Bool: "); }
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    for ch in input.chars() {
                        match ch {
                            'T' | 't' | '1' | '#' | 'F' | 'f' | '0' | '.' => {
                                self.input_buffer.push_back(ch.to_ascii_uppercase());
                            }
                            _ => {}
                        }
                    }
                    if log_mode {
                        print!("\n  INPUT Bool Closed; INPUT Buffer: {}", 
                               self.input_buffer.iter().collect::<String>());
                    }
                }

                // 601: IN U
                601 => {
                    if log_mode { print!("INPUT UTF-8: "); }
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    input.chars().for_each(|ch| self.input_buffer.push_back(ch));
                    if log_mode {
                        print!("\n  INPUT UTF-8 Closed; INPUT Buffer: {}", 
                               self.input_buffer.iter().collect::<String>());
                    }
                }

                // INBC   очистить буффер
                625 => {
                    if log_mode { print!("INPUT Buffer cleared"); }
                    self.input_buffer.clear();
                }

                // 10 INBC   проверка пустоты
                650 => {
                    if (line[0] as usize) >= self.memory.len() {
                        self.memory.resize(line[0] as usize + 1, 0);
                    }
                    self.memory[line[0] as usize] = if self.input_buffer.is_empty() { 1 } else { 0 };
                    if log_mode {
                        print!("{} <= IF INPUT Buffer clear ({})", 
                               ident_name_map.get_name_n(line[0]), self.input_buffer.is_empty());
                    }
                }

                // 10 INB    берёт следующий символ
                675 => {
                    if let Some(ch) = self.input_buffer.pop_front() {
                        if (line[0] as usize) >= self.memory.len() {
                            self.memory.resize(line[0] as usize + 1, 0);
                        }
                        let value = match ch {
                            'T' | 't' | '1' | '#' => 1,
                            'F' | 'f' | '0' | '.' => 0,
                            _ => {
                                return Err(vm_error(format!("   >> ! символ неподходящий для ячейки памяти {}", ch), line_num));
                            }
                        };
                        self.memory[line[0] as usize] = value;
                        if log_mode {
                            print!("{} <= First Char From INPUT Buffer ({})", 
                                   ident_name_map.get_name_n(line[0]), ch);
                        }
                    } else {
                        return Err(vm_error("   >>  ! буфер ввода пустой".to_string(), line_num));
                    }
                }

                // 676: 10 U INB - взять первый символ из буфера
                676 => {
                    if let Some(ch) = self.input_buffer.pop_front() {
                        if log_mode {
                            print!("{} <= First Char UTF-8 From INPUT Buffer ({})", 
                                   ident_name_map.get_name_n(line[0]), ch);
                        }
                        self.store_char_to_memory(line[0], ch);
                    } else {
                        return Err(vm_error("   >>  ! буфер ввода пустой (utf-8) ".to_string(), line_num));
                    }
                }

                700 => {
                    println!("\n\n   >>   BreakPoint on {}\n\n", line_num);
                    break;
                }

                730 => {
                    println!("\n   >>   StepPoint on {}: press Enter", line_num);
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                }

                760 => {
                    if log_mode { 
                        print!("log_mode: {}; step_mode: {}", log_mode, step_mode); 
                    }
                    if line[0] == 0 || line[0] == 2 { 
                        step_mode = true; 
                    }
                    if line[0] == 1 || line[0] == 2 { 
                        log_mode = true; 
                    }
                }

                761 => {
                    if log_mode { 
                        print!("log_mode: {}; step_mode: {}", log_mode, step_mode); 
                    }
                    if line[0] == 0 || line[0] == 2 { 
                        step_mode = false; 
                    }
                    if line[0] == 1 || line[0] == 2 { 
                        log_mode = false; 
                    }
                }
                
                _ => {
                    return Err(vm_error(format!("Неизвестный опкод: {}", opcode), line_num));
                }
            }

            if step_mode {
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
            }

            pc += 1;
            instr_counter += 1;
        }
        Ok(())
    }

    fn store_char_to_memory(&mut self, start_addr: i32, ch: char) {
        let mut bytes = [0u8; 4];
        let utf8_bytes = ch.encode_utf8(&mut bytes);
        let start = start_addr as usize;

        // Очистить 32 ячейки
        for i in 0..32 {
            if start + i >= self.memory.len() {
                self.memory.resize(start + i + 1, 0);
            }
            self.memory[start + i] = 0;
        }

        // Записать байты
        for (byte_idx, &byte) in utf8_bytes.as_bytes().iter().enumerate() {
            for bit_pos in 0..8 {
                let addr = start + (byte_idx * 8) + bit_pos;
                if addr >= self.memory.len() {
                    self.memory.resize(addr + 1, 0);
                }
                let bit_shift = 7 - bit_pos;
                let bit = (byte >> bit_shift) & 1;
                self.memory[addr] = bit;
            }
        }
    }
}

fn vm_error(message: String, line_num: i32) -> BooplError {
    BooplError::new(message, line_num)
}

pub fn start(bytecode: Vec<(Vec<i32>, i32)>, ident_name_map: IdentNameMap) -> Result<()> {
    let mut vm = VM::new();
    let (program, lines_n) = pre_run::pre_run(bytecode, &ident_name_map);
    
    if program.len() != lines_n.len() {
        return Err(BooplError::new(
            format!("длина байткода ({}) != длина номеров строк ({})", 
                program.len(), lines_n.len()),
            -1
        ));
    }
    
    vm.run(program, ident_name_map, lines_n)?;
    Ok(())
}