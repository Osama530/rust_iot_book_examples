use iron::prelude::*;
use iron::{error::IronError,status, Url};
use iron::modifiers::Redirect;

use params::{Params, Value, Map};

const COMMENT_FIELD: &str = "My Comment"; //Example 2

fn add_server(req: &mut Request) -> IronResult<Response> {
    
    let map = req.get_ref::<Params>().unwrap();
    Ok(Response::with((status::Ok, get_comment(&map, "coool"))))
}

fn main() {
    // Iron::new(qurie_handler) //example 1
    Iron::new(add_server)
        .http("localhost:3000").unwrap();
}

// Parsing parameters examaple 2a
fn get_comment(map: &Map, set_value: &str)-> String {
    let x: &str = match map.find(&[COMMENT_FIELD]).unwrap() {
        Value::String(set_value) => set_value, //sets the value as string(or can be anything) for the key found(COMMENT_FIELD)
        // i.e: http://localhost:3000/?My Comment=coool //return ok with response coool
        _ => "none",
    };
    String::from(x)

}

// Parsing parameters example 1
fn qurie_handler(req: &mut Request)-> IronResult<Response> {
    
    let maping_data = req.get_ref::<Params>().unwrap();

    match maping_data.find(&["user"]) {
        Some(&Value::String(ref name)) if name == "Osama" => {
            Ok(Response::with((status::Ok, "Welcome back, Osama!")))
        },
        _ => Ok(Response::with(iron::status::NotFound)),
        
    }

}

// let url = Url::parse("http://osama_iot.com").unwrap();
// Iron::new( move |_: &mut Request|
//     Ok(Response::with((status::NoContent,Redirect(url.clone())))))
//     .http("localhost:3000");