use diesel::prelude::*;
use crate::schema::posts;

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32, 
    pub title: String,
    pub body: String, 
    pub published: i32
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: &'a i32,
}