use gtk4::{gdk, gio, prelude::*, FlowBox};
use gtk4::{glib, Application, ApplicationWindow, Builder, EventControllerKey};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use xdg::BaseDirectories;

type LaunchData = HashMap<String, String>;

#[derive(Deserialize, Debug, Default)]
struct Config {
    apps: LaunchData,
}

// fn load_all_desktop_apps() {}

fn load_config() -> LaunchData {
    let conf_path = get_config_path();
    match conf_path {
        Some(v) => {
            let content = match fs::read_to_string(v) {
                Ok(c) => c,
                Err(_) => {
                    eprintln!("Could not read config.toml");
                    return HashMap::new();
                }
            };

            let config: Config = match toml::from_str(&content) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to parse config: {}", e);
                    return HashMap::new();
                }
            };

            config.apps
        }
        None => HashMap::new(),
    }
}

fn get_config_path() -> Option<PathBuf> {
    let xdg_dirs = BaseDirectories::with_prefix("hypr-presto");
    xdg_dirs.find_config_file("config.toml")
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("dev.uliboooo.hypr-presto")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn runnable(id: &str) -> bool {
    if let Some(app_info) = gio::DesktopAppInfo::new(&format!("{id}.desktop")) {
        app_info.should_show()
    } else {
        false
    }
}

fn build_ui(app: &Application) {
    // Load CSS
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("style.css"));
    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let launch_data = load_config();

    let flow_box = FlowBox::builder()
        .valign(gtk4::Align::Start)
        .halign(gtk4::Align::Start)
        .max_children_per_line(5)
        .min_children_per_line(1)
        .selection_mode(gtk4::SelectionMode::None)
        .column_spacing(20)
        .row_spacing(20)
        .build();

    launch_data.iter().filter(|d| runnable(d.1)).for_each(|f| {
        let app_info = gio::DesktopAppInfo::new(&format!("{}.desktop", f.1)).unwrap();
        let icon = app_info.icon();
        let name = app_info.name();

        let img = gtk4::Image::new();
        if let Some(icon_data) = icon {
            img.set_from_gicon(&icon_data);
        }
        img.set_pixel_size(64);
        img.add_css_class("app-icon");

        let key_label = gtk4::Label::new(Some(&f.0.to_uppercase().to_string()));
        key_label.add_css_class("app-key");

        let name_label = gtk4::Label::new(Some(&name));
        name_label.add_css_class("app-name");

        let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        vbox.append(&img);
        vbox.append(&key_label);
        vbox.append(&name_label);
        vbox.set_margin_top(15);
        vbox.set_margin_bottom(15);
        vbox.set_margin_start(15);
        vbox.set_margin_end(15);

        // FlowBoxChild is created automatically, but we just insert the box content
        // Note: The CSS style `flowboxchild` will target the container created by insert
        flow_box.insert(&vbox, -1);
    });

    let ui_src = include_str!(concat!(env!("OUT_DIR"), "/window.ui"));
    let builder = Builder::from_string(ui_src);

    let window: ApplicationWindow = builder
        .object("prefix_launcher")
        .expect("Could not find window 'prefix_launcher'");

    let main_box: gtk4::Box = builder
        .object("main_box")
        .expect("Could not find box 'main_box'");

    main_box.append(&flow_box);

    window.set_application(Some(app));

    // Layer Shell setup
    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_keyboard_mode(KeyboardMode::Exclusive);

    let key_controller: EventControllerKey = builder
        .object("key_controller")
        .expect("Could not find key controller 'key_controller'");
    let window_weak = window.downgrade();

    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        if keyval == gdk::Key::Escape {
            if let Some(window) = window_weak.upgrade() {
                window.close();
            }
        }
        let input_key = keyval
            .name()
            .map(|f| f.to_string())
            .unwrap()
            .trim()
            .to_string();

        let res = launch_data
            .get(&input_key)
            .and_then(|app_id| gio::DesktopAppInfo::new(&format!("{app_id}.desktop")))
            .map(|f| f.launch(&[], Some(&gio::AppLaunchContext::new())));

        match res {
            Some(v) => match v {
                Ok(_) => {
                    if let Some(window) = window_weak.upgrade() {
                        window.close()
                    }
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            },
            None => {
                eprintln!("error. not found a app");
            }
        }

        glib::Propagation::Proceed
    });

    // Load config just to show we did it (logic wasn't used in C app for UI yet)
    let _apps = load_config();
    // In the future, _apps could be used to populate buttons dynamically.

    window.present();
}
