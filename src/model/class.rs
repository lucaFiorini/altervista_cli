use std::rc::Rc;
use crate::model::{AltervistaId, SQLId};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::course)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Course {
    sql_id : SQLId,
    altervista_id: AltervistaId
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::class)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CLass {
    sql_id : SQLId,
    altervista_id: AltervistaId,
    course : Rc<Course>,
    year_start : u32,
    section : char,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::student)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Student {
    sql_id : SQLId,
    altervista_id: AltervistaId,
    name : String,
    class : Rc<CLass>
}

