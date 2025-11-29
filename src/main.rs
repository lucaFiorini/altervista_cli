use std::{env, fs::File, io};
use std::sync::Arc;
use serde::{Deserialize, Deserializer};
use crate::model::class::*;
use crate::front::cli;
use register;
use register::{AltervistaItem, HasRegistry};

mod front;
mod model;

fn preload() {
    let path = dotenv::var("FILE_PATH")
        .expect("FILE_PATH must be set");
    let fs  = File::open(&path)
        .expect(format!("Trying to read \"{}\" - file not found",path).as_str());
    let deserialized : serde_json::Value  = serde_json::from_reader(fs).
        expect("File must be valis json");

    let courses: Vec<Arc<Course>> = Course::deserialize_and_register_arr(deserialized["courses"].clone()).unwrap();
    let students: Vec<Arc<Student>> = Student::deserialize_and_register_arr(deserialized["students"].clone()).unwrap();
    let classes: Vec<Arc<Class>> = Class::deserialize_and_register_arr(deserialized["classes"].clone()).unwrap();
    println!("ACTUALLY LOADED WTF");
    println!("Courses: {:?}",courses);
    println!("Students: {:?}",students);
    println!("Classes: {:?}",classes);
}

fn main() -> io::Result<()>{
    preload();
    cli::cli_run()
}