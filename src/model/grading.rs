use crate::model::*;
use exam::*;

trait Graded{
    fn get_score(&self) -> Score;
    fn get_comment(&self) -> &Option<String>;
}

struct Grade {
    score : Score,
    comment : Option<String>
}
impl Grade {
    fn new(score : Score, comment : Option<String>) -> Self {
        Grade{score,comment}
    }
}

impl Graded for Grade{
    fn get_score(&self) -> Score {self.score}
    fn get_comment(&self) -> &Option<String> { &self.comment }
}

struct GradedExamPart<'a>{
    exam_part: &'a ExamPart,
    grade : Grade
}
impl<'a> Graded for GradedExamPart<'a> {
    fn get_score(&self) -> Score {self.grade.score}
    fn get_comment(&self) -> &Option<String> { &self.grade.comment }
}
impl <'a> GradedExamPart<'a> {
    fn new(exam_part: &ExamPart,grade: Grade){
        GradedExamPart{exam_part,grade};
    }
}

struct GradedExam<'a> {
    exam_definition : &'a Exam,
    graded_parts : Vec<GradedExamPart<'a>>,
    comment : Option<String>
}
impl<'a> Graded for GradedExam<'a> {
    fn get_score(&self) -> Score {
        self.graded_parts.iter().
            fold(0 ,|acc,next| acc + next.get_score())
    }

    fn get_comment(&self) -> &Option<String> { &self.comment }
}
