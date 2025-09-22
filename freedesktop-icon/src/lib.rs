use ini::Ini;
use std::{
    collections::HashSet,
    fmt::format,
    hash::Hash,
    path::{Path, PathBuf},
};

#[derive(Default, Debug, Clone)]
pub struct IconTheme {
    name: String,
    path: PathBuf,
}

impl IconTheme {
    pub fn name(&self) -> &str {
        &self.name
    }

    fn config(&self) -> Ini {
        let config_path = &self.path.join("index.theme");
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

    pub fn icon_dirs(&self, size: u32, scale: u8) -> Vec<PathBuf> {
        let Some(dir_str) = &self.config_value("Icon Theme", "Directories") else {
            return Vec::new();
        };

        let dirs: Vec<String> = dir_str.split(",").map(|s| String::from(s)).collect();

        let mut paths: Vec<PathBuf> = Vec::new();

        for d in &dirs {
            let dir_size = &self
                .config_value(d, "Size")
                .and_then(|s| Some(s.parse::<u32>().unwrap_or(0)))
                .or_else(|| Some(0))
                .unwrap();

            let dir_scale = match &self.config_value(d, "Scale") {
                Some(s) => s.parse::<u8>().unwrap_or(1),
                None => 1,
            };

            if (dir_size == &size) && (dir_scale == scale) {
                paths.push(self.path.join(d));
            }
        }

        paths
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
    pub fn inheritance_stack(&self) -> Vec<IconTheme> {
        // We will do a depth first search over
        // the inheritance chain
        let mut seen: HashSet<String> = HashSet::new();
        let mut search_stack: Vec<String> = Vec::new();
        let mut stack: Vec<IconTheme> = Vec::new();

        seen.insert(self.name.clone());
        search_stack.extend(self.inherits().into_iter().rev().collect::<Vec<String>>());
        stack.push(self.clone());

        while let Some(next) = search_stack.pop() {
            if seen.contains(&next) {
                continue;
            }

            seen.insert(next.clone());
            let Some(theme) = IconTheme::from_name(next) else {
                continue;
            };

            search_stack.extend(theme.inherits().into_iter().rev().collect::<Vec<String>>());
            stack.push(theme);
        }

        stack
    }

    /// Get an icon by name following the freedesktop icon theme specification
    /// Searches through the current theme and inherited themes for the icon
    pub fn get(&self, icon_name: &str) -> Option<PathBuf> {
        let size = self.default_size().unwrap_or(48);
        let stack = self.inheritance_stack();
        let filenames = [
            format!("{}.{}", icon_name, "svg"),
            format!("{}.{}", icon_name, "png"),
            format!("{}.{}", icon_name, "xpm"),
        ];

        for theme in stack {
            for d in theme.icon_dirs(size, 1) {
                for f in &filenames {
                    let icon_path = d.join(f);

                    if icon_path.exists() {
                        return Some(icon_path);
                    }
                }
            }
        }

        None
    }
}

impl IconTheme {
    /// According to the spec:
    /// First search $XDG_DATA_HOME/icons/[theme name]
    /// If not found, search $XDG_DATA_DIRS in order for
    /// [dir name]/icons/[theme name]
    /// The order of $XDG_DATA_DIRS needs to be respected, as the
    /// first hit counts as the "canonical path" of the theme.
    /// We will also check that an index.theme exists at the path
    /// since any valid theme must have this file.
    pub fn from_name<S: Into<String>>(name: S) -> Option<IconTheme> {
        let name: String = name.into();
        let xdg_home_path = freedesktop_core::xdg_data_home().join("icons").join(&name);

        if xdg_home_path.exists() && xdg_home_path.join("index.theme").exists() {
            return Some(IconTheme {
                name: name.into(),
                path: xdg_home_path,
            });
        }

        for data_dir in freedesktop_core::xdg_data_dirs() {
            let theme_path = data_dir.join("icons").join(&name);

            if theme_path.exists() && theme_path.join("index.theme").exists() {
                return Some(IconTheme {
                    name: name.into(),
                    path: theme_path,
                });
            }
        }

        None
    }

    pub fn current() -> IconTheme {
        let home = std::env::var("HOME").expect("$HOME variable not set.");
        let config_path = freedesktop_core::xdg_config_home();
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
        let fallback_theme = IconTheme::from_name("hicolor").expect("The hicolor theme is not present. This is a required fallback theme and must be installed");

        for p in &settings_paths {
            if !p.exists() {
                continue;
            }

            let Ok(conf) = Ini::load_from_file(p) else {
                continue;
            };

            if let Some(section) = conf.section(Some("Settings")) {
                if let Some(theme) = section.get("gtk-icon-theme-name") {
                    return IconTheme::from_name(theme).unwrap_or(fallback_theme);
                } else {
                    continue;
                }
            }
        }

        fallback_theme
    }
}
