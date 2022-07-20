mod config;
mod routes;
mod models;
mod collections;
mod error_handling;

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};




fn main() {
    let v = vec![1,2,3,4,5];


   
    println!("Printed from the main file!");
    config::foo();
    routes::health_routes::bar();
    routes::user_route::print_user_route();
    models::user_model::print_user_model(); 
    collections::vectors::third_element(&v);
    collections::hashmap::print_a_hashmap();




}
