use std::collections::{HashMap};

use crate::name_map::IdentNameMap;

pub fn pre_run(mut program: Vec<(Vec<i32>, i32)>, i_n_m: &IdentNameMap) -> (Vec<Vec<i32>>, Vec<i32>) {
    let mut memory_p: HashMap<i32, i32> = HashMap::new();
    let mut n_pointer = 0;
    let mut counter = 0;
    let mut new_program: Vec<Vec<i32>> = Vec::new();
    let mut lines_n: Vec<i32> = Vec::new();


    for (i , line_n) in program.iter() {
        counter += 1;
        
        // для P.10
        if i[0] == 200 {
            // если ещё не обьявлен, обьявлаем
            if !memory_p.contains_key(&i[1]) {
                n_pointer += 1;
                memory_p.insert(i[1], counter-n_pointer-1);
            } 
            // иначе ошибка
            else {panic!("   >>  ! переобьявление статического указателя: P.{}  ({})", i_n_m.get_name(i[1]), line_n)}
        }
    }

    for (i , line_n) in program.iter_mut() {
        // для  G P.10
        if i[0] == 230 {
            if !memory_p.contains_key(&i[1]) {
                panic!("   >>  ! попытка перейти по не определённому указателю: P.{}; в строке: {:?}  ({})",
                         i_n_m.get_name(i[1]), i, line_n)
            } else {
                println!("замена: P.{} на номер строки {} ", i_n_m.get_name(i[1]), memory_p[&i[1]]);
                i[1] = memory_p[&i[1]];
            }
        }
        // для  PD.10 P.10
        else if i[0] == 260 {
            if !memory_p.contains_key(&i[2]) {
                panic!("   >>  ! попытка присвоить значение неопределённого указателя: P.{}; динамическому: {:?}  ({})",
                        i_n_m.get_name(i[2]), i, line_n)
            } else {
                println!("замена: P.{} на номер строки {}", i_n_m.get_name(i[2]), memory_p[&i[2]]);
                i[2] = memory_p[&i[2]];
            }
        }
        // для  IG 10 P.10
        else if i[0] == 302 {
            if !memory_p.contains_key(&i[2]) {
                panic!("   >>  ! попытка перейти по неопределённому указателю после if: P.{}; в строке: {:?}  ({})",
                        i_n_m.get_name(i[2]), i, line_n)
            } else {
                println!("замена: P.{} на номер строки {}", i_n_m.get_name(i[2]), memory_p[&i[2]]);
                i[2] = memory_p[&i[2]];
            }
        }

        if i[0] != 200 {
            new_program.push(i.to_vec());
            lines_n.push(*line_n);
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
                    let (_, line_n) = program[i];
                    panic!("   >>  ! не найдаен end для if: I P.{}  ({})", i_n_m.get_name(new_program[i][1]), line_n)
                } else {
                    new_program[i].push(pointer);
                }       
            }
        }
        if all_if_replaced {break}
    }

    (new_program, lines_n)
}