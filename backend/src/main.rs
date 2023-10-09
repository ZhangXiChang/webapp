use actix_files::*;
use actix_web::*;
use sqlx::postgres::*;

#[main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect("postgres://beigua:123456@localhost/webapp")
        .await?;

    let pooldata = web::Data::new(pool);
    let httpserver = HttpServer::new(move || {
        App::new()
            .app_data(pooldata.clone())
            .service(webtitle)
            .service(webicon)
            .service(Files::new("/", "./dist").index_file("index.html"))
    })
    .bind("0.0.0.0:80")?
    .run();
    println!("Web application link: http://127.0.0.1");
    httpserver.await?;
    Ok(())
}

#[get("/webtitle")]
async fn webtitle(pooldata: web::Data<sqlx::Pool<Postgres>>) -> impl Responder {
    let webtitle: (String,) =
        sqlx::query_as("SELECT app_name FROM app_name_history ORDER BY id DESC")
            .fetch_one(pooldata.as_ref())
            .await
            .unwrap();
    HttpResponse::Ok().body(webtitle.0)
}
#[get("/webicon")]
async fn webicon() -> impl Responder {
    HttpResponse::Ok().body("")
}
