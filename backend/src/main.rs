use actix_files::*;
use actix_web::*;
use rustls::*;
use rustls_pemfile::*;
use sqlx::postgres::*;
use std::*;

#[main]
async fn main() -> Result<(), sqlx::Error> {
    //收集信息
    let info = information_gathering();

    println!("Starting the server...");

    //连接数据库
    let pool = connect_database(
        info.databaseinfo.address,
        info.databaseinfo.username,
        info.databaseinfo.password,
        info.databaseinfo.database,
    )
    .await?;

    //创建tls
    let result: (String, String) =
        sqlx::query_as("SELECT certs,key FROM certificates ORDER BY id DESC")
            .fetch_one(&pool)
            .await
            .unwrap();
    let tls = create_tls(result.0, result.1);

    //创建HTTPS服务器
    let httpserver = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_app_name)
            .service(get_app_logo)
            .service(Files::new("/", "./dist").index_file("index.html"))
    })
    .bind_rustls_021("0.0.0.0:443", tls)?
    .run();
    println!("Server started successfully!");
    httpserver.await?;
    Ok(())
}

struct DatabaseInfo {
    address: String,
    username: String,
    password: String,
    database: String,
}
struct Information {
    databaseinfo: DatabaseInfo,
}
fn information_gathering() -> Information {
    println!("IP address:");
    let mut address = String::new();
    std::io::stdin().read_line(&mut address).unwrap();
    println!("user name:");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    println!("password:");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    println!("database:");
    let mut database = String::new();
    std::io::stdin().read_line(&mut database).unwrap();

    let databaseinfo = DatabaseInfo {
        address,
        username,
        password,
        database,
    };
    Information { databaseinfo }
}

async fn connect_database(
    address: String,
    username: String,
    password: String,
    database: String,
) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .connect(
            format!(
                "postgres://{}:{}@{}/{}",
                username, password, address, database
            )
            .as_str(),
        )
        .await
}

fn create_tls(cert_str: String, key_str: String) -> ServerConfig {
    let tlsconfig = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_chain = certs(&mut io::BufReader::new(io::Cursor::new(cert_str)))
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> =
        rsa_private_keys(&mut io::BufReader::new(io::Cursor::new(key_str)))
            .unwrap()
            .into_iter()
            .map(PrivateKey)
            .collect();
    if keys.is_empty() {
        eprintln!("The RSA private key cannot be found.");
        std::process::exit(1);
    }

    tlsconfig
        .with_single_cert(cert_chain, keys.remove(0))
        .unwrap()
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
