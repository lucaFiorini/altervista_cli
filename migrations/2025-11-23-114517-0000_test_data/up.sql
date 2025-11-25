-- Your SQL goes here
CREATE TABLE Subject(
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(45) NOT NULL
);

CREATE TABLE Test(
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    subject_id INTEGER NOT NULL,
    name VARCHAR(45) NOT NULL,
    FOREIGN KEY (subject_id) REFERENCES Subject(id)
);

CREATE TABLE Assignment(
    test_id Integer NOT NULL,
    class_id Integer NOT NULL,
    scheduled_on DATE,
    PRIMARY KEY (test_id,class_id),
    FOREIGN KEY (test_id) REFERENCES Test(id),
    FOREIGN KEY (class_id) REFERENCES Test(id)
)