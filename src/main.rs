use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

mod file_manager;
mod utils;

use file_manager::{dategroup, file_metadata};
use utils::catalog;

use clap::{Parser, ValueEnum};

fn write_to_file(content: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(r"Данные о файлах.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", content.as_str()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn folder_walk(dir: &PathBuf) {
    let files = catalog::scan_dir(dir);
    match files {
        Ok(vec_of_path) => {
            for file in vec_of_path {
                if file.is_file() {
                    write_to_file(file_metadata::file_metadata(&file).unwrap().to_string())
                } else if file.is_dir() {
                    folder_walk(&file);
                }
            }
        }
        Err(e) => println!("Ошибка какая-то -> {}", e),
    };
}

fn dir_report(path: String) -> std::io::Result<()> {
    let dir = PathBuf::from(path.trim());
    if dir.is_dir() {
        println!("Начинаю записывать данные...");
        folder_walk(&dir);
        println!("Данны успешно записаны.");
    } else {
        println!("Не является папкой: {:#?}", dir.as_os_str());
    }
    Ok(())
}

fn cataloging(path: String) -> std::io::Result<()> {
    let dir = PathBuf::from(path.trim());
    let _ = dategroup::date_cataloging(dir);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "FileManager")]
#[command(author = "Nekit S. <nekit-sns@yandex.ru>")]
#[command(version = "0.1")]
#[command(about = "Работа с файлами и каталогами", long_about = None)]
struct Args {
    #[arg(value_enum)]
    action: Action,
    #[arg(long)]
    path: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Action {
    Report,
    Cataloging,
}

fn main() -> std::io::Result<()> {
    let cli = Args::parse();
    //folder_slider report
    match cli.action {
        Action::Report => {
            dir_report(cli.path)?;
        }
        Action::Cataloging => cataloging(cli.path)?,
    }
    Ok(())
}
