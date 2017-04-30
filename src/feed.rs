

use std::collections::HashMap;
use std::slice::Iter;
use std::ops::Index;
use story::StoryNode;


pub struct StoryFeed {
    nodes: Vec<StoryNode>,

    /// Maps Node ids to vector indeces for quicker lookup.
    node_index: HashMap<String, usize>,
}

impl StoryFeed {
    pub fn new(story: Vec<StoryNode>) -> Self {
        let mut feed = StoryFeed {
            nodes: story,
            node_index: HashMap::new(),
        };

        feed.rebuild_index();

        feed
    }

    fn rebuild_index(&mut self) {
        self.node_index.clear();

        for (idx, node) in self.nodes.iter().enumerate() {
            self.node_index.insert(node.id.to_owned(), idx);
        }
    }

    pub fn borrow_node(&self, id: &str) -> Option<&StoryNode> {
        let idx = match self.node_index.get(id) {
            Some(&idx) => idx,
            None => return None, 
        };

        if idx >= self.nodes.len() {
            return None;
        }

        Some(&self.nodes[idx])
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn iter(&self) -> Iter<StoryNode> {
        self.nodes.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn id_to_index(&self, id: &str) -> usize {
        self.node_index.get(id).unwrap().clone()
    }
}


impl Index<usize> for StoryFeed {
    type Output = StoryNode;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}
