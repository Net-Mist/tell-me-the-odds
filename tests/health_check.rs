use anyhow::Result;
use millennium_falcon::{
    application_services::{into_galaxy_routes_and_planet_id, MillenniumFalconData},
    infrastructure_services::{actix::run, get_routes_from_db},
};

#[tokio::test]
async fn test_health_check() {
    spawn_app().await.unwrap();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute the request");

    println!("{response:?}");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

async fn spawn_app() -> Result<()> {
    let millennium_falcon_data_path = "examples/millennium-falcon.json";
    let millennium_falcon_data = MillenniumFalconData::read(millennium_falcon_data_path)?;
    let routes = get_routes_from_db(&millennium_falcon_data.routes_db).await?;
    let (galaxy_routes, planet_ids) = into_galaxy_routes_and_planet_id(routes);
    let server = run(
        "127.0.0.1:8080",
        galaxy_routes,
        planet_ids,
        millennium_falcon_data,
    )?;

    tokio::spawn(server);
    Ok(())
}
