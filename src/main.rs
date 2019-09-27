mod db;
use db::user::NewUser;

fn main() {
    println!("Hello, world!");
    NewUser::new(String::from("something@somewhere.io"), "abadpass", String::from("username"));
}
