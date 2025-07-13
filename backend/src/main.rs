use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use actix_web::http::header;
use dotenv::dotenv;

mod models;
mod auth;
mod controllers;
mod database;

use controllers::{login, register, get_current_user, protected_route};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    println!("Starting Student Management API server...");
    println!("Server running at http://localhost:3001");
    
    // Establish database connection
    let db = database::establish_connection()
        .await
        .expect("Failed to connect to database");

    let db = web::Data::new(db);

    HttpServer::new(move || {
        // Configure CORS for each app instance
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(db.clone())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(login))
                            .route("/register", web::post().to(register))
                            .route("/me", web::get().to(get_current_user))
                    )
                    .service(
                        web::scope("/protected")
                            .route("/test", web::get().to(protected_route))
                    )
            )
    })
    .bind("127.0.0.1:3001")?
    .run()
    .await
}
