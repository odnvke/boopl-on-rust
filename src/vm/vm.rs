use std::collections::{HashMap};

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
    
    fn run(&mut self, program: Vec<Vec<i32>>) {
        self.bytecode = program;
        let mut pc = 0;
        loop {
            if pc >= self.bytecode.len() {break;}
            
            let line: &Vec<i32> = &self.bytecode[pc]; 
            let opcode = line[0];
            let line = &line[1..];
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
                        panic!("попытка взять значение из пустой ячейки {}", line[1])
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
                        panic!("попытка перейти к несуществующему динам указателю {}", line[0])
                    }
                }

                // PD.10 P.10
                260 => {self.memory_pd.insert(line[0], line[1] );}
                // PD.10 PD.10
                261 => { 
                    if self.memory_pd.contains_key(&line[1]) {
                        self.memory_pd.insert(line[0], line[1] );
                    } else {
                        panic!("попытка передать несуществующий динам указатель: {} динам указателю: {}", line[0], line[1])
                    }
                }

                // I 10 6(указатель на иначе)
                300 => {
                    if self.memory.contains_key(&line[0]) {
                        if self.memory[&line[0]] == 1 {}
                        else {pc = line[1] as usize}
                    } else {
                        panic!("в IF проверка не сушествующей ячейки")
                    }
                }
                // IG 10 P.10
                302 => {
                    if self.memory.contains_key(&line[0]) { 
                        if self.memory[&line[0]] == 1 {pc = line[1] as usize}
                    } else {
                        panic!("в IF проверка не сушествующей ячейки: {}", line[0])
                    }
                }
                // IG 10 PD.10
                303 => {
                    if self.memory.contains_key(&line[0]) {
                            if self.memory.contains_key(&line[1]) {
                            if self.memory[&line[0]] == 1 {pc = line[1] as usize}
                        }
                        else {
                            panic!("в IF GOTO указан не определённый динамический указатель: {}", line[1])
                        }
                    } else {
                        panic!("в IF проверка не сушествующей ячейки: {}", line[0])
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
                        panic!("попытка вывисти не определённую ячейку {}", line[0])
                    }
                }
                // P N
                403 => {println!()}
                // P S
                404 => {print!(" ")}

                // 10 N 10
                500 => {
                    if self.memory.contains_key(&line[1]) {
                        self.memory.insert(line[0], *self.memory.get(&line[1]).unwrap_or(&0));
                    } else {
                        panic!("в действии Not указана не опреденная ячейка {}", line[1])
                    }
                }

                // 10 O 10 10
                550 => {
                    if self.memory.contains_key(&line[1]) && self.memory.contains_key(&line[2]) {
                        if *self.memory.get(&line[1]).unwrap_or(&0) == 1 || *self.memory.get(&line[1]).unwrap_or(&0) == 1 {
                            self.memory.insert(line[0], 1);
                        } else {
                            self.memory.insert(line[0], 0);
                        }
                    } else {
                        panic!("в действии Or указана не опреденная ячейка {} или {}", line[1], line[2])
                    }
                }
                // 10 A 10 10
                551 => {
                    if self.memory.contains_key(&line[1]) && self.memory.contains_key(&line[2]) {
                        if *self.memory.get(&line[1]).unwrap_or(&0) == 1 && *self.memory.get(&line[1]).unwrap_or(&0) == 1 {
                            self.memory.insert(line[0], 1);
                        } else {
                            self.memory.insert(line[0], 0);
                        }
                    } else {
                        panic!("в действии And указана не опреденная ячейка {} или {}", line[1], line[2])
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
                        panic!("в действии Xor указана не опреденная ячейка {} или {}", line[1], line[2])
                    }
                }

                _ => {panic!("$$$$$$$$$$$$$$")}
            }
            pc += 1;
        }
    }
}



pub fn start(bytecode: Vec<Vec<i32>>) {
    let mut vm = VM::new();

    let program = pre_run::pre_run(bytecode);

    vm.run(program);
}