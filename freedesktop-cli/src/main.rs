use freedesktop_apps::ApplicationEntry;
use std::time::Instant;

fn main() {
    let total_start = Instant::now();
    let mut apps_processed = 0;
    let mut icons_found = 0;
    let mut icons_not_found = 0;
    let mut total_icon_search_time = std::time::Duration::ZERO;
    
    println!("Starting icon lookup benchmark...\n");
    
    for app in ApplicationEntry::all() {
        if app.should_show() {
            apps_processed += 1;
            println!("App {}: {}", apps_processed, app.name().unwrap());
            
            if let Some(app_icon) = app.icon() {
                println!("  Icon name: {}", app_icon);
                
                let icon_start = Instant::now();
                let icon = freedesktop_icon::get_icon(&app_icon);
                let icon_time = icon_start.elapsed();
                total_icon_search_time += icon_time;
                
                match icon {
                    Some(path) => {
                        icons_found += 1;
                        println!("  Found: {} (took {:?})", path.to_string_lossy(), icon_time);
                    },
                    None => {
                        icons_not_found += 1;
                        println!("  Not found (took {:?})", icon_time);
                    }
                }
                
                if icon_time.as_millis() > 10 {
                    println!("  ⚠️  SLOW LOOKUP: {:?}", icon_time);
                }
            } else {
                println!("  No icon specified");
            }
            println!();
        }
    }
    
    let total_time = total_start.elapsed();
    
    println!("=== BENCHMARK RESULTS ===");
    println!("Total apps processed: {}", apps_processed);
    println!("Icons found: {}", icons_found);
    println!("Icons not found: {}", icons_not_found);
    println!("Total icon search time: {:?}", total_icon_search_time);
    println!("Total execution time: {:?}", total_time);
    println!("Average time per icon: {:?}", total_icon_search_time / (icons_found + icons_not_found).max(1) as u32);
    println!("Success rate: {:.1}%", (icons_found as f64 / (icons_found + icons_not_found).max(1) as f64) * 100.0);

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
