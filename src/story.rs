
use serde::de::{Deserialize};


#[derive(Deserialize, Debug)]
pub struct StoryNode {
    pub title: String,
}
