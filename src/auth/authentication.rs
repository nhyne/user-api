use crate::db::user::Claims;
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, Validation};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use std::env;

pub struct AuthenticatedJWT(String);

#[derive(Debug)]
pub enum JWTError {
    Missing,
    Invalid,
    BadCount,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedJWT {
    type Error = JWTError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, JWTError::Missing)),
            1 if jwt_is_valid(keys[0]) => Outcome::Success(AuthenticatedJWT(keys[0].to_string())),
            1 => Outcome::Failure((Status::Unauthorized, JWTError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, JWTError::BadCount)),
        }
    }
}

fn jwt_is_valid(jwt: &str) -> bool {
    // The Auth headed should come in the form "Authorization: Bearer <token>".
    //   We're splitting to just validate the token.
    let split: Vec<&str> = jwt.split_whitespace().collect();
    let token = match split.last() {
        Some(x) => x,
        None => "",
    };

    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let valid_token = decode::<Claims>(
        token,
        jwt_secret.as_bytes(),
        &Validation::new(Algorithm::default()),
    );

    valid_token.is_ok()
}
