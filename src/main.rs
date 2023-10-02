use gtk::prelude::*;
use gtk::{Button, Grid, Entry, Window, WindowType};
use std::process::Command;
use std::env;
use std::rc::Rc;
use std::cell::RefCell;

struct App {
    name: &'static str,
    path: &'static str,
    command: &'static str,
}

const APPS: [App; 15] = [
    App { name: "Alarm", path: "rustedalarm", command: "./" },
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

fn launch_app(app: &App, project_dir: &std::path::PathBuf) {
    let script_path = project_dir.join(app.path);

    let output = Command::new(app.command)
        .arg(&script_path)
        .current_dir(&project_dir)
        .output()
        .expect("Failed to launch app");

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Failed to launch app {}: {}", app.name, error_message);
    }
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let project_dir = Rc::new(RefCell::new(current_dir.join(".scripts")));

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
        let button = Button::with_label(app.name);

        let project_dir_clone = Rc::clone(&project_dir);
        let app_clone = app.clone();
        button.connect_clicked(move |_| {
            let project_dir = project_dir_clone.borrow();
            launch_app(&app_clone, &project_dir);
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
