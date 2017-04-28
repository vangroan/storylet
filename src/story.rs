
use serde::de::Deserialize;


#[derive(Deserialize, Debug)]
pub struct StoryNode {
    pub id: String,
    pub content: String,
}

pub struct StoryRunner {}
