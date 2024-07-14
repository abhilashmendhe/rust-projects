#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{prelude::*, query_dsl::methods::FilterDsl};
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


use self::models::{NewPost, Post};

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) {
    use crate::schema::posts;

    let new_post = NewPost { 
        title: title,
        body: body,
        published: &0
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    // let results = posts::table
    //     .filter(posts::dsl::title.like(format!("%{}%",new_post.title)));
    
}
