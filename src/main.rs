use gtk::prelude::*;
use gtk::{Button, Grid, Entry, Window, WindowType};
use std::process::Command;
use std::env;

struct App {
    name: &'static str,
    path: &'static str,
    command: &'static str,
}

const APPS: [App; 15] = [
    App { name: "Alarm", path: "rusted_alarm", command: "./" },
    App { name: "Whisper", path: "whisper.sh", command: "sh" },
    App { name: "Realtime Transcription", path: "realtimeWhisper.sh", command: "sh" },
    App { name: "Tesseract OCR", path: "tesseract.sh", command: "sh" },
    App { name: "Translator", path: "argos.sh", command: "sh" },
    App { name: "Converter", path: "converter.sh", command: "sh" },
    App { name: "Polymath", path: "polymath.sh", command: "sh" },
    App { name: "Downloader", path: "downloader.sh", command: "sh" },
    App { name: "TTS", path: "tts.sh", command: "sh" },
    App { name: "ComfyUI", path: "comfyUI.sh", command: "sh" },
    App { name: "Riffusion", path: "riffusion.sh", command: "sh" },
    App { name: "Background remover", path: "bgRemover.sh", command: "sh" },
    App { name: "Images to video", path: "img2vid.sh", command: "sh" },
    App { name: "Remove Duplicates", path: "rmDuplicate.sh", command: "sh" },
    App { name: "Virtualenv activate", path: "vactivate.sh", command: "sh" },
];

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let project_dir = current_dir.join(".scripts");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("App Drawer");
    window.set_default_size(400, 300);

    let search_entry = Entry::new();
    let search_button = Button::with_label("Search");

    let grid = Grid::new();
    grid.set_column_homogeneous(true);
    grid.set_row_spacing(10);
    grid.set_column_spacing(10);

    for (i, app) in APPS.iter().enumerate() {
        let script_path = project_dir.join(app.path);
        let button = Button::with_label(app.name);

        button.connect_clicked(move |_| {
            let _ = Command::new(app.command)
                .arg(&script_path)
                .spawn()
                .expect("Failed to launch app");
        });

        grid.attach(&button, (i % 5) as i32, (i / 5) as i32, 1, 1);
    }

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    main_box.pack_start(&search_entry, false, false, 0);
    main_box.pack_start(&search_button, false, false, 0);
    main_box.pack_start(&grid, true, true, 0);

    window.add(&main_box);
    window.show_all();

    gtk::main();
}
