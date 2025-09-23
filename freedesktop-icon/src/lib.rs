use ini::Ini;
use std::{collections::HashSet, io::BufRead, path::PathBuf, time::Instant};

#[derive(Debug, Clone)]
pub struct IconTheme {
    name: String,
    path: PathBuf,
    config: Ini,
}

impl IconTheme {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn config(&self) -> &Ini {
        &self.config
    }

    pub fn config_value<S: Into<String>, A: AsRef<str>>(
        &self,
        section_name: S,
        key: A,
    ) -> Option<String> {
        let cfg = self.config();
        let section = cfg.section(Some(section_name))?;
        let value = section.get(key)?;

        Some(value.to_string())
    }

    pub fn inherits(&self) -> Vec<String> {
        let Some(inherits) = &self.config_value("Icon Theme", "Inherits") else {
            return Vec::new();
        };

        inherits.split(",").map(String::from).collect()
    }

    pub fn icon_dirs(&self, size: u32, scale: u8) -> Vec<PathBuf> {
        let Some(dir_str) = &self.config_value("Icon Theme", "Directories") else {
            return Vec::new();
        };

        let dirs: Vec<String> = dir_str.split(",").map(String::from).collect();

        let mut paths: Vec<PathBuf> = Vec::new();

        for d in &dirs {
            let dir_size = &self
                .config_value(d, "Size")
                .map(|s| s.parse::<u32>().unwrap_or(0))
                .unwrap_or(0);

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

        size_str.parse::<u32>().ok()
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
    // pub fn inheritance_stack(&self) -> Vec<String> {
    //     // We will do a depth first search over
    //     // the inheritance chain
    //     let mut seen: HashSet<String> = HashSet::new();
    //     let mut search_stack: Vec<String> = Vec::new();
    //     let mut stack: Vec<String> = Vec::new();

    //     seen.insert(self.name.clone());

    //     search_stack.extend(self.inherits().into_iter().rev().collect::<Vec<String>>());

    //     while let Some(next) = search_stack.pop() {
    //         if seen.contains(&next) {
    //             println!("      Skipping already seen theme: {}", next);
    //             continue;
    //         }

    //         seen.insert(next.clone());

    //         let from_name_start = Instant::now();
    //         let Some(theme) = IconTheme::from_name(next.clone()) else {
    //             let from_name_time = from_name_start.elapsed();
    //             println!(
    //                 "      Failed to load theme '{}' in {:?}",
    //                 next, from_name_time
    //             );
    //             continue;
    //         };
    //         let from_name_time = from_name_start.elapsed();
    //         from_name_total += from_name_time;
    //         themes_loaded += 1;
    //         println!("      Loaded theme '{}' in {:?}", next, from_name_time);

    //         let inherits_start = Instant::now();
    //         let theme_inherits = theme.inherits();
    //         let inherits_time = inherits_start.elapsed();
    //         inherits_total += inherits_time;
    //         println!(
    //             "        Theme '{}' inherits {} themes, took {:?}",
    //             next,
    //             theme_inherits.len(),
    //             inherits_time
    //         );

    //         search_stack.extend(theme_inherits.into_iter().rev().collect::<Vec<String>>());
    //         stack.push(theme);
    //     }

    //     println!("      Loop processing took: {:?}", loop_start.elapsed());
    //     println!("      Stats: {} themes loaded", themes_loaded);
    //     println!("        Total from_name() time: {:?}", from_name_total);
    //     println!("        Total inherits() time: {:?}", inherits_total);
    //     println!(
    //         "    Total inheritance_stack took: {:?}, returning {} themes",
    //         total_start.elapsed(),
    //         stack.len()
    //     );

    //     stack
    // }

    // Internal function for getting an icon from the
    // theme. The public get() function actually
    // traverses the inheritance chain.
    fn get_icon(&self, icon_name: &str, size: u32, scale: u8) -> Option<PathBuf> {
        let filenames = [
            format!("{}.{}", icon_name, "svg"),
            format!("{}.{}", icon_name, "png"),
            format!("{}.{}", icon_name, "xpm"),
        ];

        for d in &self.icon_dirs(size, scale) {
            for f in &filenames {
                let icon_path = d.join(f);
                if icon_path.exists() {
                    return Some(icon_path);
                }
            }
        }

        None
    }

    /// Get an icon by name following the freedesktop icon theme specification
    /// Searches through the current theme and inherited themes for the icon
    pub fn get(&self, icon_name: &str) -> Option<PathBuf> {
        let size_start = Instant::now();
        let size = self.default_size().unwrap_or(48);
        println!("  Getting default size took: {:?}", size_start.elapsed());
        let scale: u8 = 1;

        // let stack_start = Instant::now();
        // let stack = self.inheritance_stack();
        // println!(
        //     "  Building inheritance stack took: {:?}",
        //     stack_start.elapsed()
        // );
        // println!("  Stack has {} themes", stack.len());

        if let Some(icon_path) = &self.get_icon(icon_name, size, scale) {
            return Some(icon_path.to_owned());
        }

        // If we don't find it in the current theme, start recursing
        // into the inheritance chain loading the themes lazily
        for theme_name in &self.inherits() {
            let Some(theme) = IconTheme::from_name(theme_name) else {
                continue;
            };

            match theme.get_icon(icon_name, size, scale) {
                Some(icon_path) => return Some(icon_path),
                None => continue,
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

        if xdg_home_path.exists() {
            let config_path = xdg_home_path.join("index.theme");
            if config_path.exists() {
                let parse_start = Instant::now();
                let config = Ini::load_from_file(&config_path).unwrap_or_else(|_| Ini::new());
                println!(
                    "        Parsing {} index.theme took: {:?}",
                    name,
                    parse_start.elapsed()
                );
                return Some(IconTheme {
                    name,
                    path: xdg_home_path,
                    config,
                });
            }
        }

        for data_dir in freedesktop_core::xdg_data_dirs() {
            let theme_path = data_dir.join("icons").join(&name);

            if theme_path.exists() {
                let config_path = theme_path.join("index.theme");
                if config_path.exists() {
                    let parse_start = Instant::now();
                    let config = Ini::load_from_file(&config_path).unwrap_or_else(|_| Ini::new());
                    println!(
                        "        Parsing {} index.theme took: {:?}",
                        name,
                        parse_start.elapsed()
                    );
                    return Some(IconTheme {
                        name,
                        path: theme_path,
                        config,
                    });
                }
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
        let fallback_theme = || {
            println!("Ran this bitch");
            IconTheme::from_name("hicolor").expect("The hicolor theme is not present. This is a required fallback theme and must be installed")
        };

        for p in &settings_paths {
            println!("path: {}", p.display());
            if !p.exists() {
                println!("No exists");
                continue;
            }

            let Ok(conf) = Ini::load_from_file(p) else {
                println!("Could not load ini");
                continue;
            };

            if let Some(section) = conf.section(Some("Settings")) {
                if let Some(theme) = section.get("gtk-icon-theme-name") {
                    return IconTheme::from_name(theme).unwrap_or_else(fallback_theme);
                } else {
                    println!("No section");
                    continue;
                }
            }
        }

        fallback_theme()
    }
}

// We'll use this function when we just need a line from
// an index.theme without having to parse the whole file
fn find_line(path: &str, prefix: &str) -> Option<String> {
    let file = std::fs::File::open(path).ok()?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines().filter_map(|l| l.ok()) {
        if line.starts_with(prefix) {
            return Some(line);
        }
    }

    None
}
