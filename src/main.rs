mod api;
mod model;
mod repository;

use api::task::{
    get_task, submit_task,
    start_task, 
    fail_task,
    complete_task,
    pause_task
    };

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use aws_config::meta::region::RegionProviderChain;
use repository::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Needed to add features = ["behavior-version-latest"] to stop warning
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;

    HttpServer::new(move || {
        let ddb_repo = DDBRepository::init(String::from("task"), config.clone());
        let ddb_data = Data::new(ddb_repo);
        let logger = Logger::default();
            App::new()
            .wrap(logger)
            .app_data(ddb_data.clone())
            .service(get_task)
            .service(submit_task)
            .service(start_task)
            .service(fail_task)
            .service(complete_task)
            .service(pause_task)
    }).bind(("127.0.0.1", 80))?
    .run()
    .await
}


