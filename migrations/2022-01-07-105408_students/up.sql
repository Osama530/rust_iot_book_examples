-- Your SQL goes here
CREATE TABLE students (
    id SERIAL PRIMARY KEY,
    student_name VARCHAR(20) NOT NULL,
    subject_name VARCHAR(40),
    gender VARCHAR(20) NOT NULL,
    avilability BOOLEAN NOT NULL DEFAULT 'f'
);