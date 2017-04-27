
extern crate storylet;


use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use storylet::load::load_story;
use storylet::story::StoryNode;


#[test]
fn test_basic_deserialise() {
    let story = match load_story("tests/test_story.json") {
        Ok(story) => story,
        Err(reason) => panic!(reason),
    };

    assert_eq!(story.len(), 1);
    assert_eq!(story[0].title, "Test story");
}
