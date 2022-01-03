use diesel::{self, RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::pg::PgConnection;
use diesel::Queryable;

use crate::schema::book;
use book::dsl::book as all_books;

#[derive(Queryable, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub auther: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "book"]
pub struct NewBook {
    pub title: String,
    pub auther: String,
    pub published: bool,
}

impl Book {
    fn show(id: i32, conn: &PgConnection)-> Vec<Book>   {
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("Error loading book")
    }

    pub fn all(conn: &PgConnection)-> Vec<Book> {
        all_books
            .order(book::id.desc())
            .load::<Book>(conn)
            .expect("Error loading book")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, book: NewBook)-> bool {
        use book::dsl::{ auther as a , title as t , published as p};

        let NewBook {
            auther,
            title,
            published
        }   = book;

        diesel::update(all_books.find(id))
            .set((a.eq(auther), p.eq(published), t.eq(title)))
            .get_result::<Book>(conn)
            .is_ok()
    }
    
    pub fn insert(book: NewBook, conn: &PgConnection)-> bool {
        diesel::insert_into(book::table)
            .values(&book)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection)-> bool {
        if Book::show(id, conn).is_empty() {
            false
        }
        else {
            diesel::delete(all_books.find(id))
                .execute(conn)
                .is_ok()
        }
    }

    pub fn all_by_auther(auther: String, conn: &PgConnection)-> Vec<Book> {
        all_books
            .filter(book::auther.eq(auther))
            .load::<Book>(conn)
            .expect("error loading books")
    }
}
