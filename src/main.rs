#[macro_use]
extern crate diesel;

use std::{env, io};
use diesel::{PgConnection, Connection};
use dotenv::dotenv;

mod schema;
mod model;

use model::{Student,NewStudent};

enum Cli {
    Get,
    Add,
    Delete,
    // Update(),
}
fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("database url must be provided");
    let conn = PgConnection::establish(&database_url).expect("error connecting to database");
    
    let arg: Vec<String> = env::args().collect();
    
    let command = match arg[1].as_str() {
        "get" => Cli::Get,
        "add" => Cli::Add,
        "delete" => Cli::Delete,
        _=> panic!("invalid command")
    };



    match command {
        Cli::Get => println!("{:?}",Student::list_all(&conn)),
        Cli::Add => println!("successfully insert new student: {:?}",Student::insert(get_input(), &conn)),
        Cli::Delete => println!("{}",Student::delete(get_id(), &conn)),       
    }
}

fn get_input()-> NewStudent {
    println!("pleas input name: ");
    let mut name = String::new(); 
    io::stdin().read_line(&mut name).expect("error reading name");
 
    println!("pleas input subject: ");
    let mut subject = String::new(); 
    io::stdin().read_line(&mut subject).expect("error reading sunject");
    
    println!("pleas input gender: ");
    let mut gender = String::new(); 
    io::stdin().read_line(&mut gender).expect("error reading gender");
    
    println!("Are you avalaible for evening shift: ");
    let mut status = String::new(); 
    io::stdin().read_line(&mut status).expect("error reading shift status");
    let status: char = status.trim().parse().unwrap();
    
    let state =  match status {
         'y' | 'Y' => true,
         _ => false,
     };
 
     //instance for our new student
     NewStudent {
         student_name: name,
         subject_name: Some(subject),
         gender: gender,
         avilability: state,
     }
}

fn get_id()-> i32 {
    println!("pleas input id to be delete: ");
    let mut id = String::new(); 
    io::stdin().read_line(&mut id).expect("error reading name");
    let id_i32: i32 = id.trim().parse().unwrap();
    id_i32
}
