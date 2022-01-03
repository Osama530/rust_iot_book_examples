#[macro_use] extern crate diesel;

use diesel::{PgConnection, Connection};
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;


fn main() {
    dotenv().ok();

    let database = env::var("DATABASE_URL").expect("DATABASE_URL");
    let conn = PgConnection::establish(&database).unwrap();

    let book = models::NewBook { 
        title: "A_iot".to_string(),
        auther: "osama".to_string(),
        published: true,
    };

    if models::Book::insert(book, &conn) {
        println!("success");
    } else {
        println!("failed");
    }

    let books = models::Book::all(&conn);
    println!("{:?}",books);


    

}
