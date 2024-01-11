use actix_web::{
    dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder, ResponseError,
};
use anyhow::Result;

use crate::{
    application_services::{EmpireData, MillenniumFalconData},
    domain_models::{GalaxyRoutes, PlanetCatalog},
    domain_services::compute_probability_of_success,
};

struct AppState {
    galaxy_routes: GalaxyRoutes,
    planet_catalog: PlanetCatalog,
    millennium_falcon_data: MillenniumFalconData,
}

/// Custom Error type that wrap anyhow::Error and implement actix_web::ResponseError
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an internal error occurred: {0}")]
    InternalError(#[from] anyhow::Error),
}

impl ResponseError for Error {}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/proba")]
async fn proba(data: web::Data<AppState>, req_body: String) -> std::result::Result<String, Error> {
    let empire_data = EmpireData::parse(&req_body)?;
    let hunter_planning = empire_data.to_bounty_hunters_planning(&data.planet_catalog);
    let proba = compute_probability_of_success(
        &hunter_planning,
        &data.galaxy_routes,
        &data.planet_catalog,
        data.millennium_falcon_data.autonomy,
        &data.millennium_falcon_data.departure,
        &data.millennium_falcon_data.arrival,
        empire_data.countdown,
    )? * 100.;
    Ok(format!("{proba}%"))
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../../front/index.html"))
}

pub fn run(
    address: &str,
    galaxy_routes: GalaxyRoutes,
    planet_catalog: PlanetCatalog,
    millennium_falcon_data: MillenniumFalconData,
) -> Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                galaxy_routes: galaxy_routes.clone(),
                planet_catalog: planet_catalog.clone(),
                millennium_falcon_data: millennium_falcon_data.clone(),
            }))
            .service(health_check)
            .service(proba)
            .service(index)
    })
    .bind(address)?
    .run();
    Ok(server)
}
