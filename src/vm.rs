use std::collections::HashMap;

struct VM {
    memory: HashMap<i32, u8>,
    memory_p: HashMap<i32, u32>,
    memory_pd: HashMap<i32, u32>,
    pc: usize,
    level: i32,
    program: Vec<Vec<i32>>,
    bytecode: Vec<i32>,
}

impl VM {
    fn new(program: Vec<Vec<i32>>) -> Self {
        VM {
            memory: HashMap::new(),
            memory_p: HashMap::new(),
            memory_pd: HashMap::new(),
            pc: 0,
            level: 0,
            program,
            bytecode: Vec::new()
        }
    }
    
    fn run(&mut self) {
        self.pre_run();
    }

    fn pre_run(&mut self) {
        let mut n_pointer = 0;
        let mut counter = 0;

        for i 
        in self.program.iter() {
            counter += 1;
            // для P.10
            if i[0] == 200 {
                // если ещё не обьявлен, обьявлаем
                if !self.memory_p.contains_key(&i[1]) {
                    n_pointer += 1;
                    self.memory_p.insert(i[1], counter-n_pointer);
                } 
                // иначе ошибка
                else {panic!("   >>  ! переобьявление статического указателя, {:?}", i)}
            }
        }

        for i in self.program.iter() {
            // для  G P.10
            if i[0] == 230 {
                if !self.memory_p.contains_key(&i[1]) {
                    panic!("   >>  ! попытка перейти по не определённому указателю: {}; в строке: {:?}", i[1], i)
                }
            }
            // для  PD.10 P.10
            if i[0] == 260 {
                if !self.memory_p.contains_key(&i[1]) {
                    panic!("   >>  ! попытка присвоить значение неопределённого указателя: {}; динамическому: {:?}", i[1], i)
                }
            }

        }
    }
}

pub fn start(bytecode: Vec<Vec<i32>>) {
    let mut vm = VM::new(bytecode);
    vm.run();
}