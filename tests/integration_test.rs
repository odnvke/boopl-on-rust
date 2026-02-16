use std::fs;
use std::path::Path;
use std::process::Command;

const GOLDEN_DIR: &str = "tests/golden";

// Путь к уже собранному бинарю
fn get_binary_path() -> String {
    // Для Windows:
    "target/debug/leng1.exe".to_string()
    // Для Linux/Mac:
    // "target/debug/leng1".to_string()
}

#[test]
fn test_golden_valid() {
    // Сначала собираем (один раз)
    let build = Command::new("cargo")
        .args(&["build"])
        .output()
        .expect("Не удалось собрать");
    
    assert!(build.status.success(), "Сборка не удалась: {}", 
        String::from_utf8_lossy(&build.stderr));

    let valid_dir = Path::new(GOLDEN_DIR).join("valid");
    fs::create_dir_all(&valid_dir).unwrap();
    
    for entry in fs::read_dir(&valid_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) != Some("bpl") {
            continue;
        }
        
        let test_name = path.file_stem().unwrap().to_str().unwrap();
        let expected_path = path.with_extension("stdout");
        
        println!("Тест: {}", test_name);
        
        // Используем готовый бинарь, не cargo run!
        let output = Command::new(get_binary_path())
            .arg(path.to_str().unwrap())
            .output()
            .expect("Не удалось запустить");
        
        let actual_stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let actual_stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        // Проверяем что нет ошибок компиляции
        assert!(output.status.success() && actual_stderr.is_empty(),
            "Тест {} упал с ошибкой:\n{}", test_name, actual_stderr);
        
        // Сравниваем или создаём эталон
        if expected_path.exists() {
            let expected = fs::read_to_string(&expected_path).unwrap();
            let expected = expected.replace("\r\n", "\n");  // Windows -> Unix
            let actual = actual_stdout.replace("\r\n", "\n");  // На всякий случай

            assert_eq!(actual, expected, "Вывод теста {} не совпадает!", test_name);
        } else {
            fs::write(&expected_path, &actual_stdout).unwrap();
            println!("Создан эталон: {:?}", expected_path);
        }
    }
}

#[test]
fn test_golden_errors() {
    // Аналогично, но проверяем что программа падает
    let build = Command::new("cargo")
        .args(&["build"])
        .output()
        .expect("Не удалось собрать");
    
    assert!(build.status.success());

    let error_dir = Path::new(GOLDEN_DIR).join("error");
    fs::create_dir_all(&error_dir).unwrap();
    
    for entry in fs::read_dir(&error_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) != Some("bpl") {
            continue;
        }
        
        let test_name = path.file_stem().unwrap().to_str().unwrap();
        
        let output = Command::new(get_binary_path())
            .arg(path.to_str().unwrap())
            .output()
            .unwrap();
        
        assert!(!output.status.success(), 
            "Тест {} должен был упасть", test_name);
    }
}