extern crate jsonwebtoken as jwt;
extern crate rand;
extern crate regex;
extern crate scrypt;

use crate::db::schema::users;
use crate::responses::Error as ResponseError;
use diesel::result::Error;
use dotenv::dotenv;
use jwt::{decode, encode, Algorithm, Header, Validation};
use rand::prelude::*;
use regex::Regex;
use scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use std::env;
use uuid::Uuid;

use chrono::Utc;
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: Uuid,
    services: Vec<String>,
    // expiration time of the token
    exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RocketNewUser {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct RocketLogin {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
    salt: String,
}

impl NewUser {
    // TODO: Should return a result here based on if encryption works or not
    // TODO: Need to validate that we have a valid email
    pub fn new(
        email: String,
        raw_password: String,
        username: String,
    ) -> Result<NewUser, ResponseError> {
        let valid_email = NewUser::validate_email(&email);
        match valid_email {
            Ok(_) => {
                let salt: String = rand::thread_rng()
                    .sample_iter(rand::distributions::Alphanumeric)
                    .take(50)
                    .collect();

                // The number for Scrypt came from the docs on the package website
                let params = ScryptParams::new(15, 8, 1).unwrap();
                let salt_combination = format!("{}{}", salt, raw_password);
                let password =
                    scrypt_simple(&salt_combination, &params).expect("OS RNG should not fail");

                Ok(NewUser {
                    email,
                    username,
                    salt,
                    password,
                })
            }
            Err(e) => Err(e),
        }
    }

    fn validate_email(email: &String) -> Result<bool, ResponseError> {
        let connection = crate::db::establish_connection();
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();

        if email_regex.is_match(email) {
            let selected_user_vec: Result<User, Error> = crate::db::schema::users::table
                .filter(users::email.eq(email))
                .first(&connection);
            match selected_user_vec {
                Ok(_) => {
                    return Err(ResponseError {
                        message: String::from("Email already in use"),
                    })
                }
                // TODO: There are plenty of other errors that could happen besides not found, these should be accounted for
                Err(_) => return Ok(true),
            }
        } else {
            Err(ResponseError {
                message: String::from("Invalid email address"),
            })
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    salt: String,
    pub password: String,
}

impl User {
    pub fn login_and_receive_jwt(&self, raw_password_attempt: &String) -> Token {
        let login_attempt = self.validate_password(raw_password_attempt);
        if login_attempt {
            self.generate_jwt()
        } else {
            Token {
                token: String::from(""),
            }
        }
    }

    pub fn validate_password(&self, raw_password_attempt: &String) -> bool {
        let raw_password_val = format!("{}{}", self.salt, raw_password_attempt);
        match scrypt_check(&raw_password_val, &self.password) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn generate_jwt(&self) -> Token {
        let user_claims = Claims {
            user_id: self.id,
            services: vec![String::from("archiver")],
            exp: Utc::now().timestamp() + 1, //_800,
        };
        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let token = encode(&Header::default(), &user_claims, &jwt_secret.as_bytes()).unwrap();
        Token { token }
    }
}
