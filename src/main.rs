#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
mod db;
use db::user::{NewUser, RocketLogin, RocketNewUser, User};
use rocket_contrib::json::{Json, JsonValue};
use uuid::Uuid;

extern crate openssl;
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
        // TODO: This should not need to be a clone, just make the function take a pointer
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

#[post("/login", format = "json", data = "<login_attempt>")]
fn login(login_attempt: Json<RocketLogin>) -> JsonValue {
    use db::schema::users;
    let connection = establish_connection();

    // select user
    let selected_user_vec: Result<User, Error> = users::table
        .filter(users::email.eq(&login_attempt.email))
        .first(&connection);

    println!("{:#?}", selected_user_vec);
    match selected_user_vec {
        Ok(user) => {
            let successful_login = User::validate_password(&user, &login_attempt.password);
            if successful_login {
                json!("{message: good}")
            } else {
                json!("{message: bad password}")
            }
        }
        Err(_) => json!("{message: bad}"),
    }

    // run login attempt

    // TODO: Should return a JWT that has the information the user will need
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api/users", routes![new, login])
}

// TODO: Use database pooling
fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    rocket().launch();
}
