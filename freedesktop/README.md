# freedesktop

[![Crates.io](https://img.shields.io/crates/v/freedesktop.svg)](https://crates.io/crates/freedesktop)
[![Documentation](https://docs.rs/freedesktop/badge.svg)](https://docs.rs/freedesktop)

Rust implementations of the freedesktop.org specifications for Linux desktop integration.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
freedesktop = "0.0.3"
```

### XDG Base Directories

```rust
use freedesktop::base_directories;

for dir in base_directories() {
    println!("XDG data directory: {}", dir.display());
}
```

### Desktop Applications

```rust
use freedesktop::ApplicationEntry;

// List all applications
for app in ApplicationEntry::all() {
    if app.should_show() {
        println!("{}: {}", 
            app.id().unwrap_or_default(), 
            app.name().unwrap_or_default()
        );
    }
}

// Parse and execute a desktop file
let app = ApplicationEntry::try_from_path("/usr/share/applications/firefox.desktop")?;
app.execute()?;
```

### Icon Themes

```rust
use freedesktop::{IconTheme, get_icon};

// Get the current system icon theme
let theme = IconTheme::current();
println!("Current theme: {}", theme.name());

// Find an icon using the convenience function
if let Some(path) = get_icon("firefox") {
    println!("Firefox icon: {}", path.display());
}

// Search within a specific theme
if let Some(theme) = IconTheme::from_name("Adwaita") {
    if let Some(path) = theme.get("folder") {
        println!("Folder icon: {}", path.display());
    }
}
```

## Features

This crate provides optional features for different functionality:

- **`core`** (default) - XDG base directories and desktop environment detection
- **`apps`** (default) - Desktop Entry parsing and application execution  
- **`icon`** (default) - Icon theme support and icon lookup
- **`cli`** - Command-line utilities (enables `apps`)

### Feature Usage

```toml
# Default features (core + apps + icon)
freedesktop = "0.0.3"

# Only XDG base directories
freedesktop = { version = "0.0.3", default-features = false, features = ["core"] }

# Only desktop applications
freedesktop = { version = "0.0.3", default-features = false, features = ["apps"] }

# Icon theme support
freedesktop = { version = "0.0.3", default-features = false, features = ["icon"] }
```

## Standards Compliance

- **Spec-compliant** - Follows freedesktop.org specifications exactly
- **Safe execution** - Proper field code expansion and shell escaping
- **Comprehensive** - Supports localization, terminal apps, working directories
- **Well-tested** - Extensive test coverage including edge cases

## Platform Support

- Linux
- BSD variants  
- Other Unix-like systems with XDG support

## Development

This library is built as a workspace with the following crates:

- **[freedesktop-core](./freedesktop-core)** - XDG base directories and desktop environment detection
- **[freedesktop-apps](./freedesktop-apps)** - Desktop Entry parsing and application execution
- **[freedesktop-icon](./freedesktop-icon)** - Icon theme support and icon lookup

## License

MIT