#[macro_use] extern crate diesel;
use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::PgConnection;

mod models;
mod schema;
mod lib;

use models::NewPost;
use lib::*;

fn main(){
    let connection = establish_connection();

    let title = lib::get_input("title: ".to_string());
    let body = lib::get_input("body: ".to_string());

    NewPost::create_post(&connection, title, body);

}

fn establish_connection()->PgConnection {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE URL must be set");
    PgConnection::establish(&database_url)
        .expect("error establishing connection")
}