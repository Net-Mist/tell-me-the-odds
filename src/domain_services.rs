use std::{cmp::Reverse, collections::BinaryHeap};

use anyhow::{Result};

use crate::domain_model::{BountyHunterPlanning, GalaxyRoutes, PlanetCatalog, PlanetId};

/// When exploring all the states to find the best route, we want to
/// privilege the one with less bounty hunter, then the one that took the less amont of time.
/// To do this, we derive the trait Ord that will automatically sort the State, first by
/// `n_bounty_hunter`, then by `elapsed_time` (then by `fluel` and then by `planet`, but this we don't care)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    pub n_bounty_hunter: u64,
    pub elapsed_time: u64,
    pub fluel: u64,
    pub planet: PlanetId,
}

pub fn compute_probability_of_success(
    hunter_planning: &BountyHunterPlanning,
    galaxy_routes: &GalaxyRoutes,
    planet_id_map: &PlanetCatalog,
    autonomy: u64,
    departure: &str,
    arrival: &str,
    countdown: u64,
) -> Result<f64> {
    let departure_id = match planet_id_map.get(departure) {
        Some(v) => v,
        None => return Ok(0.), // departure planet is not connected to the other planets. This is sad
    };

    let arrival_id = match planet_id_map.get(arrival) {
        Some(v) => v,
        None => return Ok(0.), // arrival planet is not connected to other planets. How did the rebel get there ?
    };

    let mut state_to_process = BinaryHeap::new();
    state_to_process.push(Reverse(State {
        n_bounty_hunter: 0,
        elapsed_time: 0,
        fluel: autonomy,
        planet: *departure_id,
    }));

    while let Some(state) = state_to_process.pop() {
        let state = state.0;
        if state.elapsed_time > countdown {
            continue;
        }
        let n_bounty_hunter = state.n_bounty_hunter
            + hunter_planning.meet_with_hunter(&state.planet, &state.elapsed_time);

        if state.planet == *arrival_id {
            return Ok(1. - probability_been_captured(n_bounty_hunter));
        }

        // Millennium Falcon can refluel
        state_to_process.push(Reverse(State {
            n_bounty_hunter,
            elapsed_time: state.elapsed_time + 1,
            fluel: autonomy,
            planet: state.planet,
        }));

        // or visit neightbours planets, if it has enough fluel
        for (new_planet_id, time) in galaxy_routes.get(&state.planet)? {
            if *time > state.fluel {
                continue;
            }
            state_to_process.push(Reverse(State {
                n_bounty_hunter,
                elapsed_time: state.elapsed_time + time,
                fluel: state.fluel - time,
                planet: *new_planet_id,
            }));
        }
    }

    Ok(0.)
}

fn probability_been_captured(n_bounty_hunter: u64) -> f64 {
    let mut nom = 1.;
    let mut den = 10.;
    let mut r = 0.;
    for _ in 0..n_bounty_hunter {
        r += nom / den;
        nom *= 9.;
        den *= 10.;
    }
    r
}

#[cfg(test)]
mod test {
    

    use crate::{
        domain_model::{BountyHunterPlanning, GalaxyRoutes, PlanetCatalog},
        domain_services::probability_been_captured,
    };

    use super::compute_probability_of_success;

    #[test]
    fn test_probability_been_captured() {
        assert_eq!(probability_been_captured(0), 0.);
        assert_eq!(probability_been_captured(1), 0.1);
        assert_eq!(probability_been_captured(2), 0.19);
        assert_eq!(probability_been_captured(3), 0.271);
    }

    #[test]
    fn test_compute_probability_of_success_simple() {
        let planet_id_map = PlanetCatalog::from_vec(vec![
            "Tatooine".to_string(),
            "Dagobah".to_string(),
            "Endor".to_string(),
        ])
        .unwrap();

        let tatooine_id = *planet_id_map.get("Tatooine").unwrap();
        let dagobah_id = *planet_id_map.get("Dagobah").unwrap();
        let endor_id = *planet_id_map.get("Endor").unwrap();

        let hunter_planning = BountyHunterPlanning::new(
            [(dagobah_id, [1].into_iter().collect())]
                .into_iter()
                .collect(),
        );
        let galaxy_routes = GalaxyRoutes::from_hashmap(
            [
                (tatooine_id, vec![(dagobah_id, 1)]),
                (dagobah_id, vec![(tatooine_id, 1), (endor_id, 1)]),
                (endor_id, vec![(dagobah_id, 1)]),
            ]
            .into_iter()
            .collect(),
        )
        .unwrap();

        let r = compute_probability_of_success(
            &hunter_planning,
            &galaxy_routes,
            &planet_id_map,
            2,
            "Tatooine",
            "Endor",
            2,
        )
        .unwrap();
        assert_eq!(r, 0.9);
    }

    #[test]
    fn test_compute_probability_of_success() {
        let planet_id_map = PlanetCatalog::from_vec(vec![
            "Tatooine".to_string(),
            "Dagobah".to_string(),
            "Endor".to_string(),
            "Hoth".to_string(),
        ])
        .unwrap();

        let tatooine_id = *planet_id_map.get("Tatooine").unwrap();
        let dagobah_id = *planet_id_map.get("Dagobah").unwrap();
        let endor_id = *planet_id_map.get("Endor").unwrap();
        let hoth_id = *planet_id_map.get("Hoth").unwrap();

        let hunter_planning = BountyHunterPlanning::new(
            [(hoth_id, [6, 7, 8].into_iter().collect())]
                .into_iter()
                .collect(),
        );

        let galaxy_routes = GalaxyRoutes::from_hashmap(
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

        let r = compute_probability_of_success(
            &hunter_planning,
            &galaxy_routes,
            &planet_id_map,
            6,
            "Tatooine",
            "Endor",
            7,
        )
        .unwrap();
        assert_eq!(r, 0.);

        let r = compute_probability_of_success(
            &hunter_planning,
            &galaxy_routes,
            &planet_id_map,
            6,
            "Tatooine",
            "Endor",
            8,
        )
        .unwrap();
        assert_eq!(r, 0.81);

        let r = compute_probability_of_success(
            &hunter_planning,
            &galaxy_routes,
            &planet_id_map,
            6,
            "Tatooine",
            "Endor",
            9,
        )
        .unwrap();
        assert_eq!(r, 0.9);

        let r = compute_probability_of_success(
            &hunter_planning,
            &galaxy_routes,
            &planet_id_map,
            6,
            "Tatooine",
            "Endor",
            10,
        )
        .unwrap();
        assert_eq!(r, 1.)
    }
}
