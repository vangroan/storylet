
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


