use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmitArgs {
    pub title: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoundSubmit {
    pub submit: SubmitArgs,
    pub rr: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitWithTime {
    pub title: String,
    pub name: String,
    pub description: String,
    pub date: String,
    pub time: String,
}