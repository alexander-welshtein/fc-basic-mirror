use std::env;

use actix_files::Files;
use actix_web::{App, guard, HttpServer, middleware, web};

use crate::db::create_pool;
use crate::schemas::root::create_schema_with_context;

mod db;
mod schemas;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let pool = create_pool().await;
    let schema = create_schema_with_context(pool);

    HttpServer::new(move ||
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(handlers::graphql))
            .service(Files::new("/", "static/dist/").index_file("index.html"))
    )
        .bind("localhost:3000")?
        .run()
        .await
}
