extern crate iron;
extern crate router;
extern crate logger;
extern crate chrono;
extern crate env_logger;
// extern crate rustc_serialize;
extern crate serde;
extern crate uuid;
extern  crate serde_json;

mod models;
mod handlers;
mod database;

use models::*;
use handlers::*;
use database::Database;

use iron::Iron;
use iron::prelude::Chain;
use router::Router;
use logger::Logger;
use uuid::Uuid;

fn main(){
    env_logger::init();
    let (logger_befor, logger_after) = Logger::new(None);

    let mut db = Database::new();

    let p = Post::new (
        "the first post",
        "first post in our API",
        "Osama",
        chrono::offset::Utc::now(),
        Uuid::new_v4()
    );
    db.add_post(p);

    let p2 = Post::new (
        "the second post",
        "second post in our API",
        "Osama",
        chrono::offset::Utc::now(),
        Uuid::new_v4()
    );
    db.add_post(p2);

    let handlers = Handlers::new(db);
    let json_content_middleware = JsonAfterMiddleware;

    let mut router = Router::new();
    router.get("/post_feed", handlers.postfeed, "post_feed");
    router.post("/post", handlers.post_post, "post_post");
    router.get("/post/:id", handlers.post, "post");

    let mut chain = Chain::new(router);
    chain.link_before(logger_befor);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);


    Iron::new(chain).http("localhost:8000").unwrap();
}