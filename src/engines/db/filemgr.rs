#[derive(PartialEq)]
#[allow(unused)]
pub enum FsType {
    FLDR,
    FILE,
    BOTH
}

pub fn list_fs(pth: &str, fstype : &FsType) -> Vec<String> {
    let mut folders = Vec::new();
    match std::fs::read_dir(pth) {
        Ok(entries) => {
            for entry_result in entries {
                if let Ok(entry) = entry_result && let Ok(metadata) = entry.metadata() {
                    let path: String = entry.file_name().into_string().unwrap();
                    match fstype {
                        &FsType::FLDR if metadata.is_dir()                       => folders.push(path),
                        &FsType::FILE if metadata.is_file()                      => folders.push(path),
                        &FsType::BOTH if metadata.is_dir() || metadata.is_file() => folders.push(path),
                        _ => {}
                    };
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
        }
    }
    folders
}