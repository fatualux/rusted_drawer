use gtk::prelude::*;
use gtk::{Button, Grid, Entry, EntryExt, Window, WindowType};

struct App {
    name: &'static str,
    path: &'static str,
}

const SCRIPTS_DIR: &str = "apps/";


const APPS: [App; 15] = [
    App { name: "Alarm", path: "alarm.sh" },
    App { name: "Whisper", path: "whisper.sh" },
    App { name: "Realtime Transcription", path: "realtimeWhisper.sh" },
    App { name: "Tesseract OCR", path: "tesseract.sh" },
    App { name: "Translator", path: "argos.sh" },
    App { name: "Converter", path: "converter.sh" },
    App { name: "Polymath", path: "polymath.sh" },
    App { name: "Downloader", path: "downloader.sh" },
    App { name: "TTS", path: "tts.sh" },
    App { name: "ComfyUI", path: "comfyui.sh" },
    App { name: "Riffusion", path: "riffusion.sh" },
    App { name: "Background remover", path: "bgRemover.sh" },
    App { name: "Images to video", path: "img2vid.sh" },
    App { name: "Remove Duplicates", path: "rmDuplicate.sh" },
    App { name: "Virtualenv activate", path: "vactivate.sh" },
];


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

    for app in APPS.iter() {
        let button = Button::with_label(app.name);
        button.connect_clicked(move |_| {
            let path = format!("{}{}", SCRIPTS_DIR, app.path);
            let _ = std::process::Command::new("sh")
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
        search_apps(&search_text, &grid);
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit::default()
    });

    window.show_all();

    gtk::main();
}

fn search_apps(search_text: &str, grid: &Grid) {
    // Remove all existing buttons from the grid
    grid.foreach(|child| {
        grid.remove(child);
    });

    let mut row = 0;
    let mut column = 0;

    for app in APPS.iter() {
        if app.name.to_lowercase().contains(&search_text.to_lowercase()) {
            let button = Button::with_label(app.name);
            button.connect_clicked(move |_| {
                let path = format!("{}{}", SCRIPTS_DIR, app.path);
                let _ = std::process::Command::new("sh")
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
