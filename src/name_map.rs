use std::collections::{HashMap};

#[derive(Debug, Clone)]
pub struct IdentNameMap {
    id_to_name: HashMap<i32, String>,
}

impl IdentNameMap {
    pub fn new() -> Self {
        IdentNameMap {
            id_to_name: HashMap::new(),
        }
    }

    pub fn load(&mut self, _hash_map: HashMap<i32, String>) {
        self.id_to_name = _hash_map;
    }
    
pub fn get_name(&self, id: i32) -> String {
        match self.id_to_name.get(&id) {
            Some(name) => name.clone(),
            None => {
                panic!("Имя с ID {} не найдено", id);
            }
        }
    }
}