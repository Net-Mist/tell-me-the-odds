use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;

use crate::application_services::Route;

pub mod actix;
pub mod args;

#[derive(Debug)]
struct RouteDB {
    origin: Option<String>,
    destination: Option<String>,
    travel_time: Option<i64>,
}

impl TryFrom<RouteDB> for Route {
    type Error = anyhow::Error;

    fn try_from(value: RouteDB) -> Result<Self> {
        if value.origin.is_none() {
            return Err(anyhow!("origin can't be None"));
        }
        if value.destination.is_none() {
            return Err(anyhow!("destination can't be None"));
        }
        if value.travel_time.is_none() {
            return Err(anyhow!("travel_time can't be None"));
        }
        // now we can safely unwrap the data
        let origin = value.origin.unwrap();
        let destination = value.destination.unwrap();
        let travel_time = value.travel_time.unwrap();

        if travel_time < 1 {
            return Err(anyhow!("travel_time need to be >= 1"));
        }
        if origin.is_empty() {
            return Err(anyhow!("origin can't be empty"));
        }
        if destination.is_empty() {
            return Err(anyhow!("destination can't be empty"));
        }
        // now we can safely convert our data
        Ok(Route {
            origin,
            destination,
            travel_time: travel_time as u64,
        })
    }
}

pub async fn get_routes_from_db(db_path: &Path) -> Result<Vec<Route>> {
    let db_path = db_path
        .to_path_buf()
        .into_os_string()
        .into_string()
        .map_err(|e| anyhow!("{e:?}"))
        .context("Unable to convert path to string")?;
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_path)
        .await
        .context(format!("Unable to connect the the database at {db_path}"))?;

    let routes: Vec<Route> = sqlx::query_as!(RouteDB, "SELECT * FROM ROUTES")
        .fetch_all(&pool)
        .await?
        .into_iter()
        .filter_map(
            |d| match d.try_into().context("Issue reading route in the database") {
                Ok(v) => Some(v),
                Err(e) => {
                    println!("{e:#?}");
                    None
                }
            },
        )
        .collect();

    Ok(routes)
}
