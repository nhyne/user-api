extern crate rand;
extern crate scrypt;

use rand::prelude::*;
use scrypt::{ScryptParams, scrypt_simple, scrypt_check};
use crate::db::schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
    salt: String,
}

impl NewUser {
    pub fn new(email: String, raw_password: String, username: String) -> NewUser {
        let salt : String = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(50)
            .collect();

        let params = ScryptParams::new(15, 8, 1).unwrap();
        let salt_combination = format!("{}{}", salt, raw_password);
        let password = scrypt_simple(&salt_combination, &params)
            .expect("OS RNG should not fail");

        NewUser {
            email,
            username,
            salt,
            password,
        }
    }
}
