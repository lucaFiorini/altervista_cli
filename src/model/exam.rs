use crate::model::*;

pub struct ExamPart {
    name : String,
    max_score : Score
}
impl ExamPart{
    pub fn new(name: String, max_score: Score) -> Self{
        ExamPart{name,max_score}
    }
}

pub struct Exam {
    name : String,
    parts : Vec<ExamPart>
}
