use std::{collections::HashMap, fs::read_to_string};

use anyhow::Result;
use millennium_falcon::{
    application_services::{into_galaxy_routes_and_planet_id, MillenniumFalconData},
    infrastructure_services::{actix::run, db::get_routes_from_db},
};

#[tokio::test]
async fn test_health_check() {
    spawn_app("127.0.0.1:8080").await.unwrap();

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

#[tokio::test]
async fn test_proba_endpoint() {
    spawn_app("127.0.0.1:8081").await.unwrap();

    let client = reqwest::Client::new();

    let responses = [(1, "0%"), (2, "81%"), (3, "90%"), (4, "100%")]
        .into_iter()
        .collect::<HashMap<_, _>>();

    for example_id in 1..5 {
        let response = client
            .post("http://127.0.0.1:8081/proba")
            .body(read_to_string(format!("examples/example{example_id}/empire.json")).unwrap())
            .send()
            .await
            .expect("Failed to execute the request");

        println!("{response:?}");

        assert!(response.status().is_success());
        let text = response.text().await.unwrap();
        assert_eq!(text, responses.get(&example_id).unwrap().to_string());
    }
}

#[tokio::test]
async fn test_index() {
    spawn_app("127.0.0.1:8082").await.unwrap();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8082/")
        .send()
        .await
        .expect("Failed to execute the request");

    println!("{response:?}");

    assert!(response.status().is_success());
    let text = response.text().await.unwrap();
    assert_eq!(text, read_to_string("front/index.html").unwrap());
}

async fn spawn_app(address: &str) -> Result<()> {
    let millennium_falcon_data_path = "examples/millennium-falcon.json";
    let millennium_falcon_data = MillenniumFalconData::read(millennium_falcon_data_path)?;
    let routes = get_routes_from_db(&millennium_falcon_data.routes_db).await?;
    let (galaxy_routes, planet_ids) = into_galaxy_routes_and_planet_id(routes);
    let server = run(address, galaxy_routes, planet_ids, millennium_falcon_data)?;

    tokio::spawn(server);
    Ok(())
}
