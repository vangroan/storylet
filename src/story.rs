
use serde::de::Deserialize;


#[derive(Deserialize, Debug)]
pub struct StoryNode {
    pub id: String,
    pub content: String,
    pub answers: Vec<StoryAnswer>,
}


#[derive(Deserialize, Debug)]
pub struct StoryAnswer {
    pub id: String,
    pub content: String,
    pub next: Option<String>,
}


/// Holds the state of the story
pub struct StoryRunner {
    /// The currently active node in the story. When the Option is None, no Node
    /// is active.
    pub current_idx: Option<usize>,

    /// The story the StoryRunner is working with. The loaded story must not be 
    /// changed while the StoryRunner owns it.
    pub story: Vec<StoryNode>,
}


impl StoryRunner {
    pub fn new(story: Vec<StoryNode>) -> Self {
        let mut runner = StoryRunner {
            current_idx: None,
            story: story,
        };

        runner.init();

        runner
    }

    fn init(&mut self) {
        // Set the first node as current
        if !self.story.is_empty() {
            self.current_idx = Some(0);
        }
    }

    pub fn has_current(&self) -> bool {
        !self.current_idx.is_none()
    }

    pub fn borrow_current(&self) -> Option<&StoryNode> {
        match self.current_idx {
            Some(idx) => Some(&self.story[idx]),
            None => None
        }
    }
}
