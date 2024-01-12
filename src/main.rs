use actix_web::dev::Server;
use anyhow::Result;
use core::panic;
use millennium_falcon::{
    application_services::{into_galaxy_routes_and_planet_id, MillenniumFalconData},
    infrastructure_services::{actix::run, args::parse_webserver, db::get_routes_from_db},
};

pub async fn setup_webserver(address: &str) -> Result<Server> {
    let millennium_falcon_data_path = parse_webserver()?;
    let millennium_falcon_data = MillenniumFalconData::read(&millennium_falcon_data_path)?;
    let routes = get_routes_from_db(&millennium_falcon_data.routes_db).await?;
    let (galaxy_routes, planet_ids) = into_galaxy_routes_and_planet_id(routes);
    run(address, galaxy_routes, planet_ids, millennium_falcon_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file_appender = tracing_appender::rolling::daily("logs", "millennium.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();

    match setup_webserver("0.0.0.0:8000").await {
        Ok(server) => server.await,
        Err(e) => {
            let e = e.context("unable to start the server");
            println!("{e:#?}");
            panic!();
        }
    }
}
