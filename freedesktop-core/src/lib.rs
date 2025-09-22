pub mod info;
use std::path::PathBuf;

/// The base directories all other searches are
/// based on. Data comes from XDG_DATA_DIRS
pub fn base_directories() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();

    if let Ok(var_str) = std::env::var("XDG_DATA_DIRS") {
        for p in var_str.split(":") {
            let pb = PathBuf::from(p);

            if pb.exists() {
                dirs.push(pb);
            }
        }
    }

    if let Ok(var_str) = std::env::var("XDG_DATA_HOME") {
        let pb = PathBuf::from(var_str);

        if pb.exists() {
            dirs.push(pb);
        }
    }

    dirs
}

pub fn config_dir() -> PathBuf {
    let Ok(dir) = std::env::var("XDG_CONFIG_HOME") else {
        let home = std::env::var("HOME").expect("CRITICAL: $HOME variable not set or available");
        return PathBuf::from(home).join(".config");
    };

    PathBuf::from(dir)
}
