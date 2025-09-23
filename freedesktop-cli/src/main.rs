use freedesktop_apps::ApplicationEntry;

fn main() {
    for app in ApplicationEntry::all() {
        if app.should_show() {
            println!("{}", app.name().unwrap());
            if let Some(app_icon) = app.icon() {
                // println!("-- ICON --");
                println!("{}", app_icon);
                let icon = freedesktop_icon::get_icon(&app_icon);
                println!(
                    "{}",
                    icon.map_or_else(
                        || "Not found".to_string(),
                        |p| p.to_string_lossy().into_owned()
                    )
                );
            }
            println!();
        }
    }

    // let icon = theme.get("brave-fcmiknabmjbdhccoehdfdbplhgbeccdl-Default");
    // println!("{}", icon.unwrap_or("Not found".into()).display());
    // println!(
    //     "{}",
    //     Pixmap::get("brave-fcmiknabmjbdhccoehdfdbplhgbeccdl-Default")
    //         .unwrap_or("Not found".into())
    //         .display()
    // )
}

// fn print_theme_info(theme: IconTheme) {
//     println!("--- {} ---", theme.name());
//     println!("Default size: {}", theme.default_size().unwrap_or(1));
//     for i in theme.inherits() {
//         println!("INHERIT: {}", i);
//     }
//     println!("Path: {}", theme.path().display());
// }
