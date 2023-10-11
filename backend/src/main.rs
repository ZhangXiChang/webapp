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

    //获取tls
    let tls = get_tls(info.tlsinfo.cert_file_path, info.tlsinfo.key_file_path);

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
struct TlsInfo {
    cert_file_path: String,
    key_file_path: String,
}
struct Information {
    databaseinfo: DatabaseInfo,
    tlsinfo: TlsInfo,
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
    println!("cert file path:");
    let mut cert_file_path = String::new();
    std::io::stdin().read_line(&mut cert_file_path).unwrap();
    println!("key file path:");
    let mut key_file_path = String::new();
    std::io::stdin().read_line(&mut key_file_path).unwrap();

    let databaseinfo = DatabaseInfo {
        address,
        username,
        password,
        database,
    };
    let tlsinfo = TlsInfo {
        cert_file_path,
        key_file_path,
    };
    Information {
        databaseinfo,
        tlsinfo,
    }
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

fn get_tls(cert_file_path: String, key_file_path: String) -> ServerConfig {
    let tlsconfig = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file = &mut io::BufReader::new(fs::File::open(cert_file_path).unwrap());
    let key_file = &mut io::BufReader::new(fs::File::open(key_file_path).unwrap());

    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
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
