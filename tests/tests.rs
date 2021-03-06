
extern crate storylet;

use storylet::load::load_story;
use storylet::runner::StoryRunner;

#[test]
fn test_basic_deserialise() {
    let story = match load_story("tests/test_story.json") {
        Ok(story) => story,
        Err(reason) => panic!(reason),
    };

    assert_eq!(story.len(), 2);
    assert_eq!(story[0].content, "Test story");
}

#[test]
fn test_validation_error() {
    let story = match load_story("tests/test_faulty_story.json") {
        Ok(story) => story,
        Err(reason) => panic!(reason),
    };

    let result = storylet::validation::validate_story(&story);

    assert!(result.is_err());
}

#[test]
fn test_story_runner_get_current() {
    let story = load_story("tests/test_story.json").unwrap();
    let runner = StoryRunner::new(story);

    assert_eq!(runner.has_current(), true);
    assert_eq!(runner.borrow_current().unwrap().id, "test_story_intro");
}

#[test]
fn test_story_runner_advance_to_nex() {
    let story = load_story("tests/test_story.json").unwrap();
    let mut runner = StoryRunner::new(story);
    runner.next_by_answer_idx(0).unwrap();

    assert_eq!(runner.borrow_current().unwrap().id, "test_story_prologue");
}
