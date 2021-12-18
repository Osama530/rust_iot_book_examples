use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use serde::{ Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    title: String,
    body: String,
    auther: String,
    datetime: DateTime<Utc>,
    uuid: Uuid
}

impl Post {
    // assosiated function
    pub fn new(title: &str, body: &str, auther: &str, datetime: DateTime<Utc>, uuid: Uuid)-> Post {
        Post {
            title: title.to_string(),
            body: body.to_string(),
            auther: auther.to_string(),
            datetime,
            uuid,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        self.uuid()
    }
}