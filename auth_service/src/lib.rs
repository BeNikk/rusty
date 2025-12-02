#![allow(dead_code, unused_variables)]

pub mod database;
mod auth_utils;

pub use auth_utils::models::Credentials;
use database::Status;
use rand::prelude::*;

pub fn authenticate(creds: Credentials) {
    let timeout = rand::thread_rng().gen_range(100..500);
    println!("The timeout is {}",timeout);
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}