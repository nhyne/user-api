extern crate rand;
extern crate scrypt;

use rand::prelude::*;
use scrypt::{ScryptParams, scrypt_simple};

pub struct NewUser {
    pub email: String,
    pub hashed_password: String,
    pub username: String,
    salt: String,
}

impl NewUser {
    pub fn new(email: String, raw_password: &str, username: String) -> NewUser {
        let salt : String = rand::thread_rng()
            .gen_ascii_chars()
            .take(10)
            .collect();
        println!("salt: {}", salt);

        let params = ScryptParams::new(15, 8, 1).unwrap();
        let hashed_password = scrypt_simple(raw_password, &params)
            .expect("OS RNG should not fail");

        println!("hashed pass: {}", hashed_password);
        NewUser {
            email,
            username,
            salt,
            hashed_password,
        }
    }
}
