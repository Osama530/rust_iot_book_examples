#[macro_use] extern crate diesel;
// use dotenv::dotenv;
use model::Movie;
use std::env;

use diesel::prelude::*;
use diesel::PgConnection;

mod schema;
mod model;

use model::NewMovie;

fn main(){

    let database_url = env::var("DATABASE_URL")
        .expect("database url must be provided");

    let conn = PgConnection::establish(&database_url)
        .expect("error establishing connection to database");

        let movie_01 = NewMovie {
            movie_name: "spider man".to_string(),
            movie_gener: "Action, SciFi".to_string(),
            // idb_rating: 7.2,
            published: true,
        };

        if Movie::insert(movie_01, &conn) {
            println!("success inserting");
        }
        else {
            println!("error inserting to database");
        }

}
