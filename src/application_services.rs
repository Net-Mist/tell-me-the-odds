use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::domain_model::{BountyHunterPlanning, GalaxyRoutes, PlanetCatalog};

#[derive(Debug, Deserialize)]
pub struct MillenniumFalconData {
    pub autonomy: u64,
    pub departure: String,
    pub arrival: String,
    pub routes_db: String,
}

impl MillenniumFalconData {
    pub fn read(path: &str) -> Result<Self> {
        let content = &fs::read_to_string(path).context("Unable to read millennium data file")?;
        MillenniumFalconData::parse(content)
    }

    pub fn parse(text: &str) -> Result<Self> {
        serde_json::from_str(text).context("Unable to parse millennium falcon data")
    }
}

#[derive(Debug, Deserialize)]
pub struct EmpireData {
    pub countdown: u64,
    pub bounty_hunters: Vec<BountyHunter>,
}

impl EmpireData {
    pub fn read(path: &str) -> Result<Self> {
        let content = &fs::read_to_string(path).context("Unable to read empire data file")?;
        EmpireData::parse(content)
    }

    pub fn parse(text: &str) -> Result<Self> {
        serde_json::from_str(text).context("Unable to parse empire data")
    }

    pub fn to_bounty_hunters_planning(
        &self,
        planet_id_map: &PlanetCatalog,
    ) -> BountyHunterPlanning {
        let mut planet_to_days = HashMap::new();
        for hunter in self.bounty_hunters.iter() {
            let planet_id = match planet_id_map.get(&hunter.planet) {
                Some(v) => v,
                None => {
                    println!("Hunter outside of map");
                    continue;
                }
            };
            planet_to_days
                .entry(*planet_id)
                .or_insert(HashSet::new())
                .insert(hunter.day);
        }
        BountyHunterPlanning::new(planet_to_days)
    }
}

#[derive(Debug, Deserialize)]
pub struct BountyHunter {
    pub planet: String,
    pub day: u64,
}

#[derive(Debug)]
pub struct Route {
    pub origin: String,
    pub destination: String,
    pub travel_time: u64,
}

pub fn into_galaxy_routes_and_planet_id(routes: Vec<Route>) -> (GalaxyRoutes, PlanetCatalog) {
    let mut galaxy_routes = GalaxyRoutes::new();
    let mut plannet_id_map = PlanetCatalog::new();

    for route in routes {
        let origin_id = plannet_id_map.get_or_insert(route.origin);
        let destination_id = plannet_id_map.get_or_insert(route.destination);
        galaxy_routes.add_route(origin_id, destination_id, route.travel_time);
    }

    (galaxy_routes, plannet_id_map)
}

#[cfg(test)]
mod test {

    use crate::{
        application_services::BountyHunter,
        domain_model::{BountyHunterPlanning, GalaxyRoutes, PlanetCatalog},
    };

    use super::{into_galaxy_routes_and_planet_id, EmpireData, Route};

    #[test]
    fn test_to_bounty_hunters_planning() {
        let empire_data = EmpireData {
            countdown: 7,
            bounty_hunters: vec![
                BountyHunter {
                    planet: "Hoth".to_string(),
                    day: 6,
                },
                BountyHunter {
                    planet: "Hoth".to_string(),
                    day: 7,
                },
                BountyHunter {
                    planet: "Hoth".to_string(),
                    day: 8,
                },
            ],
        };
        let planet_id_map = get_planet_id_map();
        let hoth_id = *planet_id_map.get("Hoth").unwrap();
        let bh_planning = empire_data.to_bounty_hunters_planning(&planet_id_map);
        let bh_planning_gt = BountyHunterPlanning::new(
            [(hoth_id, [6, 7, 8].into_iter().collect())]
                .into_iter()
                .collect(),
        );
        assert_eq!(bh_planning, bh_planning_gt);
    }

    #[test]
    fn test_into_galaxy_routes_and_planet_id() {
        let routes = vec![
            Route {
                origin: "Tatooine".to_string(),
                destination: "Dagobah".to_string(),
                travel_time: 6,
            },
            Route {
                origin: "Dagobah".to_string(),
                destination: "Endor".to_string(),
                travel_time: 4,
            },
            Route {
                origin: "Dagobah".to_string(),
                destination: "Hoth".to_string(),
                travel_time: 1,
            },
            Route {
                origin: "Hoth".to_string(),
                destination: "Endor".to_string(),
                travel_time: 1,
            },
            Route {
                origin: "Tatooine".to_string(),
                destination: "Hoth".to_string(),
                travel_time: 6,
            },
        ];
        let (galaxy_route, planet_id_map) = into_galaxy_routes_and_planet_id(routes);
        let planet_id_map_gt = get_planet_id_map();

        let tatooine_id = *planet_id_map.get("Tatooine").unwrap();
        let dagobah_id = *planet_id_map.get("Dagobah").unwrap();
        let endor_id = *planet_id_map.get("Endor").unwrap();
        let hoth_id = *planet_id_map.get("Hoth").unwrap();

        assert_eq!(planet_id_map, planet_id_map_gt);

        let galaxy_route_gt = GalaxyRoutes::from_hashmap(
            [
                (tatooine_id, vec![(dagobah_id, 6), (hoth_id, 6)]),
                (
                    dagobah_id,
                    vec![(tatooine_id, 6), (endor_id, 4), (hoth_id, 1)],
                ),
                (endor_id, vec![(dagobah_id, 4), (hoth_id, 1)]),
                (
                    hoth_id,
                    vec![(dagobah_id, 1), (endor_id, 1), (tatooine_id, 6)],
                ),
            ]
            .into_iter()
            .collect(),
        )
        .unwrap();
        assert_eq!(galaxy_route, galaxy_route_gt);
    }

    fn get_planet_id_map() -> PlanetCatalog {
        PlanetCatalog::from_vec(vec![
            "Tatooine".to_string(),
            "Dagobah".to_string(),
            "Endor".to_string(),
            "Hoth".to_string(),
        ])
        .unwrap()
    }
}
