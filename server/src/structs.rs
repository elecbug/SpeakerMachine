use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct SubmitArgs {
    pub title: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct RoundSubmit {
    pub submit: SubmitArgs,
    pub rr: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitWithTime {
    pub title: String,
    pub name: String,
    pub description: String,
    pub date: String,
    pub time: String,
}