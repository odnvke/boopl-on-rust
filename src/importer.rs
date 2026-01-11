use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use crate::tokens::{RawToken, start as tokenize};

pub fn importing(tokens: Vec<Vec<RawToken>>, base_path: &Path) -> Result<Vec<Vec<RawToken>>, String> {
    let mut processed_files = HashSet::new();
    process_imports(tokens, base_path, &mut processed_files)
}

fn process_imports(
    tokens: Vec<Vec<RawToken>>,
    base_path: &Path,
    processed_files: &mut HashSet<PathBuf>
) -> Result<Vec<Vec<RawToken>>, String> {
    let mut result: Vec<Vec<RawToken>> = Vec::new();
    
    for line_tokens in tokens {
        match line_tokens.as_slice() {
            // Корректный IMPORT с именем файла
            [RawToken::Keyword(s, l_n), RawToken::Number(filename, _)] if s == "IMPORT" => {
                let filename_str = filename.clone();
                // 1. Находим файл
                let file_path = find_file(&filename_str, base_path)
                    .ok_or_else(|| format!("Строка {}: Файл '{}' не найден", l_n, filename_str))?;
                // 2. Проверяем, не обрабатывали ли уже этот файл
                let canonical_path = file_path.canonicalize().unwrap_or(file_path.clone());
                if processed_files.contains(&canonical_path) {
                    // Пропускаем уже обработанный файл (не ошибка!)
                    //println!("Файл '{}' уже был импортирован, пропускаем", filename_str);
                    continue;
                }
                // 3. Добавляем в обработанные
                processed_files.insert(canonical_path.clone());
                
                // 4. Читаем и обрабатываем файл
                let content = fs::read_to_string(&file_path)
                    .map_err(|e| format!("Строка {}: Не удалось прочитать файл '{}': {}", 
                                         l_n, filename_str, e))?;
                
                let imported_tokens = tokenize(content)
                    .map_err(|(e, line)| format!("Строка {} в файле '{}': {}", 
                                                 line, filename_str, e))?;
                // 5. Рекурсивно обрабатываем импорты в этом файле
                let parent_dir = file_path.parent().unwrap_or(base_path);
                let processed = process_imports(imported_tokens, parent_dir, processed_files)?;
                
                result.extend(processed);
                //result.push(vec![RawToken::Keyword("E".to_string(), 100)]);
            }
            
            _ => {
                result.push(line_tokens);
            }
        }
    }
    
    Ok(result)
}

fn find_file(filename: &str, base_path: &Path) -> Option<PathBuf> {
    let search_dirs = vec![
        base_path.to_path_buf(),
        base_path.join("lib"),
        base_path.join("libs"),
    ];
    
    for dir in search_dirs {
        let possibilities = vec![
            dir.join(filename),
            dir.join(format!("{}.bpl", filename)),
            dir.join(format!("{}.txt", filename)),
        ];
        
        for path in possibilities {
            if path.exists() {
                return Some(path);
            }
        }
    }
    
    None
}