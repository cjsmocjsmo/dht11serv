use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::{params, Connection, Result};
use serde::Serialize;
use chrono::{DateTime, Local};
use local_ip_address::local_ip;

#[derive(Serialize)]
struct SensorData {
    id: i64,
    tempc: String,
    tempf: String,
    tempo: String,
    humi: String,
    date: String,
    time: String,
    timestamp: String,
}

async fn get_last_entry() -> Result<SensorData> {
    let conn = Connection::open("/usr/share/dht11rs/sensor_data.db")?;
    let mut stmt = conn.prepare("SELECT * FROM sensor ORDER BY id DESC LIMIT 1")?;
    let sensor_data = stmt.query_row(params![], |row| {
        Ok(SensorData {
            id: row.get(0)?,
            tempc: row.get(1)?,
            tempf: row.get(2)?,
            tempo: row.get(3)?,
            humi: row.get(4)?,
            date: row.get(5)?,
            time: row.get(6)?,
            timestamp: row.get(7)?,
        })
    })?;
    Ok(sensor_data)
}

fn get_current_date() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y-%m-%d").to_string()
}

fn get_yesterdays_date() -> String {
    let local: DateTime<Local> = Local::now();
    let yesterdays_date = local - chrono::Duration::days(1);
    yesterdays_date.format("%Y-%m-%d").to_string()
}

async fn todays_data() -> Result<Vec<SensorData>> {
    let date = get_current_date();
    let conn = Connection::open("/usr/share/dht11rs/sensor_data.db")?;
    let stmstr = format!(
        "SELECT * FROM sensorhour WHERE date = '{}'",
        date
    );
    let mut stmt = conn.prepare(&stmstr)?;
    let sensor_data_iter = stmt.query_map(params![], |row| {
        Ok(SensorData {
            id: row.get(0)?,
            tempc: row.get(1)?,
            tempf: row.get(2)?,
            tempo: row.get(3)?,
            humi: row.get(4)?,
            date: row.get(5)?,
            time: row.get(6)?,
            timestamp: row.get(7)?,
        })
    })?;
    
    let sensor_data_vec: Vec<SensorData> = sensor_data_iter.filter_map(Result::ok).collect();
    Ok(sensor_data_vec)
}

async fn yesterdays_data() -> Result<Vec<SensorData>> {
    let date = get_yesterdays_date();
    let conn = Connection::open("/usr/share/dht11rs/sensor_data.db")?;
    let stmstr = format!(
        "SELECT * FROM sensorhour WHERE date = '{}'",
        date
    );
    let mut stmt = conn.prepare(&stmstr)?;
    let sensor_data_iter = stmt.query_map(params![], |row| {
        Ok(SensorData {
            id: row.get(0)?,
            tempc: row.get(1)?,
            tempf: row.get(2)?,
            tempo: row.get(3)?,
            humi: row.get(4)?,
            date: row.get(5)?,
            time: row.get(6)?,
            timestamp: row.get(7)?,
        })
    })?;
    
    let sensor_data_vec: Vec<SensorData> = sensor_data_iter.filter_map(Result::ok).collect();
    Ok(sensor_data_vec)
}

async fn get_todays_tempf() -> impl Responder {
    match todays_data().await {
        Ok(sensor_data) => {
            let tempf_vec: Vec<String> = sensor_data.iter().map(|x| x.tempf.clone()).collect();
            HttpResponse::Ok().json(tempf_vec)
        }
        Err(e) => {
            eprintln!("Error querying the database: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn get_yesterdays_tempf() -> impl Responder {
    match yesterdays_data().await {
        Ok(sensor_data) => {
            let tempf_vec: Vec<String> = sensor_data.iter().map(|x| x.tempf.clone()).collect();
            HttpResponse::Ok().json(tempf_vec)
        }
        Err(e) => {
            eprintln!("Error querying the database: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn get_todays_tempc() -> impl Responder {
    match todays_data().await {
        Ok(sensor_data) => {
            let tempc_vec: Vec<String> = sensor_data.iter().map(|x| x.tempc.clone()).collect();
            HttpResponse::Ok().json(tempc_vec)
        }
        Err(e) => {
            eprintln!("Error querying the database: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn get_yesterdays_tempc() -> impl Responder {
    match yesterdays_data().await {
        Ok(sensor_data) => {
            let tempc_vec: Vec<String> = sensor_data.iter().map(|x| x.tempc.clone()).collect();
            HttpResponse::Ok().json(tempc_vec)
        }
        Err(e) => {
            eprintln!("Error querying the database: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn get_todays_humi() -> impl Responder {
    match todays_data().await {
        Ok(sensor_data) => {
            let humi_vec: Vec<String> = sensor_data.iter().map(|x| x.humi.clone()).collect();
            HttpResponse::Ok().json(humi_vec)
        }
        Err(e) => {
            eprintln!("Error querying the database: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn get_yesterdays_humi() -> impl Responder {
    match yesterdays_data().await {
        Ok(sensor_data) => {
            let humi_vec: Vec<String> = sensor_data.iter().map(|x| x.humi.clone()).collect();
            HttpResponse::Ok().json(humi_vec)
        }
        Err(e) => {
            eprintln!("Error querying the database: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
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

async fn tempo() -> impl Responder {
    match get_last_entry().await {
        Ok(sensor_data) => HttpResponse::Ok().json(sensor_data.tempo),
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

async fn time_stamp() -> impl Responder {
    match get_last_entry().await {
        Ok(sensor_data) => HttpResponse::Ok().json(sensor_data.timestamp),
        Err(e) => {
            eprintln!("Error querying the database: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let localip = local_ip().unwrap();
    let ipaddr = format!("{}:8080", localip);
    println!("{}", ipaddr);

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600))
            .route("/tempc", web::get().to(tempc))
            .route("/tempf", web::get().to(tempf))
            .route("/tempo", web::get().to(tempo))
            .route("/humi", web::get().to(humi))
            .route("/timestamp", web::get().to(time_stamp))
            .route("/todays_tempf", web::get().to(get_todays_tempf))
            .route("/todays_humi", web::get().to(get_todays_humi))
            .route("/todays_tempc", web::get().to(get_todays_tempc))
            .route("/yesterdays_tempf", web::get().to(get_yesterdays_tempf))
            .route("/yesterdays_humi", web::get().to(get_yesterdays_humi))
            .route("/yesterdays_tempc", web::get().to(get_yesterdays_tempc))
        })
    .bind(ipaddr)?
    // .bind("10.0.4.112:8080")?
    // .bind("10.0.4.72:8080")?
    .run()
    .await
}
