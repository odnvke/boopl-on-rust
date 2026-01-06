use std::collections::{HashMap};

use super::pre_run;

struct VM {
    memory: HashMap<i32, u8>,
    memory_pd: HashMap<i32, u32>,
    pc: usize,
    bytecode: Vec<Vec<i32>>,
}

impl VM {
    fn new() -> Self {
        VM {
            memory: HashMap::new(),
            memory_pd: HashMap::new(),
            pc: 0,
            bytecode: Vec::new()
        }
    }
    
    fn run(&mut self, program: Vec<Vec<i32>>) {
        self.bytecode = program;
        
        loop {
            match self.bytecode[self.pc][0] {
                50 => {}
                _ => {panic!("несуществующий опкод")}
            }
        }
    }
}



pub fn start(bytecode: Vec<Vec<i32>>) {
    let mut vm = VM::new();

    let program = pre_run::pre_run(bytecode);

    vm.run(program);
}