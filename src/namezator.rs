// namezator.rs
use std::collections::{HashMap, BTreeMap};
use crate::{name_map::IdentNameMap, tokens::{RawToken, Token}};

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
        
        *self.base_ids.get(path).unwrap()
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
    
    if name.is_empty() {
        return parts;
    }
    
    // Разбиваем с сохранением пустых частей как отдельных элементов
    let segments: Vec<&str> = name.split('_').collect();
    
    for (i, segment) in segments.iter().enumerate() {
        if segment.is_empty() {
            // Пустая часть означает двойное подчеркивание или подчеркивание в начале/конце
            parts.push(NamePart::Text("_".to_string()));
        } else if let Ok(num) = segment.parse::<i32>() {
            parts.push(NamePart::Number(num));
        } else {
            parts.push(NamePart::Text(segment.to_string()));
        }
        
        // Добавляем разделитель между частями (кроме последней)
        if i < segments.len() - 1 {
            parts.push(NamePart::Text("_".to_string()));
        }
    }
    
    parts
}

fn extract_names(raw_tokens: &[Vec<RawToken>], token_type: &RawToken) -> Vec<String> {
    let mut unique_names = Vec::new();
    
    for line in raw_tokens {
        for token in line {
            match (token, token_type) {
                (RawToken::Number(name, _), RawToken::Number(_, _)) |
                (RawToken::LabelP(name, _), RawToken::LabelP(_, _)) |
                (RawToken::LabelPD(name, _), RawToken::LabelPD(_, _)) => {
                    let name_clone = name.clone();
                    if !unique_names.contains(&name_clone) {
                        unique_names.push(name_clone);
                    }
                },
                _ => {}
            }
        }
    }
    
    unique_names
}

// Основная функция
pub fn namezating(raw_tokens: Vec<Vec<RawToken>>, debug_mode: bool) -> (Vec<Vec<Token>>, IdentNameMap) {
    let unique_names_p = extract_names(&raw_tokens, &RawToken::LabelP(String::new(), 0));
    let unique_names_pd = extract_names(&raw_tokens, &RawToken::LabelPD(String::new(), 0));
    let unique_names_n = extract_names(&raw_tokens, &RawToken::Number(String::new(), 0));
    
    let mut tree_p = TreeNode::default();
    let mut tree_pd = TreeNode::default();
    let mut tree_n = TreeNode::default();

    for name in &unique_names_p {
        tree_p.insert(name);
    }
    for name in &unique_names_pd {
        tree_pd.insert(name);
    }
    for name in &unique_names_n {
        tree_n.insert(name);
    }

    let mut name_to_id_p: HashMap<String, i32> = HashMap::new();
    let mut name_to_id_pd: HashMap<String, i32> = HashMap::new();
    let mut name_to_id_n: HashMap<String, i32> = HashMap::new();
    
    // Создаем отдельные контексты для каждого типа
    let mut context_p = NumberingContext::new(1);
    let mut context_pd = NumberingContext::new(1);
    let mut context_n = NumberingContext::new(1);
    
    let mut current_path = Vec::new();
    
    tree_p.assign_ids(&mut name_to_id_p, &mut context_p, &mut current_path);
    current_path.clear();
    tree_pd.assign_ids(&mut name_to_id_pd, &mut context_pd, &mut current_path);
    current_path.clear();
    tree_n.assign_ids(&mut name_to_id_n, &mut context_n, &mut current_path);

    if debug_mode {
        println!("=~=~=~=~=~ Таблица имен =~=~=~=~=~");
        println!(" Labels Static:");
        // Собираем владеющие значения
        let mut sorted_entries: Vec<(String, i32)> = name_to_id_p
            .iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();

        sorted_entries.sort_by_key(|&(_, id)| id);
        
        for (name, id) in &sorted_entries {
            println!("{:30} → {}", name, id);
        }

        println!(" Labels Dynamic:");
        // Собираем владеющие значения
        let mut sorted_entries: Vec<(String, i32)> = name_to_id_pd
            .iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();

        sorted_entries.sort_by_key(|&(_, id)| id);
        
        for (name, id) in &sorted_entries {
            println!("{:30} → {}", name, id);
        }
        
        println!(" Vars:");
        // Собираем владеющие значения
        let mut sorted_entries: Vec<(String, i32)> = name_to_id_n
            .iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();

        sorted_entries.sort_by_key(|&(_, id)| id);
        
        for (name, id) in &sorted_entries {
            println!("{:30} → {}", name, id);
        }
        println!("=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~\n");
    }
    
    let mut result = Vec::new();
    
    for line in raw_tokens {
        let mut converted_line = Vec::new();
        
        // Используем ссылку на токен в match
        for token in line {
            match &token {  // Берем ссылку
                RawToken::Bool(b, l_n) => converted_line.push(Token::Bool(*b, *l_n)),
                RawToken::Keyword(k, l_n) => converted_line.push(Token::Keyword(k.clone(), *l_n)),
                RawToken::Number(name, l_n) => {
                    let id = *name_to_id_n.get(name)
                        .unwrap_or_else(|| panic!("Имя переменной не найдено: {}", name));
                    converted_line.push(Token::Number(id, *l_n));
                }
                RawToken::LabelP(name, l_n) => {
                    let id = *name_to_id_p.get(name)
                        .unwrap_or_else(|| panic!("Статическая метка не найдена: {}", name));
                    converted_line.push(Token::LabelP(id, *l_n));
                }
                RawToken::LabelPD(name, l_n) => {
                    let id = *name_to_id_pd.get(name)
                        .unwrap_or_else(|| panic!("Динамическая метка не найдена: {}", name));
                    converted_line.push(Token::LabelPD(id, *l_n));
                }
            }
        }
        
        if !converted_line.is_empty() {
            result.push(converted_line);
        }
    }
    

    let mut ident_name_map = IdentNameMap::new(); 

    let mut id_to_name: HashMap<i32, String> = HashMap::new();
    for (name, id) in name_to_id_p {
        id_to_name.insert(id, name);
    }
    ident_name_map.load_p(id_to_name.clone());

    // Очищаем и заполняем для PD
    id_to_name.clear();
    for (name, id) in name_to_id_pd {
        id_to_name.insert(id, name);
    }
    ident_name_map.load_pd(id_to_name.clone());

    // Очищаем и заполняем для N
    id_to_name.clear();
    for (name, id) in name_to_id_n {
        id_to_name.insert(id, name);
    }
    ident_name_map.load_n(id_to_name);

    (result, ident_name_map)
}