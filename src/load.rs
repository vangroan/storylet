

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use de::deserialise;
use story::StoryNode;


pub fn load_story(filepath: &str) -> Result<Vec<StoryNode>, String> {
    let path = Path::new(filepath);
    let display = path.display();

    // Open file
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(err) => return Err(format!("Could not open file {}: {}", display, err.description())),
    };

    // Load file content into a string
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!("Could not read from file {}: {}",
                               display,
                               err.description()))
        }
    };

    // Deserialize story
    let story: Vec<StoryNode> = match deserialise(content.as_str()) {
        Ok(nodes) => nodes,
        Err(err) => return Err(format!("Could not deserialise {}: {}", display, err.description())),
    };

    Ok(story)
}
