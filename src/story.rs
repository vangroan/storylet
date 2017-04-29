
use std::collections::HashMap;
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
    current_idx: Option<usize>,

    /// The story the StoryRunner is working with. The loaded story must not be
    /// changed while the StoryRunner owns it.
    story: Vec<StoryNode>,

    /// Maps Node ids to vector indeces for quicker lookup.
    node_index: HashMap<String, usize>,
}


impl StoryRunner {
    pub fn new(story: Vec<StoryNode>) -> Self {
        let mut runner = StoryRunner {
            current_idx: None,
            story: story,
            node_index: HashMap::new(),
        };

        runner.init();

        runner
    }

    fn init(&mut self) {
        // Set the first node as current
        if !self.story.is_empty() {
            self.current_idx = Some(0);
        }

        self.node_index.clear();

        // Build index
        for (idx, node) in self.story.iter().enumerate() {
            self.node_index.insert(node.id.to_owned(), idx);
        }
    }

    pub fn has_current(&self) -> bool {
        !self.current_idx.is_none()
    }

    pub fn borrow_current(&self) -> Option<&StoryNode> {
        match self.current_idx {
            Some(idx) => Some(&self.story[idx]),
            None => None,
        }
    }

    /// Advances the story by the index of the answer in the node.
    pub fn next_by_answer_idx(&mut self, answer_idx: usize) -> Result<(), String> {
        if self.current_idx.is_none() {
            return Err("Runner has no current node".to_owned());
        }

        let answers = &self.story[self.current_idx.unwrap()].answers;
        if answer_idx >= answers.len() {
            return Err("Answer index argument is out of bounds".to_owned());
        }

        match answers[answer_idx].next {
            Some(ref node_id) => self.current_idx = Some(self.id_to_index(node_id)),
            None => self.current_idx = None,
        }

        Ok(())
    }

    fn id_to_index(&self, id: &str) -> usize {
        self.node_index.get(id).unwrap().clone()
    }
}
