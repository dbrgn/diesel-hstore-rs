#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate diesel_hstore;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let connection = establish_connection();

    let results = schema::animals.load<models::Animal>(&connection)
                                 .expect("Error loading animals");

    println!("Displaying {} animals", results.len());
    for animal in results {
        println!("# {}: {}", animal.id, animal.name);
        println!("\n{:?}", animal.attributes);
    }
}
