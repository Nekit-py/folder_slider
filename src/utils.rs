pub mod catalog {

    use std::fs;
    use std::io;
    use std::path::PathBuf;

    pub fn scan_dir(path: &PathBuf) -> io::Result<Vec<PathBuf>> {
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort();

        Ok(entries)
    }
}

pub mod common {
    use chrono::prelude::{DateTime, Utc};

    pub fn iso8601(st: std::time::SystemTime) -> String {
        let dt: DateTime<Utc> = st.into();
        format!("{}", dt.format("%d.%m.%Y %H:%M:%S"))
    }
}
