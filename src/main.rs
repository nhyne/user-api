#![feature(proc_macro_hygiene)]
// Rocket web server
#![feature(decl_macro)]
mod db;
use db::user::NewUser;

// Diesel ORM
#[macro_use]
extern crate diesel;

fn main() {
    NewUser::new(String::from("something@somewhere.io"), String::from("abadpass"), String::from("username"));
}
