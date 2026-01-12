use std::{collections::HashMap};
use crossterm::{terminal, event::{read, Event, KeyCode}};
//use std::thread;
use std::time::Duration;

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
                0 => {}

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

600 => {
    use std::io::{self, Write};
    use crossterm::event::{poll, read, Event, KeyCode};
    use std::time::Duration;
    
    print!("_");
    io::stdout().flush().unwrap();
    
    // Включаем raw mode
    terminal::enable_raw_mode().unwrap();
    
    // ОЧИЩАЕМ БУФЕР ПЕРЕД НАЧАЛОМ
    while poll(Duration::from_millis(0)).unwrap_or(false) {
        let _ = read();
    }
    
    let value = loop {
        // Ждём событие с таймаутом
        if poll(Duration::from_millis(100)).unwrap_or(false) {
            match read() {
                Ok(Event::Key(event)) => {
                    match event.code {
                        KeyCode::Char('t') | KeyCode::Char('T') | KeyCode::Char('1') | KeyCode::Char('#') | KeyCode::Char('е') | KeyCode::Char('Е') => {
                            print!("\r\r");
                            break 1;
                        }
                        KeyCode::Char('f') | KeyCode::Char('F') | KeyCode::Char('0') | KeyCode::Char('.') | KeyCode::Char('а') | KeyCode::Char('А') => {
                            print!("\r\r");
                            break 0;
                        }
                        _ => {
                            // Игнорируем, но продолжаем ждать
                            continue;
                        }
                    }
                }
                _ => continue,
            }
        }
    };

    // ОЧИЩАЕМ БУФЕР ПОСЛЕ ПОЛУЧЕНИЯ ЗНАЧЕНИЯ
    while poll(Duration::from_millis(0)).unwrap_or(false) {
        let _ = read();
    }
    
    terminal::disable_raw_mode().unwrap();
    println!();
    
    self.memory.insert(line[0], value as u8);
}

                601 => { // Ввод UTF-8 символа и сохранение в последовательность ячеек
                    use crossterm::{event::poll, terminal::enable_raw_mode, terminal::disable_raw_mode};
                    
                    enable_raw_mode().unwrap();
                    
                    while poll(Duration::from_millis(10)).unwrap() { let _ = read(); }

                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    
                    let mut utf8_bytes = Vec::new();
                    let mut got_input = false;
                    
                    while !got_input {
                        if poll(Duration::from_millis(100)).unwrap() {
                            match read() {
                                Ok(Event::Key(event)) => {
                                    match event.code {
                                        KeyCode::Char(c) => {
                                            utf8_bytes = c.to_string().into_bytes();
                                            got_input = true;
                                            //print!("\r")
                                        }
                                        _ => {}
                                    }
                                }
                                Err(e) => {
                                    error_print(format!("\n ! ран-тайм\n\n   >>  ! Ошибка чтения: {:?}", e), line_n)
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    disable_raw_mode().unwrap();
                    
                    // Сохраняем байты UTF-8 в память
                    let start_addr = line[0] as i32;
                    
                    // Очищаем область (8 ячеек на каждый возможный байт)
                    for byte_idx in 0..4 {
                        for bit_idx in 0..8 {
                            let addr = start_addr + (byte_idx as i32 * 8) + bit_idx as i32;
                            self.memory.insert(addr, 0);
                        }
                    }
                    
                    // Записываем фактически введённые байты
                    for (byte_idx, &byte) in utf8_bytes.iter().enumerate() {
                        for bit_idx in 0..8 {
                            let addr = start_addr + (byte_idx as i32 * 8) + (7 - bit_idx) as i32;
                            let bit = (byte >> bit_idx) & 1;
                            self.memory.insert(addr, bit as u8);
                        }
                    }
                }

                _ => {panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAAA!!!")}
            }
            pc += 1;
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