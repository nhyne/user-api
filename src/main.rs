#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
mod db;
mod responses;
use db::user::{Claims, NewUser, RocketLogin, RocketNewUser, Token, User};
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, Validation};
use rocket_contrib::json::{Json, JsonValue};


use std::env;

extern crate openssl;
// Diesel ORM
#[macro_use]
extern crate diesel;


use diesel::prelude::*;
use diesel::result::Error;


use rocket::http::Status;
use rocket::response::status::Custom;

#[post("/new", format = "json", data = "<input_user>")]
fn new(input_user: Json<RocketNewUser>) -> Result<Json<User>, Custom<Json<responses::Error>>> {
    use db::schema::users;
    let connection = db::establish_connection();
    let new_user = NewUser::new(
        // TODO: This should not need to be a clone, just make the function take a pointer
        // I genuinely feel ashamed for doing this
        input_user.email.clone(),
        input_user.password.clone(),
        input_user.username.clone(),
    );
    match new_user {
        Ok(user) => {
            let created_user: Result<User, Error> = diesel::insert_into(users::table)
                .values(user)
                .get_result(&connection);
            match created_user {
                Ok(inserted_user) => Ok(Json(inserted_user)),
                Err(e) => Err(Custom(
                    Status::InternalServerError,
                    Json(responses::Error {
                        message: e.to_string(),
                    }),
                )),
            }
        }
        Err(e) => Err(Custom(Status::InternalServerError, Json(e))),
    }
}

#[post("/login", format = "json", data = "<login_attempt>")]
fn login(login_attempt: Json<RocketLogin>) -> Json<Token> {
    use db::schema::users;
    let connection = db::establish_connection();
    // select user
    let selected_user_vec: Result<User, Error> = users::table
        .filter(users::email.eq(&login_attempt.email))
        .first(&connection);

    match selected_user_vec {
        Ok(user) => {
            let login_token = User::login_and_receive_jwt(&user, &login_attempt.password);
            Json(login_token)
        }
        Err(_) => {
            let login_token = Token {
                token: String::from(""),
            };
            Json(login_token)
        }
    }
}

#[post("/verify_jwt", format = "json", data = "<jwt>")]
fn verify_jwt(jwt: Json<Token>) -> JsonValue {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let valid_token = decode::<Claims>(
        &jwt.token,
        jwt_secret.as_bytes(),
        &Validation::new(Algorithm::default()),
    );
    match valid_token {
        Ok(_token) => json!("{valid: true}"),
        Err(_) => json!("{valid: false}"),
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api/users", routes![new, login, verify_jwt])
}

fn main() {
    rocket().launch();
}
