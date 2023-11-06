use gtk::prelude::*;
use gtk::{Button, Box, Entry, EntryExt, Window, WindowType, ScrolledWindow};
use serde::Deserialize;
use std::sync::Arc;
use gtk::Adjustment;

#[derive(Deserialize, Clone)]
struct App {
    name: String,
    path: String,
    command: String,
}

const SCRIPTS_DIR: &str = "";

use std::fs::File;
use std::io::Read;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("App Drawer");
    window.set_default_size(200, 300); // Reduced the default size for vertical layout

    let search_entry = Entry::new();
    let search_button = Button::with_label("Search");

    // Create a ScrolledWindow to contain the vertical box
    let scrolled_window = ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);

    // Create a vertical box to hold the buttons
    let vertical_box = Box::new(gtk::Orientation::Vertical, 0);

    // Load the apps from the JSON file
    let mut apps_json = String::new();
    File::open(".scripts/apps.json")
        .and_then(|mut file| file.read_to_string(&mut apps_json))
        .expect("Failed to read apps.json");

    // Parse the JSON into a vector of App
    let apps: Arc<Vec<App>> = Arc::new(serde_json::from_str(&apps_json).expect("Failed to parse apps.json"));

    for app in apps.iter().cloned() {
        let button = Button::with_label(&app.name);
        button.connect_clicked(move |_| {
            let path = format!("{}{}", SCRIPTS_DIR, app.path);
            let _ = std::process::Command::new(&app.command)
                .arg(path)
                .spawn()
                .expect("Failed to launch app");
        });

        // Pack the button into the vertical box
        vertical_box.pack_start(&button, false, false, 0);
    }

    // Add the vertical box to the scrolled window
    scrolled_window.add(&vertical_box);

    let main_box = Box::new(gtk::Orientation::Vertical, 0);
    main_box.pack_start(&search_entry, false, false, 0);
    main_box.pack_start(&search_button, false, false, 0);
    main_box.pack_start(&scrolled_window, true, true, 0);

    window.add(&main_box);

    // Add search functionality
    search_entry.connect_changed(move |entry| {
        let search_text = entry.get_text().to_string();
        search_apps(&search_text, &vertical_box, &apps); // Pass the vertical_box for search
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit::default()
    });

    window.show_all();

    gtk::main();
}

fn search_apps(search_text: &str, vertical_box: &Box, apps: &Arc<Vec<App>>) {
    // Remove all existing buttons from the vertical box
    vertical_box.foreach(|child| {
       vertical_box.remove(child); 
    });

    for app in apps.iter().cloned() {
        if app.name.to_lowercase().contains(&search_text.to_lowercase()) {
            let button = Button::with_label(&app.name);
            let _apps_clone = Arc::clone(apps);
            button.connect_clicked(move |_| {
                let path = format!("{}{}", SCRIPTS_DIR, app.path);
                let _ = std::process::Command::new(&app.command)
                    .arg(path)
                    .spawn()
                    .expect("Failed to launch app");
            });

            // Pack the button into the vertical box
            vertical_box.pack_start(&button, false, false, 0);
        }
    }

    vertical_box.show_all();
}
