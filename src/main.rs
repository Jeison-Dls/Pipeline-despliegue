use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use std::fs;

mod routes;
mod models;
mod lib;





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    verificar_archivo_credenciales();

    // Cargar base de datos
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no configurada");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Error al conectar a la base de datos");



    // Cargar ruta de credenciales de Google
    let google_credentials = env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .expect("GOOGLE_APPLICATION_CREDENTIALS no configurada");

    println!("Firebase autenticación configurada correctamente.");
    println!("Servidor corriendo en http://0.0.0.0:8081");

    // Crear servidor HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(google_credentials.clone())) // Compartir credenciales
            .configure(routes::config_routes)

    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}

fn verificar_archivo_credenciales() {
    let cred_path = env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .expect("GOOGLE_APPLICATION_CREDENTIALS no configurada");

    match fs::File::open(&cred_path) {
        Ok(_) => println!("Archivo de credenciales accesible."),
        Err(err) => println!("No se pudo abrir el archivo de credenciales: {}", err),
    }

    if fs::metadata(&cred_path).is_ok() {
        println!("Archivo de credenciales encontrado en: {}", cred_path);
    } else {
        println!("Archivo de credenciales NO encontrado en: {}", cred_path);
    }
}



