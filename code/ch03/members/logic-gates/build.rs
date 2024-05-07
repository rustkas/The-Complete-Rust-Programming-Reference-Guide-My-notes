use std::env;
use std::fs;
use std::process::Command;

fn main() {
    // Выполняем сборку проекта с помощью cargo doc
    let status = Command::new("cargo")
        .arg("doc")
        .status()
        .expect("Failed to execute cargo doc command");

    if status.success() {
        // Получаем путь к текущей директории
        let current_dir = env::current_dir().unwrap();

        // Составляем путь к папке assets
        let assets_dir = current_dir.join("assets");

        // Составляем путь к файлу, который нужно скопировать
        let file_to_copy = assets_dir.join("logic_gates_icon.png");

        // Составляем путь к папке, куда нужно скопировать файл
        let dest_dir = current_dir.join("target").join("doc").join("static.files");

        // Создаем папку, если она не существует
        if !dest_dir.exists() {
            fs::create_dir_all(&dest_dir).unwrap();
        }

        // Составляем путь к целевому файлу
        let dest_file = dest_dir.join("logic_gates_icon.png");

        // Копируем файл
        fs::copy(&file_to_copy, &dest_file).unwrap();
    } else {
        println!("Failed to generate documentation");
    }
}
