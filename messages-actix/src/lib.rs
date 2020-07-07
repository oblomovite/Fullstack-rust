#[macro_use]
extern crate actix_web;

// use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
// use serde::Serialize;

use actix_web::{middleware, web, App, HttpServer, Result};
use serde::Serialize;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};


pub struct MessageApp {
    port: u16,
}

impl MessageApp {

    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
            .bind(("127.0.0.1", self.port))?
            .workers(8)
            .run()
    }

}

// #[derive(Serialize)]
// struct IndexResponse {
//     message: String,
// }
// 
// #[get("/")]
// fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
//     let hello = req
//         .headers()
//         .get("hello")
//         .and_then(|v| v.to_str().ok())
//         .unwrap_or_else(|| "world");
// 
//     Ok(web::Json(IndexResponse {
//         message: hello.to_owned(),
//     }))
// }


static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);


#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: size, 
    messages: Vec<String>,
}

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: ms.clone(),
    }))
}

