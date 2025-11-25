-- Your SQL goes here
CREATE TABLE Class (
                       id Integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                       altervista_id Integer,
                       year_start Integer NOT NULL,
                       section Char(1) NOT NULL
);
CREATE TABLE Course (
                        id Integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                        name Varchar
);
CREATE TABLE Class_Course (
                              Class_id Integer NOT NULL,
                              Course_id Integer NOT NULL,
                              PRIMARY KEY (Class_id,Course_id),
                              FOREIGN KEY(Class_id) REFERENCES Class(id),
                              FOREIGN KEY(Course_id) REFERENCES Course(id)
);
CREATE TABLE Student (
                         id Integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                         altervista_id Integer,
                         first_name Varchar NOT NULL,
                         middle_name Varchar
);
CREATE TABLE Enrolled (
                          Student_id Integer NOT NULL,
                          Class_id   Integer NOT NULL,
                          date_start Date    NOT NULL,
                          date_end   Date,
                          PRIMARY KEY (student_id, Class_id),
                          FOREIGN KEY (Class_id) REFERENCES Student (id),
                          FOREIGN KEY (Student_id) REFERENCES Class (id)
);