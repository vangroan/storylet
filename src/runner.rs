

use std::collections::HashMap;
use feed::StoryFeed;
use story::StoryNode;


/// Holds the state of the story
pub struct StoryRunner {
    /// The currently active node in the story. When the Option is None, no Node
    /// is active.
    current_idx: Option<usize>,

    /// The story the StoryRunner is working with. The loaded story must not be
    /// changed while the StoryRunner owns it.
    feed: StoryFeed,
}


impl StoryRunner {
    pub fn new(feed: StoryFeed) -> Self {
        let mut runner = StoryRunner {
            current_idx: None,
            feed: feed,
        };

        runner.init();

        runner
    }

    fn init(&mut self) {
        // Set the first node as current
        if !self.feed.is_empty() {
            self.current_idx = Some(0);
        }
    }

    pub fn has_current(&self) -> bool {
        !self.current_idx.is_none()
    }

    pub fn borrow_current(&self) -> Option<&StoryNode> {
        match self.current_idx {
            Some(idx) => Some(&self.feed[idx]),
            None => None,
        }
    }

    /// Advances the story by the index of the answer in the node.
    pub fn next_by_answer_idx(&mut self, answer_idx: usize) -> Result<(), String> {
        if self.current_idx.is_none() {
            return Err("Runner has no current node".to_owned());
        }

        let answers = &self.feed[self.current_idx.unwrap()].answers;
        if answer_idx >= answers.len() {
            return Err("Answer index argument is out of bounds".to_owned());
        }

        match answers[answer_idx].next {
            Some(ref node_id) => self.current_idx = Some(self.feed.id_to_index(node_id)),
            None => self.current_idx = None,
        }

        Ok(())
    }
}
