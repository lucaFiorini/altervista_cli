pub mod class;
pub mod exam;
pub mod grading;

type Score = i16;
type SQLId = u32;
type AltervistaId = u32;

struct School {
    class: class::CLass,

}