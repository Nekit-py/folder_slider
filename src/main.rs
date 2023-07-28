use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::prelude::{DateTime, Utc};
// use std::error::Error;
use std::fs::{self, ReadDir, File};
use std::path::{PathBuf, Path};
use std::io;

///SystemTime в человекочетаемый вид
fn iso8601(st: std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.into();
    format!("{}", dt.format("%d.%m.%Y %H:%M:%S"))
}

///Получаем имя файла из пути
fn get_file_name(path: &Path) -> String {
    let filename = match path.file_name() {
        Some(name) => name.to_str().unwrap(),
        None => panic!("Тут нет файла"),
    };
    filename.to_owned()
}

///Строковое представление размера файла
#[inline]
fn file_size(size: f32) -> String {
    let devider = 1024_f32;
    let mut f_size = format!("{:.2} КБ", size / devider);

    if size > devider.powf(2.0) {
        f_size = format!("{:.2} МБ", size / devider.powf(2.0))
    } else if size > devider.powf(3.0) {
        f_size = format!("{:.2} ГБ", size / devider.powf(3.0))
    }
    f_size
}

fn file_metadata(path: &PathBuf) -> std::io::Result<FileMetadata> {
    let metadata = fs::metadata(path)?;
    let modified = match metadata.modified() {
        Ok(st) => st,
        Err(_) => {
            println!("Файл не был модифицирован");
            std::time::SystemTime::now()
        }
    };

    let created = match metadata.created() {
        Ok(st) => st,
        Err(_) => {
            println!("Время создания не известно.");
            std::time::SystemTime::now()
        }
    };

    let size = file_size(metadata.len() as f32);
    let name = get_file_name(path);

    Ok(FileMetadata {
        name,
        created: iso8601(created),
        size,
        modified: iso8601(modified),
        path: path.clone().into_os_string().into_string().unwrap(),
    })
}

struct FileMetadata {
    name: String,
    created: String,
    size: String,
    modified: String,
    path: String,
}

impl FileMetadata {
    fn to_string(&self) -> String {
        format!(
            "Файл: {}
    - Имя: {}
    - Размер: {}
    - Создан: {}
    - Модифицирован: {}",
            { self.path.clone() },
            { self.name.clone() },
            { self.size.clone() },
            { self.created.clone() },
            { self.modified.clone() }
        )
    }
}

fn scan_dir(path: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();

    Ok(entries)
}

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
    let files = scan_dir(dir);
    match files {
        Ok(vec_of_path) => {
            for file in vec_of_path {
                if file.is_file() {
                    write_to_file(file_metadata(&file).unwrap().to_string())
                }
                else if file.is_dir() {
                   folder_walk(&file) ;
                }
            }
        },
        Err(e) => println!("Ошибка какая-то -> {}", e),
    };
}

fn main()  -> std::io::Result<()>{
    let _ = File::create("Данные о файлах.txt")?;
    let mut path = String::new();
    println!("Введите путь до директории: ");
    std::io::stdin().read_line(&mut path).unwrap();
    let dir = PathBuf::from(path.trim());
    if dir.is_dir() {
        println!("Начинаю записывать данные...");
        folder_walk(&dir);
        println!("Данны успешно записаны.");
    }
    else {
        println!("Не является папкой: {:#?}", dir.as_os_str());
    }
    Ok(())
}
