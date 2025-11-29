use crate::register::register_item;
use register::{HasRegistry, AltervistaId, AltervistaItem, Register};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock,RwLock}
};
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};


#[derive(Debug,Clone,Serialize,Deserialize,Register)]
pub struct Course {
    #[register(id)]
    altervista_id: AltervistaId,
    name: String
}

#[derive(Debug,Clone,Serialize,Deserialize,Register)]
pub struct Student {
    #[register(id)]
    altervista_id: AltervistaId,
    name : String,
    middle_name : String,
    surname : String,
}

#[derive(Debug,Clone,Serialize,Deserialize,Register)]
pub struct Class {
    #[register(id)]
    altervista_id: AltervistaId,
    #[serde(rename="course_id")]
    pub(crate) course : AltervistaItem<Course>,
    students: Vec<AltervistaItem<Student>>,
    year_held: u16,
    class_year: u8,
    section : char,
}

impl Display for Class{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{altervista_id: {},course: TODO, year_start: {}, section: {}}}", self.altervista_id, self.year_held, self.section)
    }
}
impl Class {
    pub fn new(altervista_id: AltervistaId, course: AltervistaItem<Course>, year_held: u16, class_year : u8, section: char, students: Vec<AltervistaItem<Student>>) -> Self{
        Self{
            altervista_id,
            course:course.clone(),
            year_held,
            section,
            class_year,
            students
        }
    }
}
