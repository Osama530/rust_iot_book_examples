use crate::models::Post;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Database {
    posts: Vec<Post>,

}

impl Database {
    pub fn new()-> Database {
        Database {
            posts: vec![],
        }
    }

    pub fn add_post(&mut self, post: Post) {
        self.posts.push(post);use serde_json::{Result, Value};
    }

    pub fn posts(&self)-> &Vec<Post> {
        &self.posts
    }
    
}
