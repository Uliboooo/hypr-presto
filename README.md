# HyprPresto

A minimal, keyboard-centric application launcher for Linux, designed for speed and efficiency. HyprPresto allows you to launch your favorite applications with a single keystroke.

## Features

- **Instant Launch:** Trigger applications with a single key press.
- **Modern UI:** Clean, dark-themed interface built with GTK4.
- **Wayland Native:** Built with `gtk4-layer-shell`, perfect for compositors like Hyprland and Sway.
- **Lightweight:** Fast startup and minimal resource usage.
- **Simple Configuration:** Easy-to-read TOML configuration.

## Dependencies

To run HyprPresto, you need the following installed on your system:

- **GTK4**
- **gtk4-layer-shell**

To build from source, you will also need:
- Rust (cargo)
- `blueprint-compiler`

## How to Install

Currently, HyprPresto is available as a binary release on GitHub.

1.  Go to the [Releases](https://github.com/uliboooo/hypr-presto/releases) page.
2.  Download the latest `hypr-presto` binary.
3.  Make it executable:
    ```bash
    chmod +x hypr-presto
    ```
4.  (Optional) Move it to a directory in your `$PATH`, e.g., `/usr/local/bin`:
    ```bash
    sudo mv hypr-presto /usr/local/bin/
    ```

## Configuration

HyprPresto follows the XDG Base Directory Specification for its configuration.

**Path:** `~/.config/hypr-presto/config.toml`

### Structure

The configuration file uses the TOML format. Define your shortcuts under the `[apps]` section. The key is the single character you want to press, and the value is the **Desktop Entry ID** of the application (usually the filename of the `.desktop` file without the extension).

### Example `config.toml`

```toml
[apps]
f = "firefox"                 # Press 'f' to launch Firefox
g = "com.mitchellh.ghostty"   # Press 'g' to launch Ghostty
c = "code"                    # Press 'c' to launch VS Code
```

> **Note:** To find the correct App ID, look at the filenames in `/usr/share/applications/` (e.g., for `spotify.desktop`, use `spotify`).

## Usage

![Screenshot placeholder](path/to/screenshot.png)

1.  Ensure you have your `config.toml` ready.
2.  Run the application:
    ```bash
    hypr-presto
    ```
3.  The launcher window will appear.
4.  **Press the key** corresponding to the app you want to launch (e.g., press `f`).
5.  The app will launch, and HyprPresto will close automatically.
6.  To close without launching, press `Esc`.

### Integration with Hyprland

You can bind HyprPresto to a key in your `hyprland.conf`:

```conf
bind = $mainMod, P, exec, hypr-presto(bin path)
```
