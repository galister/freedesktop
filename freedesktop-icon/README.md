# freedesktop-icon

A Rust implementation of the [freedesktop Icon Theme Specification](https://specifications.freedesktop.org/icon-theme-spec/latest/) for finding and loading desktop icons on Linux systems.

## Features

- **Icon theme support** - Load and parse icon themes following the freedesktop specification
- **Icon lookup** - Find icons by name with proper inheritance chain traversal
- **Pixmap fallback** - Automatic fallback to `/usr/share/pixmaps` when icons aren't found in themes
- **Performance optimized** - Cached theme loading and lazy inheritance evaluation
- **XDG compliance** - Respects XDG base directories and user overrides

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
freedesktop-icon = "0.0.3"
```

### Basic Usage

```rust
use freedesktop_icon::{IconTheme, get_icon};

// Get the current system icon theme
let theme = IconTheme::current();
println!("Current theme: {}", theme.name());

// Find an icon using the convenience function
if let Some(path) = get_icon("firefox") {
    println!("Firefox icon: {}", path.display());
}

// Or search within a specific theme
let theme = IconTheme::from_name("Adwaita").unwrap();
if let Some(path) = theme.get("folder") {
    println!("Folder icon: {}", path.display());
}
```

### Icon Lookup Process

The library follows the freedesktop specification for icon lookup:

1. **Current theme** - Search in the active icon theme
2. **Inherited themes** - Recursively search parent themes
3. **Hicolor fallback** - Search in the default hicolor theme
4. **Pixmap directories** - Finally check `/usr/share/pixmaps` and other pixmap locations

### Supported Formats

- **SVG** (`.svg`) - Vector graphics (preferred)
- **PNG** (`.png`) - Raster graphics
- **XPM** (`.xpm`) - Legacy X11 pixmaps

## Advanced Usage

### Working with Specific Themes

```rust
use freedesktop_icon::IconTheme;

// Load a specific theme
if let Some(theme) = IconTheme::from_name("Papirus") {
    println!("Theme path: {}", theme.path().display());
    println!("Default size: {:?}", theme.default_size());
    
    // Get inheritance chain
    for parent in theme.inherits() {
        println!("Inherits from: {}", parent);
    }
    
    // Find icon directories for a specific size
    for dir in theme.icon_dirs(48, 1) {
        println!("48x48 icon directory: {}", dir.display());
    }
}
```

### Direct Pixmap Search

```rust
use freedesktop_icon::Pixmap;

// Search for icons in pixmap directories only
if let Some(path) = Pixmap::get("application-icon") {
    println!("Found pixmap: {}", path.display());
}
```

## XDG Integration

The library respects XDG Base Directory specifications:

- **Icon themes**: `$XDG_DATA_DIRS/icons` (typically `/usr/share/icons`)
- **User themes**: `$XDG_DATA_HOME/icons` (typically `~/.local/share/icons`)
- **Theme config**: Reads from GTK settings in `$XDG_CONFIG_HOME`
- **Pixmap fallback**: `$XDG_DATA_DIRS/pixmaps`

### Theme Detection

The current theme is detected from GTK configuration files in this order:

1. `$XDG_CONFIG_HOME/gtk-4.0/settings.ini`
2. `$XDG_CONFIG_HOME/gtk-3.0/settings.ini`
3. `$HOME/.config/gtk-4.0/settings.ini`
4. `$HOME/.config/gtk-3.0/settings.ini`

Falls back to `hicolor` if no theme is configured.

## Performance Notes

- **Theme caching**: Parsed theme configurations are cached for performance
- **Lazy loading**: Inherited themes are only loaded when needed during icon search
- **Early exit**: Search stops as soon as an icon is found in any theme
- **Static optimization**: The current theme is cached globally using `LazyLock`

## Integration

This crate is part of the [freedesktop](https://crates.io/crates/freedesktop) workspace. You can use it standalone or as part of the main crate:

```toml
# Standalone
freedesktop-icon = "0.0.3"

# Or as part of the main freedesktop crate
freedesktop = { version = "0.0.3", features = ["icon"] }
```

## Related Crates

- [`freedesktop-core`](../freedesktop-core) - XDG base directories and desktop environment detection
- [`freedesktop-apps`](../freedesktop-apps) - Desktop entry parsing and application discovery

## License

MIT - see [LICENSE](../LICENSE) for details.