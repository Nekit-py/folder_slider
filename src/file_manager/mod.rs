pub mod dategroup {
    use super::file_metadata;
    use crate::utils::catalog;
    use std::fs;
    use std::io::Error;
    use std::path::PathBuf;

    pub fn date_cataloging(dir: PathBuf) -> Result<(), Error> {
        if dir.exists() {
            for d in catalog::scan_dir(&dir).unwrap() {
                if !d.is_dir() {
                    let md = file_metadata::file_metadata(&d).unwrap();
                    //Вычленяем дату создания файла для создания соответствующей директории
                    let date = md.modified.split_once(' ').unwrap().0;
                    let date_path = dir.join(date);
                    if !date_path.exists() {
                        //Создаем каталоги с датами
                        create_catalog(&date_path)?;
                    }
                    //Если у файла есть расширение
                    if let Some(ext) = d.extension() {
                        let ext: PathBuf = <&std::ffi::OsStr as Into<PathBuf>>::into(ext);
                        let extension_dir = date_path.join(ext);
                        if !extension_dir.exists() {
                            create_catalog(&extension_dir)?;
                        }
                        let move_from = dir.join(d.clone());
                        let move_to = extension_dir.join(md.name);
                        move_file(move_from, move_to)?;
                    }
                }
            }
        }
        println!("Катологизация файлов по дате созданию и расширениям произведена успешно.");
        Ok(())
    }

    //Создаем папку с датой создания папки
    fn create_catalog(dir_name: &PathBuf) -> Result<(), Error> {
        fs::create_dir(dir_name)?;
        Ok(())
    }

    // Пермещение файла
    fn move_file(from: PathBuf, to: PathBuf) -> std::io::Result<()> {
        fs::rename(from, to)?;
        Ok(())
    }
}

pub mod file_metadata {
    use crate::utils::common;
    use std::fmt;
    use std::fs;
    use std::path::{Path, PathBuf};

    ///Получаем имя файла из пути
    pub fn get_file_name(path: &Path) -> String {
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

    pub fn file_metadata(path: &PathBuf) -> std::io::Result<FileMetadata> {
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
            created: common::iso8601(created),
            size,
            modified: common::iso8601(modified),
            path: path.clone().into_os_string().into_string().unwrap(),
        })
    }

    pub struct FileMetadata {
        pub name: String,
        pub created: String,
        pub size: String,
        pub modified: String,
        pub path: String,
    }

    //https://rust-lang.github.io/rust-clippy/master/index.html#/inherent_to_string
    impl fmt::Display for FileMetadata {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Файл: {}
        - Имя: {}
        - Размер: {}
        - Создан: {}
        - Модифицирован: {}",
                { self.path.clone().as_str() },
                { self.name.clone().as_str() },
                { self.size.clone().as_str() },
                { self.created.clone().as_str() },
                { self.modified.clone().as_str() }
            )
        }
    }
}
