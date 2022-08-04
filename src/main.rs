#![feature(decl_macro)]
#[macro_use]
extern crate diesel;

use std::time::SystemTime;

use diesel::prelude::*;
use diesel::Queryable;
use rocket::{self, get, post, put, routes};
use rocket_contrib::databases::{database, diesel::PgConnection};
use rocket_contrib::json::Json;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

mod schema;

use crate::schema::scores;

#[database("postgres")]
struct DbConn(PgConnection);

// #[derive(Queryable, Serialize)]
// struct User {
//     id: i32,
//     name: String,
//     email: String,
// }

#[derive(Queryable, Serialize)]
struct Score {
    id: i32,
    user_id: i32,
    score: i16,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "scores"]
struct NewScore {
    user_id: i32,
    score: i16,
}

#[post("/", data = "<new_score>")]
fn create_score(conn: DbConn, new_score: Json<NewScore>) -> Json<Score> {
    let result = diesel::insert_into(scores::table)
        .values(&new_score.0)
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

#[get("/")]
fn get_scores(conn: DbConn) -> Json<Vec<Score>> {
    let scores = scores::table
        .order(scores::columns::id.desc())
        .load::<Score>(&*conn)
        .unwrap();
    Json(scores)
}

#[put("/<id>", data = "<updated_score>")]
fn update_score(conn: DbConn, id: i32, updated_score: String) -> Json<Score> {
    let new_value = i16::from_str(&updated_score).unwrap(); // TODO: fix this
    let target = scores::table.filter(scores::columns::id.eq(id));
    let result = diesel::update(target)
        .set(scores::columns::score.eq(new_value))
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/hello", routes![index])
        .mount("/scores", routes![create_score, get_scores, update_score])
        .launch();
}
