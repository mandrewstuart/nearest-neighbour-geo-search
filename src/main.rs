use actix_web::{web, App, HttpServer};
mod routes;
use std::collections::HashMap;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = &(std::env::args().collect::<Vec<String>>());
    let filename = args[1].to_string();
    let port = args[2].parse::<u16>().unwrap();
    let vec_size = args[3].parse::<usize>().unwrap();
    println!("Getting data from {}", filename);
    let data = routes::get_data::get_data(filename, vec_size);
    let data = routes::get_data::kmeans(data);
    let data: web::Data<HashMap<String, routes::get_data::SearchData>> = web::Data::new(data);
    println!("Data loaded, server started on {}", port);
    HttpServer::new(move ||  App::new()
        .app_data(data.clone())
        .service(routes::search)
        .service(routes::search_ann)
    )
    .bind(("127.0.0.1", port)).unwrap()
    .run()
    .await
}
