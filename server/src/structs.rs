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