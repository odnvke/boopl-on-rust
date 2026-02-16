use std::collections::HashMap;
use crate::name_map::IdentNameMap;

pub fn pre_run(mut program: Vec<(Vec<i32>, i32)>, ident_name_map: &IdentNameMap) -> (Vec<Vec<i32>>, Vec<i32>) {
    let mut memory_p: HashMap<i32, i32> = HashMap::new();
    let mut n_pointer = 0;
    let mut counter = 0;
    let mut new_program: Vec<Vec<i32>> = Vec::new();
    let mut lines_n: Vec<i32> = Vec::new();

    for (instr, line_num) in program.iter() {
        counter += 1;
        // для P.10
        if instr[0] == 200 {
            // если ещё не объявлен, объявляем
            if !memory_p.contains_key(&instr[1]) {
                n_pointer += 1;
                memory_p.insert(instr[1], counter - n_pointer - 1);
            } 
            // иначе ошибка
            else {
                eprintln!("\n ! пре ран-тайм\n\n   >>  ! переобъявление статического указателя: P.{}  ({})\n\n",
                    ident_name_map.get_name_p(instr[1]), line_num); 
                std::process::exit(1);
            }
        }
    }

    for (instr, line_num) in program.iter_mut() {
        // для G P.10
        if instr[0] == 230 {
            if !memory_p.contains_key(&instr[1]) {
                eprintln!("\n ! пре ран-тайм\n\n   >>  ! попытка перейти по не определённому указателю: P.{}; в строке: {:?}  ({})\n\n",
                         ident_name_map.get_name_p(instr[1]), instr, line_num);
                std::process::exit(1);
            } else {
                instr[1] = memory_p[&instr[1]];
            }
        }
        // для PD.10 P.10
        else if instr[0] == 260 {
            if !memory_p.contains_key(&instr[2]) {
                eprintln!("\n ! пре ран-тайм\n\n   >>  ! попытка присвоить значение неопределённого указателя: P.{}; динамическому: {:?}  ({})\n\n",
                        ident_name_map.get_name_p(instr[2]), instr, line_num);
                std::process::exit(1);
            } else {
                instr[2] = memory_p[&instr[2]];
            }
        }
        // для IFG 10 P.10
        else if instr[0] == 302 {
            if !memory_p.contains_key(&instr[2]) {
                eprintln!("\n ! пре ран-тайм\n\n   >>  ! попытка перейти по неопределённому указателю после if: P.{}; в строке: {:?}  ({})\n\n",
                        ident_name_map.get_name_p(instr[2]), instr, line_num);
                std::process::exit(1);
            } else {
                instr[2] = memory_p[&instr[2]];
            }
        }

        if instr[0] != 200 {
            new_program.push(instr.to_vec());
            lines_n.push(*line_num);
        }
    }   
    
    // находим нужные end`ы для каждого if`а
    loop {
        let mut all_if_replaced = true;
        for i in 0..new_program.len() {
            let mut level = 1;

            // для IF 10
            if new_program[i][0] == 300 && new_program[i].len() == 2 {
                all_if_replaced = false;
                let mut pointer: i32 = 0;

                for j in i+1..new_program.len() {
                    if new_program[j][0] == 300 {
                        level += 1;
                    } else if new_program[j][0] == 50 || new_program[j][0] == 51 {
                        level -= 1;
                        if level >= 0 && new_program[j].len() == 1 {
                            new_program[j][0] = 51;
                        }
                    }
                    
                    if level == 0 {
                        pointer = j as i32;
                        break;
                    }
                }

                if level > 0 {
                    let (_, line_num) = program[i];
                    eprintln!("\n ! пре ран-тайм\n\n   >>  ! не найден end для if: IF {}  ({})\n\n",
                         ident_name_map.get_name_n(new_program[i][1]), line_num);
                    std::process::exit(1);
                } else {
                    new_program[i].push(pointer);
                }       
            }
        }
        if all_if_replaced {
            break;
        }
    }

    (new_program, lines_n)
}