mod config;
mod routes;
mod models;



fn main() {
    println!("Printed from the main file!");
    config::foo();
    routes::health_routes::bar();
    routes::user_route::print_user_route();
    models::user_model::print_user_model(); 
}
