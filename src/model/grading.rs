use serde::{Deserialize, Serialize};
use crate::model::*;
use exam::*;
use crate::model::class::Student;
use register::*;

trait Graded{
    fn get_score(&self) -> Score;
    fn get_comment(&self) -> &Option<String>;
}

trait GradableInto<T : Graded, G> {
    fn grade(self,grade : G) -> T;
}

#[derive(Serialize,Deserialize)]
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

#[derive(Serialize,Deserialize)]
struct GradedExam {
    student: AltervistaItem<Student>,
    exam_definition: Exam,
    graded_parts : Vec<GradedExamPart>,
    comment : Option<String>
}
impl GradedExam{
    fn new(exam: Exam, student: &AltervistaItem<Student>, graded_parts: Vec<GradedExamPart>, comment: Option<String>) -> GradedExam {
        assert_eq!(exam.get_parts().len(),graded_parts.len());
        GradedExam {
            exam_definition: exam,
            student: student.clone(),
            graded_parts,
            comment
        }
    }
}
impl Graded for GradedExam {
    fn get_score(&self) -> Score {
        self.graded_parts.iter().
            fold(0 ,|acc,next| acc + next.get_score())
    }

    fn get_comment(&self) -> &Option<String> { &self.comment }
}

#[derive(Serialize,Deserialize)]
struct GradedExamPart{
    exam_part: ExamPart,
    grade: Grade
}
impl Graded for GradedExamPart {
    fn get_score(&self) -> Score {self.grade.score}
    fn get_comment(&self) -> &Option<String> { &self.grade.comment }
}
impl GradedExamPart {
    fn new(exam_part: ExamPart,grade: Grade) -> Self{
        GradedExamPart{exam_part,grade}
    }
}

impl GradableInto<GradedExamPart,Grade> for ExamPart {
    fn grade(self, grade: Grade) -> GradedExamPart {
        GradedExamPart{exam_part: self,grade}
    }
}