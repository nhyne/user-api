#![feature(proc_macro_hygiene)]
// Rocket web server
#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
mod db;
use db::user::{NewUser, User, RocketNewUser};
use rocket_contrib::json::{Json, JsonValue};

// Diesel ORM
#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;
use std::ops::Deref;

#[post("/new", format = "json", data = "<input_user>")]
fn new(input_user: Json<RocketNewUser>) -> JsonValue {
    use db::schema::users;
    let connection = establish_connection();
    let new_user = NewUser::new(
        // I genuinely feel ashamed for doing this
        input_user.email.clone(),
        input_user.password.clone(),
        input_user.username.clone(),
    );
    let created_user: Result<User, Error> = diesel::insert_into(users::table)
        .values(new_user)
        .get_result(&connection);
    match created_user {
        Ok(user) => json!({"something": "cool"}),
        Err(e) => json!("{message: bad}"),
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api/users", routes![new])
}

// TODO: Use database pooling
fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    NewUser::new(String::from("something@somewhere.io"), String::from("abadpass"), String::from("username"));
    rocket().launch();
}
