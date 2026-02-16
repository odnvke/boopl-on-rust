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

    pub fn load_p(&mut self, hash_map: HashMap<i32, String>) {
        self.id_to_name_static = hash_map;
    }
    
    pub fn load_pd(&mut self, hash_map: HashMap<i32, String>) {
        self.id_to_name_dynamic = hash_map;
    }
    
    pub fn load_n(&mut self, hash_map: HashMap<i32, String>) {
        self.id_to_name_var = hash_map;
    }

    pub fn get_name_p(&self, id: i32) -> String {
        match self.id_to_name_static.get(&id) {
            Some(name) => name.clone(),
            None => panic!("Имя статической метки с ID {} не найдено", id),
        }
    }

    pub fn get_name_pd(&self, id: i32) -> String {
        match self.id_to_name_dynamic.get(&id) {
            Some(name) => name.clone(),
            None => panic!("Имя динамической метки с ID {} не найдено", id),
        }
    }

    pub fn get_name_n(&self, id: i32) -> String {
        match self.id_to_name_var.get(&id) {
            Some(name) => name.clone(),
            None => panic!("Имя переменной с ID {} не найдено", id),
        }
    }
}