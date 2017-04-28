

use std::collections::HashSet;
use story::StoryNode;

pub fn validate_story(story_tree: &Vec<StoryNode>) -> Result<(), String> {
    let mut seen_nodes: HashSet<String> = HashSet::new();

    for node in story_tree {
        if !seen_nodes.contains(&node.id) {
            seen_nodes.insert(node.id.clone());
        }
    }

    for node in story_tree.iter() {
        for answer in node.answers.iter() {
            match answer.next {
                Some(ref s) => {
                    if !seen_nodes.contains(s) {
                        return Err(format!("Answer's next Node is unknown '{}'", s));
                    }
                }
                None => {}
            }
        }
    }

    Ok(())
}
