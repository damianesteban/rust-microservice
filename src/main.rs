#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

extern crate serde;
extern crate serde_json;

#[macro_use] 
extern crate serde_derive;

extern crate actix_web;
extern crate actix_rt;

use actix_web::{HttpServer, App, web };
use std::io;

mod schema;
mod db_connection;
mod models;
mod handlers;
mod errors;
mod validation;

fn main() -> io::Result<()> {

    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,user-service=debug,actix_rt=info");
    pretty_env_logger::init();

    let sys = actix_rt::System::new("user-service");  // <- create Actix runtime

    HttpServer::new(
     move || App::new()
            .data(db_connection::establish_connection())
            .service(web::resource("/api/register")
              .route(web::get().to(handlers::user_list_handler))
              .route(web::post().to(handlers::register))
            )
            .service(web::resource("/api/login")
              .route(web::post().to(handlers::login))
            )
    )
    .bind("127.0.0.1:5000").unwrap()
    .start();

    info!("system is starting......");
    info!("such information");
    warn!("o_O");
    error!("much error");
    println!("started up...");
    sys.run()
}