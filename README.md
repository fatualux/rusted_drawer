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

***Logic:***

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

 ***Acknowledgements***

Special thanks to the Codeium engineering team for their assistance and support in developing this project. Their AI-powered assistant has been instrumental in providing quick and accurate coding help.

- [@codeium-assistant](https://github.com/codeium-assistant) - The AI assistant from Codeium that helped answer code-related questions.
- [Codeium](https://codeium.com/) - The AI company that developed the assistant, providing code autocomplete, search, and chat-based assistance.


This project was also  made possible with the assistance of [GPT-3](https://openai.com/gpt-3) developed by OpenAI and its talented team of developers.
