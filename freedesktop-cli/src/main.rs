use freedesktop_apps::ApplicationEntry;
use freedesktop_icon::IconTheme;

fn main() {
    // Icons not found (but should be I think)
    // - yazi
    // - Mattermost
    let theme = IconTheme::current();
    println!("Current icon theme: {}", theme.name());
    // for app in ApplicationEntry::all() {
    //     if app.should_show() {
    //         println!("{}", app.name().unwrap());
    //         if let Some(app_icon) = app.icon() {
    //             println!("-- ICON --");
    //             println!("{}", app_icon);
    //             let icon = theme.get(&app_icon);
    //             println!(
    //                 "{}",
    //                 icon.map_or_else(
    //                     || "Not found".to_string(),
    //                     |p| p.to_string_lossy().into_owned()
    //                 )
    //             );
    //         }
    //         println!();
    //     }
    // }

    let icon = theme.get("yazi");
    println!("{}", icon.unwrap().display());
}

fn print_theme_info(theme: IconTheme) {
    println!("--- {} ---", theme.name());
    println!("Default size: {}", theme.default_size().unwrap_or(1));
    for i in theme.inherits() {
        println!("INHERIT: {}", i);
    }
}
