
extern crate storylet;

use storylet::load::load_story;

#[test]
fn test_basic_deserialise() {
    let story = match load_story("tests/test_story.json") {
        Ok(story) => story,
        Err(reason) => panic!(reason),
    };

    assert_eq!(story.len(), 1);
    assert_eq!(story[0].content, "Test story");
}
