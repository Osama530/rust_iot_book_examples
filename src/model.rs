use diesel::{prelude::*, query_dsl::methods::FilterDsl};
use crate::schema::students;
use students::dsl::students as all_students;

#[derive(Queryable, Debug)]
pub struct Student {
    id: i32,
    student_name: String,
    subject_name: Option<String>,
    gender: String,
    avilability: bool,
}

#[derive(Insertable, Debug)]
#[table_name="students"]
pub struct NewStudent {
    pub student_name: String,
    pub subject_name: Option<String>,
    pub gender: String,
    pub avilability: bool,  
} 

impl Student {
    pub fn list_all(conn: &PgConnection)-> Vec<Student> {
        all_students
            .load::<Student>(conn)
            .expect("error listing all the students")
    }

    fn list_by_name(name: String, conn: &PgConnection)-> Vec<Student> {
        FilterDsl::filter(all_students, students::student_name.eq(name))
            .load::<Student>(conn)
            .expect("no student with that name")
    }

    pub fn list_by_subject(sub: String, conn: &PgConnection)-> Vec<Student> {
        FilterDsl::filter(all_students, students::subject_name.eq(sub))
            .load::<Student>(conn)
            .expect("no student with subject")
    }

    pub fn insert(new_student: NewStudent, conn: &PgConnection)-> NewStudent {
        diesel::insert_into(students::table)
            .values(&new_student)
            .execute(conn);
        new_student
    }

    pub fn delete(id: i32, conn: &PgConnection)-> String {
        diesel::delete(all_students.find(id)).execute(conn).expect("error deleting student");
        "successfully deleted".to_string()
    }

   
}

