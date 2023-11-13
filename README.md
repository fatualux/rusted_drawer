[![License](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

**RUSTED DRAWER**

I DON'T KNOW RUST, I have just started learning.

I needied a GUI to launch the applications I keep in my Python virtual environments, my scripts, and so on; till now, I have been using a launcher I wrote, based on bash and zenity, but I wanted to try a different approach.

[![Watch the video](demo/untitled.mp4)

If you have suggestions or ideas, please let me know, by opening an issue.

**INSTALL**

Simply clone this repository, then move into the directory and run:

'cargo build'
'cargo run'

You can change the file *apps.json*, according to your needs.

The apps/scripts started by the launcher must be placed in a directory named **Apps**.

***Old version:***

![image info](demo/old_version.jpg)

***New version:***

![image info](demo/new_1.jpg)
![image info](demo/new_2.jpg)
![image info](demo/new_3.jpg)


## Example Code

Below is an example of the main.rs logic from the codebase:

```rust
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

// Rest of the code...

fn main() {
    // Initialization and window setup code...

    // Load the apps from the JSON file
    let mut apps_json = String::new();
    File::open("Apps/apps.json")
        .and_then(|mut file| file.read_to_string(&mut apps_json))
        .expect("Failed to read apps.json");

    // Parse the JSON into a vector of App
    let apps: Arc<Vec<App>> = Arc::new(serde_json::from_str(&apps_json).expect("Failed to parse apps.json"));

    // Rest of the code...
}

fn search_apps(search_text: &str, vertical_box: &Box, apps: &Arc<Vec<App>>) {
    // Search functionality code...
}
```
**Logic:**

1. The app initializes the GTK library using `gtk::init()`.
2. It creates a new GTK window with the title "App Drawer" and a default size of 200x300 pixels.
3. The app creates an entry field and a search button for searching applications.
4. It creates a scrolled window to contain a vertical box that will hold the application buttons.
5. The app loads the applications from a JSON file named "apps.json" located in the "Apps" directory.
6. It parses the JSON file into a vector of `App` structs using serde.
7. The app iterates over each application in the vector and creates a button for each application.
8. For each button, a click event handler is attached that launches the associated application when clicked.
9. The buttons are packed into the vertical box.
10. The vertical box is added to the scrolled window.
11. The search functionality is implemented by connecting the `changed` signal of the search entry field.
12. When the search text changes, the app filters the applications based on the search text and updates the buttons in the vertical box accordingly.
13. The window is shown and the GTK main event loop is started using `gtk::main()`.
14. When the window is closed, the app quits the GTK main event loop.

***Acknowledgements***

Special thanks to the Codeium engineering team for their assistance and support in developing this project. Their AI-powered assistant has been instrumental in providing quick and accurate coding help.

- [@codeium-assistant](https://github.com/codeium-assistant) - The AI assistant from Codeium that helped answer code-related questions.
- [Codeium](https://codeium.com/) - The AI company that developed the assistant, providing code autocomplete, search, and chat-based assistance.


This project was also  made possible with the assistance of [GPT-3](https://openai.com/gpt-3) developed by OpenAI and its talented team of developers.
