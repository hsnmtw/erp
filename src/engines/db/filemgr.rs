pub enum fs_type {
    DIRECTORY,
    FILE,
    ANY
}

pub fn list_fs(pth: &str, fstype : &fs_type) -> Vec<String> {
    let mut folders = Vec::new();
    match std::fs::read_dir(pth) {
        Ok(entries) => {
            for entry_result in entries {
                if let Ok(entry) = entry_result {
                    if let Ok(metadata) = entry.metadata() {
                        match fstype {
                            &fs_type::DIRECTORY => {
                                if metadata.is_dir() {
                                    let path: String = entry.file_name().into_string().unwrap();
                                    folders.push(path);
                                }
                            },
                            &fs_type::FILE => {
                                if metadata.is_file() {
                                    let path: String = entry.file_name().into_string().unwrap();
                                    folders.push(path);
                                }
                            },
                            _ => {
                                let path: String = entry.file_name().into_string().unwrap();
                                folders.push(path);
                            }
                        };
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
        }
    }
    folders
}