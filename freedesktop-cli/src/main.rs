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
    for i in theme.inherits() {
        println!("INHERIT: {}", i);
    }

    println!("Default size: {}", theme.default_size().unwrap_or(1));

    for p in theme.paths() {
        println!("{}", p.display());
        println!("Sizes:");
        for s in theme.sizes() {
            println!("- {}", s);
        }
    }
}
