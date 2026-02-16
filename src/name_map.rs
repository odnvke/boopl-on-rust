use std::collections::{HashMap};

#[derive(Debug, Clone)]
pub struct IdentNameMap {
    id_to_name_static: HashMap<i32, String>,
    id_to_name_dynamic: HashMap<i32, String>,
    id_to_name_var: HashMap<i32, String>,
}

impl IdentNameMap {
    pub fn new() -> Self {
        IdentNameMap {
            id_to_name_static: HashMap::new(),
            id_to_name_dynamic: HashMap::new(),
            id_to_name_var: HashMap::new(),
        }
    }

    pub fn load_P(&mut self, _hash_map: HashMap<i32, String>) {
        self.id_to_name_static = _hash_map;
    }
    
    pub fn load_PD(&mut self, _hash_map: HashMap<i32, String>) {
        self.id_to_name_dynamic = _hash_map;
    }
    
    pub fn load_N(&mut self, _hash_map: HashMap<i32, String>) {
        self.id_to_name_var = _hash_map;
    }


    pub fn get_name_P(&self, id: i32) -> String {
        match self.id_to_name_static.get(&id) {
            Some(name) => name.clone(),
            None => {
                panic!("Имя статической метки с ID {} не найдено", id);
            }
        }
    }

    pub fn get_name_PD(&self, id: i32) -> String {
        match self.id_to_name_dynamic.get(&id) {
            Some(name) => name.clone(),
            None => {
                panic!("Имя динамической метки с ID {} не найдено", id);
            }
        }
    }

    pub fn get_name_N(&self, id: i32) -> String {
        match self.id_to_name_var.get(&id) {
            Some(name) => name.clone(),
            None => {
                panic!("Имя переменной с ID {} не найдено", id);
            }
        }
    }
}