use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use anyhow::anyhow;
use anyhow::Result;

/// id to identify a planet.
/// Please note that the constructor is private to this module,
/// to limit the ability to create wrong PlanetId.
/// Also note that after compilation, PlanetId will give the same
/// assembly code that using usize, thanks to Rust 0-cost abstraction.
/// PlanetId are managed only by PlanetCatalog
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct PlanetId(usize);

impl PlanetId {
    fn new(id: usize) -> PlanetId {
        PlanetId(id)
    }
}

impl Display for PlanetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The `GalaxyRoutes` structure defines all interstellar routes within the galaxy.
/// If there exists a route between planets with IDs `N` and `M`, and a Hyperspace jump between these planets take `D` days,
/// then the vector at hash index `N` contains an entry `(M, D)`, and similarly,
/// the vector at hash index `M` contains an entry `(N, D)`.
/// This design facilitates quick access to all routes originating from a specific planet.
/// It's important to note that planets are identified not by their names but by a `PlanetId`,
/// ensuring that the representation of planets remains independent of their names.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GalaxyRoutes(HashMap<PlanetId, Vec<(PlanetId, u64)>>);

impl Default for GalaxyRoutes {
    fn default() -> Self {
        Self::new()
    }
}

impl GalaxyRoutes {
    pub fn get(&self, planet_id: &PlanetId) -> Result<&Vec<(PlanetId, u64)>> {
        match self.0.get(planet_id) {
            Some(v) => Ok(v),
            None => Err(anyhow!("planet_id not in GalaxyRoute.")),
        }
    }

    pub fn new() -> GalaxyRoutes {
        GalaxyRoutes(HashMap::new())
    }

    pub fn add_route(&mut self, planet_id1: PlanetId, planet_id2: PlanetId, travel_time: u64) {
        self.0
            .entry(planet_id1)
            .or_default()
            .push((planet_id2, travel_time));
        self.0
            .entry(planet_id2)
            .or_default()
            .push((planet_id1, travel_time));
    }

    /// create a GalaxyRoute object directly from a hashmap.
    /// Check that the hashmap validate the constraints of the GalaxyRoute internal hashmap.
    /// Note that the implementation is not optimal in term of speed, but as it should only be used by unit-test, this shouldn't be an issue
    pub fn from_hashmap(
        galaxy_routes: HashMap<PlanetId, Vec<(PlanetId, u64)>>,
    ) -> Result<GalaxyRoutes> {
        // check that if path from A to B exist, then path from B to A also exists.
        let origins = galaxy_routes.keys();
        for origin in origins {
            for (destination, distance) in galaxy_routes.get(origin).unwrap().iter() {
                if !galaxy_routes.contains_key(destination) {
                    return Err(anyhow!(
                        "Route starting from {origin} to {destination} found, but no routes starting from {destination}"                    ));
                }
                if !galaxy_routes
                    .get(destination)
                    .unwrap()
                    .contains(&(*origin, *distance))
                {
                    return Err(anyhow!("Route from {origin} to {destination} found, but no similar route from {destination} to {origin}"));
                }
            }
        }
        Ok(GalaxyRoutes(galaxy_routes))
    }
}

/// Structure keeping the relationship between the planet id and its information (for now only name).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PlanetCatalog(HashMap<String, PlanetId>);

impl Default for PlanetCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl PlanetCatalog {
    pub fn get(&self, k: &str) -> Option<&PlanetId> {
        self.0.get(k)
    }

    pub fn insert(&mut self, planet_name: String) -> Result<&PlanetId> {
        if self.0.contains_key(&planet_name) {
            return Err(anyhow!(
                "Can't insert a planet that already exist in the map"
            ));
        }
        let planet_id = PlanetId::new(self.0.len());
        self.0.insert(planet_name.to_owned(), planet_id);
        Ok(self.0.get(&planet_name).unwrap())
    }

    pub fn get_or_insert(&mut self, planet_name: String) -> PlanetId {
        if let Some(v) = self.get(&planet_name) {
            return *v;
        };
        *self.insert(planet_name).unwrap()
    }

    pub fn new() -> Self {
        PlanetCatalog(HashMap::new())
    }

    pub fn from_vec(planet_names: Vec<String>) -> Result<PlanetCatalog> {
        let mut planet_id_map = PlanetCatalog::new();
        for planet_name in planet_names {
            planet_id_map.insert(planet_name)?;
        }
        Ok(planet_id_map)
    }
}

/// Structure that remember the days when bounty hunter are present on a planet
#[derive(Debug, Eq, PartialEq)]
pub struct BountyHunterPlanning(HashMap<PlanetId, HashSet<u64>>);

impl BountyHunterPlanning {
    /// Return 1 if meet with hunter, else 0
    pub fn meet_with_hunter(&self, planet: &PlanetId, elapsed_time: &u64) -> u64 {
        if let Some(v) = self.0.get(planet) {
            if v.contains(elapsed_time) {
                return 1;
            }
        }
        0
    }

    pub fn new(planet_to_days: HashMap<PlanetId, HashSet<u64>>) -> BountyHunterPlanning {
        BountyHunterPlanning(planet_to_days)
    }
}
