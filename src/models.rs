use std::string;

use diesel::{Queryable, PgConnection};
use diesel::{self, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::schema::posts::{self, title, body};
use crate::schema::posts::dsl as all_posts;

#[derive(Queryable)]
pub struct Post {
    id: u32,
    title: String,
    body: String,
    published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    title: String,
    body: String,
}

impl NewPost {
    pub fn create_post(conn: &PgConnection ,t: String, b: String)-> usize {
        let new_post = NewPost {
            title: t,
            body: b,
        };

        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(conn)
            .expect("error")
        }  
}
