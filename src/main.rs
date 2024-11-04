use actix::prelude::*;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use rusqlite::{params, Connection, Result};
use serde::Serialize;
use std::time::{Duration, Instant};

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
    let mut stmt = conn.prepare("SELECT id, tempc, tempf, humi, date, time FROM sensor ORDER BY id DESC LIMIT 1")?;
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

struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            // Here you would send a message to the client.
            // For example, you could query the database and send updated sensor data.
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn ws_index(r: web::Payload, stream: web::Query<String>) -> impl Responder {
    ws::start(MyWebSocket {}, &stream, r)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/tempc", web::get().to(tempc))
            .route("/tempf", web::get().to(tempf))
            .route("/humi", web::get().to(humi))
            .route("/ws/", web::get().to(ws_index))
    })
    .bind("10.0.4.60:8080")?
    .run()
    .await
}