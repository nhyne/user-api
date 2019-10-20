extern crate jsonwebtoken as jwt;
extern crate rand;
extern crate scrypt;

use crate::db::schema::users;
use jwt::{decode, encode, Algorithm, Header, Validation};
use rand::prelude::*;
use scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    userId: Uuid,
    services: Vec<String>,
    // expiration time of the token
    exp: SystemTime,
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
    pub fn new(email: String, raw_password: String, username: String) -> NewUser {
        let salt: String = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(50)
            .collect();

        // The number for Scrypt came from the docs on the package website
        let params = ScryptParams::new(15, 8, 1).unwrap();
        let salt_combination = format!("{}{}", salt, raw_password);
        let password = scrypt_simple(&salt_combination, &params).expect("OS RNG should not fail");

        NewUser {
            email,
            username,
            salt,
            password,
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
    pub fn login_and_receive_jwt(&self, raw_password_attempt: &String) -> String {
        let login_attempt = self.validate_password(raw_password_attempt);
        if login_attempt {
            self.generate_jwt()
        } else {
            String::from("")
        }
    }

    pub fn validate_password(&self, raw_password_attempt: &String) -> bool {
        let raw_password_val = format!("{}{}", self.salt, raw_password_attempt);
        match scrypt_check(&raw_password_val, &self.password) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn generate_jwt(&self) -> String {
        let token_expiration = SystemTime::now() + Duration::new(1_800, 0);
        let user_claims = Claims {
            userId: self.id,
            services: vec![String::from("archiver")],
            exp: token_expiration,
        };
        let token = encode(&Header::default(), &user_claims, "secret".as_ref()).unwrap();
        token
    }
}
