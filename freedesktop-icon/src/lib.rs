use ini::Ini;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

#[derive(Default)]
pub struct IconTheme {
    name: String,
}

impl IconTheme {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn paths(&self) -> Vec<PathBuf> {
        freedesktop_core::base_directories()
            .iter()
            .map(|path| path.join("icons").join(&self.name))
            .filter(|path| path.exists())
            .collect()
    }

    pub fn sizes(&self) -> HashSet<String> {
        let mut sizes: HashSet<String> = HashSet::new();
        for p in &self.paths() {
            let Ok(size_dirs) = std::fs::read_dir(&p) else {
                return sizes;
            };

            for s in size_dirs
                .filter_map(|s| s.ok())
                .filter(|s| s.path().is_dir())
            {
                sizes.insert(s.file_name().to_string_lossy().to_string());
            }
        }

        sizes
    }

    fn config(&self) -> Ini {
        let mut config_path: PathBuf = PathBuf::new();

        for p in &self.paths() {
            let theme_config_path = p.join("index.theme");
            if theme_config_path.exists() {
                config_path = theme_config_path;
                break;
            }
        }

        let Ok(config) = Ini::load_from_file(config_path) else {
            return Ini::new();
        };

        config
    }

    pub fn config_value<S: Into<String>, A: AsRef<str>>(
        &self,
        section_name: S,
        key: A,
    ) -> Option<String> {
        let cfg = &self.config();
        let Some(section) = cfg.section(Some(section_name)) else {
            return None;
        };

        let Some(value) = section.get(key) else {
            return None;
        };

        Some(value.to_string())
    }

    pub fn inherits(&self) -> Vec<String> {
        let Some(inherits) = &self.config_value("Icon Theme", "Inherits") else {
            return Vec::new();
        };

        inherits.split(",").map(|s| String::from(s)).collect()
    }

    pub fn default_size(&self) -> Option<u32> {
        let Some(size_str) = &self.config_value("Icon Theme", "DesktopDefault") else {
            return None;
        };

        match size_str.parse::<u32>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }

    /// Get the full inheritance stack for a theme
    /// by following its own Inherits= as well
    /// as the Inherits of the themes it
    /// inherits from.
    /// INCLUSIVE: The stack will include the current
    /// theme's name.
    /// This function is mostly used internally,
    /// but it is exposed in case you have a
    /// special use case.
    pub fn inheritance_stack(&self) -> HashSet<String> {
        get_inheritance_stack(&self, HashSet::new())
    }
}

impl IconTheme {
    pub fn from_name<S: Into<String>>(name: S) -> IconTheme {
        IconTheme { name: name.into() }
    }

    pub fn current() -> IconTheme {
        let home = std::env::var("HOME").unwrap_or("/home".into());
        let config_path = std::env::var("XDG_CONFIG_HOME").unwrap_or(home.clone());
        let settings_paths = [
            PathBuf::from(&config_path)
                .join("gtk-4.0")
                .join("settings.ini"),
            PathBuf::from(&config_path)
                .join("gtk-3.0")
                .join("settings.ini"),
            PathBuf::from(&home).join("gtk-4.0").join("settings.ini"),
            PathBuf::from(&home).join("gtk-3.0").join("settings.ini"),
        ];

        for p in &settings_paths {
            if !std::fs::exists(p).unwrap_or(false) {
                continue;
            }

            let Ok(conf) = Ini::load_from_file(p) else {
                continue;
            };

            if let Some(section) = conf.section(Some("Settings")) {
                if let Some(theme) = section.get("gtk-icon-theme-name") {
                    return IconTheme { name: theme.into() };
                } else {
                    continue;
                }
            }
        }

        IconTheme {
            name: "hicolor".into(),
        }
    }
}

fn get_inheritance_stack(theme: &IconTheme, mut set: HashSet<String>) -> HashSet<String> {
    set.insert(theme.name().into());

    for t in theme.inherits() {
        let it = IconTheme::from_name(t);
        set.extend(get_inheritance_stack(&it, set.clone()));
    }

    set
}
