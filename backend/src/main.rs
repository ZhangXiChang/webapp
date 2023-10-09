use actix_files::*;
use actix_web::*;

#[main]
async fn main() -> Result<()> {
    let httpserver = HttpServer::new(|| {
        App::new()
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
async fn webtitle() -> impl Responder {
    HttpResponse::Ok().body("暴虐仙女的个人小站")
}
#[get("/webicon")]
async fn webicon() -> impl Responder {
    HttpResponse::Ok().body("")
}
