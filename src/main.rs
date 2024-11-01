use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::{params, Connection, Result};
use serde::Serialize;

#[derive(Serialize)]
struct SensorData {
    id: i64,
    tempc: String,
    tempf: String,
    humi: String,
    date: String,
    time: String,
}

async fn get_last_entry() -> Result<SensorData> {
    let conn = Connection::open("/usr/share/dht11rs/dht11rs/sensor_data.db")?;
    let mut stmt = conn.prepare("SELECT idx, tempc, tempf, humi, date, time FROM sensor ORDER BY idx DESC LIMIT 1")?;
    let sensor_data = stmt.query_row(params![], |row| {
        Ok(SensorData {
            id: row.get(0)?,
            tempc: row.get(1)?,
            tempf: row.get(2)?,
            humi: row.get(3)?,
            date: row.get(4)?,
            time: row.get(4)?,
        })
    })?;
    Ok(sensor_data)
}

async fn tempc() -> impl Responder {
    match get_last_entry().await {
        Ok(sensor_data) => HttpResponse::Ok().json(sensor_data.tempc),
        Err(e) => {
            eprintln!("Error querying the database: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn tempf() -> impl Responder {
    match get_last_entry().await {
        Ok(sensor_data) => HttpResponse::Ok().json(sensor_data.tempf),
        Err(e) => {
            eprintln!("Error querying the database: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn humi() -> impl Responder {
    match get_last_entry().await {
        Ok(sensor_data) => HttpResponse::Ok().json(sensor_data.humi),
        Err(e) => {
            eprintln!("Error querying the database: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/tempc", web::get().to(tempc))
            .route("/tempf", web::get().to(tempf))
            .route("/humi", web::get().to(humi))
    })
    .bind("10.0.4.60:8080")?
    .run()
    .await
}
