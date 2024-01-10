use anyhow::Result;
use millennium_falcon::application_services::into_galaxy_routes_and_planet_id;
use millennium_falcon::application_services::EmpireData;
use millennium_falcon::application_services::MillenniumFalconData;
use millennium_falcon::domain_services::compute_probability_of_success;
use millennium_falcon::infrastructure_service::get_routes_from_db;
use millennium_falcon::infrastructure_service::parse_cli;

#[tokio::main]
async fn main() -> Result<()> {
    let (millennium_falcon_data_path, empire_data_path) = parse_cli()?;
    let millennium_falcon_data = MillenniumFalconData::read(&millennium_falcon_data_path)?;
    let empire_data = EmpireData::read(&empire_data_path)?;
    let routes = get_routes_from_db(&millennium_falcon_data.routes_db).await?;
    let (galaxy_routes, planet_ids) = into_galaxy_routes_and_planet_id(routes);
    let hunter_planning = empire_data.to_bounty_hunters_planning(&planet_ids);
    let proba = compute_probability_of_success(
        &hunter_planning,
        &galaxy_routes,
        &planet_ids,
        millennium_falcon_data.autonomy,
        &millennium_falcon_data.departure,
        &millennium_falcon_data.arrival,
        empire_data.countdown,
    )? * 100.;
    println!("{proba}");
    Ok(())
}
