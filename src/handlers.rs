use std::sync::{Arc,Mutex};
use std::io::Read;
use iron::{status, Request, Response, Handler, AfterMiddleware, IronResult };
use iron::headers::ContentType;
// use rustc_serialize::json;
use router::Router;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::database::Database;
use crate::uuid::Uuid;
use crate::models::Post;
use std::error::Error;

macro_rules! tyr_handler {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with(status::InternalServerError))
        }        
    };
    ($e:expr, $error:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with(($error, e.description())))
        }
    }
}

macro_rules! lock {
    ($e:expr) => { e.lock().unwrap()}
}

macro_rules! get_http_param {
    ($r:expr, $e:expr) => {
        match $r.extensions.get::<Router>() {
            Some(router) => {
                match router.find($e) {
                    Some(v) => v,
                    None => return Ok(Response::with(status::BadRequest)),
                }
            },
            None => return Ok(Response::with((status::InternalServerError))),

        }
    };
}

#[warn(deprecated)]

pub struct Handlers {
    pub postfeed: PostFeedHandler,
    pub post_post: PostPostHandler,
    pub post: PostHandler,
}

impl Handlers {
    pub fn new(db: Database)-> Handlers {
        let database = Arc::new(Mutex::new(db));
        Handlers {
            postfeed: PostFeedHandler::new(database.clone()),
            post_post: PostPostHandler::new(database.clone()),
            post: PostHandler::new(database.clone()),
        }

    }    
}

pub struct PostFeedHandler {
    database: Arc<Mutex<Database>>
}

impl PostFeedHandler {
    fn new(database: Arc<Mutex<Database>>)-> PostFeedHandler {
        PostFeedHandler {
            database
        }        
    }
    
}

impl Handler for PostFeedHandler {
    fn handle(&self, _:&mut Request)-> IronResult<Response> {
        let payload = tyr_handler!(serde_json::to_string(self.database.lock().unwrap().posts())); 
        Ok(Response::with((status::Ok, payload)))
    }
}

pub struct PostPostHandler {
    database: Arc<Mutex<Database>>
}

impl PostPostHandler {
    fn new (database: Arc<Mutex<Database>>)-> PostPostHandler {
        PostPostHandler {
            database
        }
    }

}

impl Handler for PostPostHandler {
    fn handle (&self, req: &mut Request)-> IronResult<Response> {
        let mut payload = String::new();
        tyr_handler!(req.body.read_to_string(&mut payload));
        
        let post = tyr_handler!(serde_json::from_str(&payload));
        
        self.database.lock().unwrap().add_post(post);
        Ok(Response::with((status::Created, payload)))
    }
}

pub struct PostHandler {
    database: Arc<Mutex<Database>>
}

impl PostHandler {
    fn new(database: Arc<Mutex<Database>>)->PostHandler {
        PostHandler {
            database
        }
    }
    
    fn find_post(&self, id: &Uuid)-> Option<Post> {
        let locked = self.database.lock().unwrap();
        let mut iterator = locked.posts().iter();
        iterator.find(|p| p.uuid() == id ).map(|p| p.clone())
    }
}

impl Handler for PostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref post_id = get_http_param!(req, "id");
        
        let id = tyr_handler!(Uuid::parse_str(post_id));

        if let Some(post) = self.find_post(&id)  {
            let payload = tyr_handler!(serde_json::to_string(&post));
            Ok(Response::with((status::Ok, payload)))           
        }
        else {
            Ok(Response::with((status::NotFound)))
        }
    }
}

pub struct JsonAfterMiddleware;

impl AfterMiddleware for JsonAfterMiddleware {
    fn after(&self, _:&mut Request, mut res: Response)-> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)
    }
}