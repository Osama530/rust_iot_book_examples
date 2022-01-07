use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::movies;
use movies::dsl::movies as all_movies;

#[derive(Queryable, Debug, PartialEq)]
pub struct Movie {
    movie_id: i32,
    movie_name: String,
    movie_gener: String,
    // idb_rating: f32,
    published: bool,
}

#[derive(Insertable)]
#[table_name = "movies"]
pub struct NewMovie {
    pub movie_name: String,
    pub movie_gener: String,
    // pub idb_rating: f32,
    pub published: bool,
}

impl Movie {
    pub fn show_all(conn: &PgConnection)-> Vec<Movie> {
        all_movies
            .order(movies::movie_id.desc())
             .load::<Movie>(conn)
            .expect("error listing all the items")
    }

    pub fn show_by_id(id: i32, conn: &PgConnection)-> Vec<Movie> {
        all_movies
            .find(id)
            .load::<Movie>(conn)
            .expect("no item with id")
    }

    pub fn show_by_status(status: bool, conn: &PgConnection)-> Vec<Movie> {
        all_movies
            .filter(movies::published.eq(status))
            .load::<Movie>(conn)
            .expect("errrror finding with status")
    }

    // pub fn show_by_rating (rating: f32, conn: &PgConnection)-> Vec<Movie> {
    //     all_movies
    //         .filter(movies::idb_rating.eq(rating))
    //         .load::<Movie>(conn)
    //         .expect("error loading data")
    // }

    pub fn insert(movie: NewMovie, conn: &PgConnection)-> bool {
        diesel::insert_into(movies::table)
            .values(&movie)
            .execute(conn)
            .is_ok()
    }

    fn update_id(id: i32, conn: &PgConnection, name: String, status: bool )-> bool {

        diesel::update(all_movies.find(id))
            .set((
                    // movies::dsl::movie_gener.eq(gener),
                   movies::dsl::movie_name.eq(name),
                //    movies::dsl::idb_rating.eq(rating),
                   movies::dsl::published.eq(status)     ))
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection)-> bool {
        if Movie::show_by_id(id, conn).is_empty() {
            return false
        }
        
        diesel::delete(all_movies.find(id))
            .execute(conn)
            .is_ok()
    }
}



