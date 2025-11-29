use serde::{Deserialize, Serialize};
use crate::model::*;

#[derive(Serialize,Deserialize)]
pub struct ExamPart {
    name : String,
    max_score : Score
}

impl ExamPart{
    pub fn new(name: String, max_score: Score) -> Self{
        ExamPart{name,max_score}
    }
}

#[derive(Serialize,Deserialize)]
pub struct Exam {
    name : String,
    parts : Vec<ExamPart>
}

impl Exam {
    pub fn get_name(&self) -> &String {&self.name}
    pub fn get_parts(&self) -> &Vec<ExamPart> {&self.parts}
}