use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use anyhow::Result;

use crate::domain_models::{BountyHunterPlanning, GalaxyRoutes, PlanetCatalog, PlanetId};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    n_bounty_hunter: u64,
    elapsed_time: u64,
    time_to_destination: u64,
    fuel: u64,
    planet: PlanetId,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.n_bounty_hunter != other.n_bounty_hunter {
            return self.n_bounty_hunter.cmp(&other.n_bounty_hunter);
        }
        let sum_time = self.elapsed_time + self.time_to_destination;
        let other_sum_time = other.elapsed_time + other.time_to_destination;
        sum_time.cmp(&other_sum_time)
    }
}

#[derive(PartialEq, Eq)]
struct AllTimeState {
    time: u64,
    planet_id: PlanetId,
}

impl PartialOrd for AllTimeState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AllTimeState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

/// run a Dijkstra algorithm to compute the minimal distance from a planet to the destination,
/// without considering the bounty hunters.
/// the output of this function will be the A* heuristic
fn compute_all_time_to_destination(
    galaxy_routes: &GalaxyRoutes,
    destination_id: &PlanetId,
) -> Result<HashMap<PlanetId, u64>> {
    let mut time_to_destination = HashMap::new();
    let mut planet_to_process = BinaryHeap::from([Reverse(AllTimeState {
        time: 0,
        planet_id: *destination_id,
    })]);

    while let Some(Reverse(state)) = planet_to_process.pop() {
        if let std::collections::hash_map::Entry::Vacant(e) =
            time_to_destination.entry(state.planet_id)
        {
            // first time we see this planet
            e.insert(state.time);
        } else {
            // this planet has already been processed
            continue;
        }
        for (neighbour_planet_id, time) in galaxy_routes.get(&state.planet_id)? {
            planet_to_process.push(Reverse(AllTimeState {
                time: state.time + time,
                planet_id: *neighbour_planet_id,
            }));
        }
    }
    Ok(time_to_destination)
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

    let all_time_to_destination = compute_all_time_to_destination(galaxy_routes, arrival_id)?;

    let mut state_to_process = BinaryHeap::from([Reverse(State {
        n_bounty_hunter: 0,
        elapsed_time: 0,
        fuel: autonomy,
        planet: *departure_id,
        time_to_destination: *all_time_to_destination
            .get(departure_id)
            .unwrap_or(&u64::MAX),
    })]);

    let mut seen_state = HashSet::new();

    while let Some(Reverse(state)) = state_to_process.pop() {
        if seen_state.contains(&state) {
            // this state has already been explored
            continue;
        }
        seen_state.insert(state.clone());
        if state.elapsed_time.saturating_add(state.time_to_destination) > countdown {
            // then it is not possible to reach the destination from this state
            continue;
        }
        let n_bounty_hunter = state.n_bounty_hunter
            + hunter_planning.meet_with_hunter(&state.planet, &state.elapsed_time);

        if state.planet == *arrival_id {
            return Ok(1. - probability_been_captured(n_bounty_hunter));
        }

        // Millennium Falcon can refuel
        state_to_process.push(Reverse(State {
            n_bounty_hunter,
            elapsed_time: state.elapsed_time + 1,
            fuel: autonomy,
            planet: state.planet,
            time_to_destination: state.time_to_destination,
        }));

        // or visit neightbours planets, if it has enough fluel
        for (new_planet_id, time) in galaxy_routes.get(&state.planet)? {
            if *time > state.fuel {
                continue;
            }
            state_to_process.push(Reverse(State {
                n_bounty_hunter,
                elapsed_time: state.elapsed_time + time,
                fuel: state.fuel - time,
                planet: *new_planet_id,
                time_to_destination: *all_time_to_destination
                    .get(new_planet_id)
                    .unwrap_or(&u64::MAX),
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

    use std::collections::{HashMap, HashSet};

    use crate::{
        domain_models::{BountyHunterPlanning, GalaxyRoutes, PlanetCatalog},
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

        let hunter_planning =
            BountyHunterPlanning::new([(dagobah_id, HashSet::from([1]))].into_iter().collect());
        let galaxy_routes = GalaxyRoutes::from_hashmap(HashMap::from([
            (tatooine_id, vec![(dagobah_id, 1)]),
            (dagobah_id, vec![(tatooine_id, 1), (endor_id, 1)]),
            (endor_id, vec![(dagobah_id, 1)]),
        ]))
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
