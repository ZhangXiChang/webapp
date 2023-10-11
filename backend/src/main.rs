use actix_files::*;
use actix_web::*;
use sqlx::postgres::*;

#[main]
async fn main() -> Result<(), sqlx::Error> {
    println!("user name:");
    let mut user = String::new();
    std::io::stdin().read_line(&mut user).unwrap();
    println!("password:");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    println!("database:");
    let mut database = String::new();
    std::io::stdin().read_line(&mut database).unwrap();
    println!("Starting the server...");

    let pool = PgPoolOptions::new()
        .connect(format!("postgres://{}:{}@localhost/{}", user, password, database).as_str())
        .await?;

    let httpserver = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_app_name)
            .service(get_app_logo)
            .service(Files::new("/", "./dist").index_file("index.html"))
    })
    .bind("0.0.0.0:80")?
    .run();
    println!("Server started successfully!");
    httpserver.await?;
    Ok(())
}

#[get("/api/get_app_name")]
async fn get_app_name(pooldata: web::Data<sqlx::Pool<Postgres>>) -> impl Responder {
    let result: (String,) = sqlx::query_as("SELECT name FROM app_name ORDER BY id DESC")
        .fetch_one(pooldata.as_ref())
        .await
        .unwrap();
    HttpResponse::Ok().body(result.0)
}
#[get("/api/get_app_logo")]
async fn get_app_logo(pooldata: web::Data<sqlx::Pool<Postgres>>) -> impl Responder {
    let result: (Vec<u8>,) = sqlx::query_as("SELECT image FROM app_logo ORDER BY id DESC")
        .fetch_one(pooldata.as_ref())
        .await
        .unwrap();
    HttpResponse::Ok().body(result.0)
}
