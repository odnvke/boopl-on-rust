// namezator.rs
use std::collections::{HashMap, BTreeMap};
use crate::{name_map::{IdentNameMap}, tokens::{RawToken, Token}};

// Часть имени (текст или число)
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum NamePart {
    Text(String),
    Number(i32),
}

// Узел дерева
#[derive(Debug, Default)]
struct TreeNode {
    children: BTreeMap<NamePart, TreeNode>,
    full_names: Vec<String>,
}

// Контекст нумерации
struct NumberingContext {
    base_ids: HashMap<Vec<NamePart>, i32>,
    local_counters: HashMap<Vec<NamePart>, i32>,
    next_base_id: i32,
    block_size: i32,
}

impl NumberingContext {
    fn new(block_size: i32) -> Self {
        Self {
            base_ids: HashMap::new(),
            local_counters: HashMap::new(),
            next_base_id: 0,
            block_size,
        }
    }
    
    fn get_base_id(&mut self, path: &[NamePart]) -> i32 {
        let key = path.to_vec();
        
        if !self.base_ids.contains_key(&key) {
            self.base_ids.insert(key.clone(), self.next_base_id);
            self.local_counters.insert(key, 0);
            self.next_base_id += self.block_size;
        }
        
        self.base_ids[path]
    }
    
    fn get_next_local_id(&mut self, path: &[NamePart]) -> i32 {
        let key = path.to_vec();
        let counter = self.local_counters.get_mut(&key).unwrap();
        let local_id = *counter;
        *counter += 1;
        local_id
    }
}

impl TreeNode {
    fn insert(&mut self, name: &str) {
        let parts = parse_name(name);
        self.insert_parts(&parts, name);
    }
    
    fn insert_parts(&mut self, parts: &[NamePart], original_name: &str) {
        if parts.is_empty() {
            self.full_names.push(original_name.to_string());
            return;
        }
        
        let first = parts[0].clone();
        let child = self.children.entry(first)
            .or_insert_with(TreeNode::default);
        
        child.insert_parts(&parts[1..], original_name);
    }
    
    fn assign_ids(
        &self,
        name_to_id: &mut HashMap<String, i32>,
        context: &mut NumberingContext,
        current_path: &mut Vec<NamePart>,
    ) {
        for (part, child) in &self.children {
            current_path.push(part.clone());
            child.assign_ids(name_to_id, context, current_path);
            current_path.pop();
        }
        
        if !self.full_names.is_empty() {
            let mut sorted_names = self.full_names.clone();
            sorted_names.sort();
            
            let base_id = context.get_base_id(current_path);
            
            for name in sorted_names {
                let local_id = context.get_next_local_id(current_path);
                let id = base_id + local_id;
                name_to_id.insert(name, id);
            }
        }
    }
}

fn parse_name(name: &str) -> Vec<NamePart> {
    let mut parts = Vec::new();
    
    for part in name.split('_') {
        if part.is_empty() {
            continue;
        }
        
        if let Ok(num) = part.parse::<i32>() {
            parts.push(NamePart::Number(num));
        } else {
            parts.push(NamePart::Text(part.to_string()));
        }
    }
    
    parts
}

// Исправленная функция extract_names
fn extract_names(raw_tokens: &[Vec<RawToken>]) -> Vec<String> {
    let mut unique_names = Vec::new();
    
    for line in raw_tokens {
        for token in line {
            match token {
                RawToken::Number(name, l_n) |
                RawToken::LabelP(name, l_n) |
                RawToken::LabelPD(name, l_n) => {
                    let name_clone = name.clone();
                    if !unique_names.contains(&name_clone) {
                        unique_names.push(name_clone);
                    }
                }
                _ => {}
            }
        }
    }
    
    unique_names
}

// Основная функция
pub fn namezating(raw_tokens: Vec<Vec<RawToken>>) -> (Vec<Vec<Token>>, IdentNameMap) {
    let unique_names = extract_names(&raw_tokens);
    
    let mut tree = TreeNode::default();
    for name in &unique_names {
        tree.insert(name);
    }
    
    let mut name_to_id = HashMap::new();
    let mut context = NumberingContext::new(1);
    let mut current_path = Vec::new();
    
    tree.assign_ids(&mut name_to_id, &mut context, &mut current_path);
    
    println!("=~=~=~=~=~ Таблица имен =~=~=~=~=~");
    
    // Собираем владеющие значения
    let mut sorted_entries: Vec<(String, i32)> = name_to_id
        .iter()
        .map(|(k, &v)| (k.clone(), v))
        .collect();

    sorted_entries.sort_by_key(|(_, id)| *id);
    
    for (name, id) in &sorted_entries {
        println!("{:30} → {}", name, id);
    }
    println!("=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~\n");
    
    // ИСПРАВЛЕННАЯ часть преобразования токенов
    let mut result = Vec::new();
    
    for line in raw_tokens {
        let mut converted_line = Vec::new();
        
        // Используем ссылку на токен в match
        for token in line {
            match &token {  // Берем ссылку
                RawToken::Bool(b, l_n) => converted_line.push(Token::Bool(*b, *l_n)),
                RawToken::Keyword(k, l_n) => converted_line.push(Token::Keyword(k.clone(), *l_n)),
                RawToken::Number(name, l_n) => {
                    let id = *name_to_id.get(name)
                        .unwrap_or_else(|| panic!("Имя не найдено: {}", name));
                    converted_line.push(Token::Number(id, *l_n));
                }
                RawToken::LabelP(name, l_n) => {
                    let id = *name_to_id.get(name)
                        .unwrap_or_else(|| panic!("Указатель P не найдено: {}", name));
                    converted_line.push(Token::LabelP(id, *l_n));
                }
                RawToken::LabelPD(name, l_n) => {
                    let id = *name_to_id.get(name)
                        .unwrap_or_else(|| panic!("Указатель PD не найдено: {}", name));
                    converted_line.push(Token::LabelPD(id, *l_n));
                }
            }
        }
        
        if !converted_line.is_empty() {
            result.push(converted_line);
        }
    }
    
    let mut id_to_name: HashMap<i32, String> = HashMap::new();
    for (v, k) in name_to_id {
        id_to_name.insert(k, v);
    }

    let mut ident_name_map = IdentNameMap::new(); 

    ident_name_map.load(id_to_name);

    (result, ident_name_map)
}