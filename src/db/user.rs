extern crate rand;
extern crate scrypt;

use crate::db::schema::users;
use rand::prelude::*;
use scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use uuid::Uuid;

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

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub username: String,
    salt: String,
}

impl User {
    pub fn validate_password(&self, raw_password_attempt: String) -> bool {
        let raw_password_val = format!("{}{}", self.salt, raw_password_attempt);
        match scrypt_check(&raw_password_val, &self.password) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
