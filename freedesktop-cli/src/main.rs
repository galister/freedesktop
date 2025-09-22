use std::{collections::HashSet, rc::Rc};

use freedesktop_apps::ApplicationEntry;
use freedesktop_icon::IconTheme;

fn main() {
    // for app in ApplicationEntry::all() {
    //     if app.should_show() {
    //         println!("{}", app.path().display());
    //     }
    // }
    // let app = ApplicationEntry::from_id("steam").expect("Could not find app");
    // app.execute();
    // let app =
    //     ApplicationEntry::from_path("/home/javi/.nix-profile/share/applications/obsidian.desktop");
    // app.execute();
    // let theme = IconTheme::current();

    // println!("Current icon theme: {}", theme);

    let theme = IconTheme::current();
    let icon = theme.get("com.discordapp.Discord");
    println!("{}", icon.unwrap().display());
}

fn print_theme_info(theme: IconTheme) {
    println!("--- {} ---", theme.name());
    println!("Default size: {}", theme.default_size().unwrap_or(1));
    for i in theme.inherits() {
        println!("INHERIT: {}", i);
    }
}
