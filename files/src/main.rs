mod config;
mod routes;
mod models;
mod collections;



fn main() {
    let v = vec![1,2,3];


    println!("Printed from the main file!");
    config::foo();
    routes::health_routes::bar();
    routes::user_route::print_user_route();
    models::user_model::print_user_model(); 
    collections::vectors::third_element(&v);
    collections::hashmap::print_a_hashmap();
}
