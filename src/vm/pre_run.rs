use std::collections::{HashMap};

pub fn pre_run(mut program: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut memory_p: HashMap<i32, i32> = HashMap::new();
    let mut n_pointer = 0;
    let mut counter = 0;
    let mut new_program: Vec<Vec<i32>> = Vec::new();

    for i 
    in program.iter() {
        counter += 1;
        // для P.10
        if i[0] == 200 {
            // если ещё не обьявлен, обьявлаем
            if !memory_p.contains_key(&i[1]) {
                n_pointer += 1;
                memory_p.insert(i[1], counter-n_pointer-1);
            } 
            // иначе ошибка
            else {panic!("   >>  ! переобьявление статического указателя, {:?}", i)}
        }
    }

    for i in program.iter_mut() {
        // для  G P.10
        if i[0] == 230 {
            if !memory_p.contains_key(&i[1]) {
                panic!("   >>  ! попытка перейти по не определённому указателю: {}; в строке: {:?}", i[1], i)
            } else {
                println!("замена: {} на {}", i[1], memory_p[&i[1]]);
                i[1] = memory_p[&i[1]];
            }
        }
        // для  PD.10 P.10
        else if i[0] == 260 {
            if !memory_p.contains_key(&i[2]) {
                panic!("   >>  ! попытка присвоить значение неопределённого указателя: {}; динамическому: {:?}", i[2], i)
            } else {
                println!("замена: {} на {}", i[2], memory_p[&i[2]]);
                i[2] = memory_p[&i[2]];
            }
        }
        // для  IG 10 P.10
        else if i[0] == 302 {
            if !memory_p.contains_key(&i[2]) {
                panic!("   >>  ! попытка перейти по не определённому указателю после if: {}; в строке: {:?}", i[2], i)
            } else {
                println!("замена: {} на {}", i[2], memory_p[&i[2]]);
                i[2] = memory_p[&i[2]];
            }
        }

        if i[0] != 200 {
            new_program.push(i.to_vec());
        }
    }   
    
    // находим нужные end`ы для каждого if`а
    loop {
        let mut all_if_replaced = true;
        for i in 0..new_program.len() {
            let mut level = 1;

            // для  I 10
            if new_program[i][0] == 300 && new_program[i].len() == 2 {
                all_if_replaced = false;
                let mut pointer: i32 = 0;

                for i2 in i+1..new_program.len() {
                    if new_program[i2][0] == 300 {level += 1;}    
                    else if new_program[i2][0] == 50 || new_program[i2][0] == 51 {
                        level -= 1;
                        if level >= 0 && new_program[i2].len() == 1 {new_program[i2][0] = 51}
                    }
                    
                    if level == 0 {pointer = i2 as i32; break;}
                }

                if level > 0 {
                    panic!("   >>  ! не найдаен end для if: I {}", new_program[i][1])
                } else {
                    new_program[i].push(pointer);
                }       
            }
        }
        if all_if_replaced {break}
    }

    new_program
}