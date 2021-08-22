mod db;
mod schemas;

use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {



    HttpServer::new(||
        App::new()
            .service(Files::new("/", "static/dist/").index_file("index.html"))
    )
        .bind("localhost:3000")?
        .run()
        .await
}
