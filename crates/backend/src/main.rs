use actix_files::*;
use actix_web::*;

#[main]
async fn main() -> Result<()> {
    println!("Starting the server...");
    let httpserver =
        HttpServer::new(|| App::new().service(Files::new("/", "./dist").index_file("index.html")))
            .bind("0.0.0.0:80")?
            .run();
    println!("Server started successfully!");
    httpserver.await?;
    Ok(())
}
