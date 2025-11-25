use std::io;
use crate::model::class::*;

pub enum State {
    Main,
    CreatingExam,
    AssigningExam(Exam),
    SelectingClass,
    SelectingTest(Class),
    GradingTest(Student)
}

fn load_class_data() {
    let data =
}
// The main bit
fn run_cli(){
    let mut state = State::Main;

    /*
    fn main() -> io::Result<()> {
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        Ok(())
    }
    */
}