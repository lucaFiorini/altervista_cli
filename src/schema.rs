// @generated automatically by Diesel CLI.

diesel::table! {
    assignment (test_id, class_id) {
        test_id -> Int4,
        class_id -> Int4,
        scheduled_on -> Nullable<Date>,
    }
}

diesel::table! {
    class (id) {
        id -> Int4,
        altervista_id -> Int4,
        year_start -> Int4,
        #[max_length = 1]
        section -> Bpchar,
    }
}

diesel::table! {
    class_course (class_id, course_id) {
        class_id -> Int4,
        course_id -> Int4,
    }
}

diesel::table! {
    course (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    student (id) {
        id -> Int4,
        altervista_id -> Nullable<Int4>,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    subject (id) {
        id -> Int4,
        #[max_length = 45]
        name -> Varchar,
    }
}

diesel::table! {
    test (id) {
        id -> Int4,
        subject_id -> Int4,
        #[max_length = 45]
        name -> Varchar,
    }
}

diesel::joinable!(class_course -> class (class_id));
diesel::joinable!(class_course -> course (course_id));
diesel::joinable!(test -> subject (subject_id));

diesel::allow_tables_to_appear_in_same_query!(
    assignment,
    class,
    class_course,
    course,
    student,
    subject,
    test,
);
