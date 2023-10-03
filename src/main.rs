use gtk::prelude::*;
use gtk::{Button, Grid, Entry, EntryExt, Window, WindowType};
use serde::Deserialize;
use std::sync::Arc;

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
    window.set_default_size(400, 300);

    let search_entry = Entry::new();
    let search_button = Button::with_label("Search");

    let grid = Grid::new();
    grid.set_column_homogeneous(true);
    grid.set_row_spacing(10);
    grid.set_column_spacing(10);

    let mut row = 0;
    let mut column = 0;

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

        grid.attach(&button, column, row, 1, 1);

        column += 1;
        if column == 5 {
            column = 0;
            row += 1;
        }
    }

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    main_box.pack_start(&search_entry, false, false, 0);
    main_box.pack_start(&search_button, false, false, 0);
    main_box.pack_start(&grid, true, true, 0);

    window.add(&main_box);

    // Add search functionality
    search_entry.connect_changed(move |entry| {
        let search_text = entry.get_text().to_string();
        search_apps(&search_text, &grid, &apps);
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit::default()
    });

    window.show_all();

    gtk::main();
}

fn search_apps(search_text: &str, grid: &Grid, apps: &Arc<Vec<App>>) {
    // Remove all existing buttons from the grid
    grid.foreach(|child| {
        grid.remove(child);
    });

    let mut row = 0;
    let mut column = 0;

    for app in apps.iter().cloned() {
        if app.name.to_lowercase().contains(&search_text.to_lowercase()) {
            let button = Button::with_label(&app.name);
            button.connect_clicked(move |_| {
                let path = format!("{}{}", SCRIPTS_DIR, app.path);
                let _ = std::process::Command::new(&app.command)
                    .arg(path)
                    .spawn()
                    .expect("Failed to launch app");
            });

            grid.attach(&button, column, row, 1, 1);

            column += 1;
            if column == 5 {
                column = 0;
                row += 1;
            }
        }
    }

    grid.show_all();
}
