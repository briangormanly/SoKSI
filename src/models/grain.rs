// external modules
extern crate rand;
use rand::Rng;
use std::collections::HashMap;

use std::sync::Mutex;
use lazy_static::lazy_static;

// constants
use crate::util::constants::{ALPHA_LANDING, X_SIZE, Y_SIZE, Z_SIZE};
use crate::util::constants::{DEBUG, DEBUG_LOCATION, DEBUG_INIT, DEBUG_LOCAL_NEIGHBORS, DEBUG_AVALANCHE, DEBUG_GRAIN_IMPACT};
use crate::util::constants::{TOTAL_GRAINS, TERMINAL_FREE_FALL_SPEED};


// internal utilities
use crate::util::sandpileUtil::{normalizedPowerLawByOrdersOfMagnitudeWithAlpha};

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum GrainState {
    Unknown,
    Falling,
    Impact,
    Rolling,
    Stationary,
    OffPile,
}

lazy_static! { // Require the lazy_static crate to handle static Mutex
    // HashMap for grains indexed by coordinates (x, y, z)
    static ref GRAINS_BY_LOCATION: Mutex<HashMap<(i32, i32, i32), Vec<Grain>>> = Mutex::new(HashMap::new());
    
    // HashMap for grains indexed by ID
    static ref GRAINS_BY_ID: Mutex<HashMap<u32, Grain>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct Grain {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    // current energy of the grain, 
    // 0 if stationary, > 0 if in motion
    // energy is transferred to other grains on impact
    pub energy: usize,
    // state of the grain
    pub state: GrainState,
}

/**
 * Model for a grain of sand in the system
 * Will be initialized with initial energy and direction of 0
 * which should be set for grains in motion
 */
impl Grain {
    // Constructor to create a new Grain with a specific id
    pub fn new(id: u32) -> Grain {
        let (x, y, z) = Grain::determineInitialPosition(id);
        Grain { 
            id, 
            // current energy of the grain, 
            // 0 if stationary, > 0 if in motion
            // energy is transferred to other grains on impact
            energy: 0,
            x,
            y,
            z,    
            state: GrainState::Unknown,        
        }
    }

    pub fn initializeGrains() {
         // initialize all the grains in the array
        for i in 0..TOTAL_GRAINS {
            // create a grain 
            let grain = Grain::new(i as u32);

            Grain::addGrain(grain);

        }
        if DEBUG && DEBUG_INIT {
            let grains: std::sync::MutexGuard<'_, HashMap<u32, Grain>> = GRAINS_BY_ID.lock().unwrap();
            let length: usize = grains.len();
            if DEBUG && DEBUG_INIT { println!( "---------------- Grains created with count: {} ----------------", grains.len()) };
        }
    }

    // Method to retrieve grains by location
    pub fn getGrainsByLocation(x: i32, y: i32, z: i32) -> Vec<Grain> {
        let grains: std::sync::MutexGuard<'_, HashMap<(i32, i32, i32), Vec<Grain>>> = GRAINS_BY_LOCATION.lock().unwrap();
        grains.get(&(x, y, z)).cloned().unwrap_or_else(Vec::new)
    }

    // Method to retrieve a grain by ID
    pub fn getGrainById(id: u32) -> Option<Grain> {
        let grains: std::sync::MutexGuard<'_, HashMap<u32, Grain>> = GRAINS_BY_ID.lock().unwrap();
        grains.get(&id).cloned()
    }

    /**
     * Save the grain to the system
     * Handles adding the grain to the grains_by_location and grains_by_id HashMaps
     */
    pub fn saveGrain(&mut self) {
        let mut grains_by_location: std::sync::MutexGuard<'_, HashMap<(i32, i32, i32), Vec<Grain>>> = GRAINS_BY_LOCATION.lock().unwrap();
        let mut grains_by_id: std::sync::MutexGuard<'_, HashMap<u32, Grain>> = GRAINS_BY_ID.lock().unwrap();

        let location_key: (i32, i32, i32) = (self.x, self.y, self.z);
        grains_by_location.entry(location_key).or_insert_with(Vec::new).push(self.clone());

        grains_by_id.insert(self.id, self.clone());
        
    }

    pub fn fall(&mut self) -> usize {

        let mut movement: usize = 0;

        // get the location with the same x, y, z as the gain
        let location: super::location::Location = crate::models::location::Location::getLocationByXyz(self.x, self.y, self.z).unwrap();
        // get the location with z-1 if z > 0
        if self.z > 0 {
            let below_location = crate::models::location::Location::getLocationByXyz(self.x, self.y, self.z-1).unwrap();
            // check to see if the location is empty space (not part of the pile) this is known because it will have a capacity and resilience of 0
            if location.capacity == 0 && location.resilience == 0 || ( self.z > 0 && below_location.grainIds.len() < below_location.capacity ) {
                // the grain is in free fall
                self.z -= 1;
                movement += 1;

                // if the grain is in free fall, increase the energy up to the terminal velocity
                if self.energy < TERMINAL_FREE_FALL_SPEED {
                    self.energy += 1;
                }
                
                
            } else {
                // the grain has impacted a location
                self.state = GrainState::Impact;
            }

        }
        else {
            // the grain has impacted a location
            self.state = GrainState::Impact;
        }

        return movement;

    }

    /**
     * Roll the grain to a lower location
     */
    pub fn roll(&mut self) -> usize {

        // keep track of the grain movement
        let mut movement: usize = 0;

        // get the lower neighborhood for this location
        let lowerNeighborhood: Vec<(i32, i32, i32)> = crate::models::location::Location::getLowerNeighborhood(self.x, self.y, self.z);

        // print out the lower neighborhood which contains a Vec<(i32, i32, i32)>
        // if DEBUG && DEBUG_LOCAL_NEIGHBORS {
        //     println!( "Grain {} is rolling to a lower location", self.id);
        //     for (x, y, z) in lowerNeighborhood {
        //         println!( "x: {}, y: {}, z: {} - Capacity: {}", x, y, z, crate::models::location::Location::getLocationByXyz(x, y, z).unwrap().capacity);
        //     }
        // }

        // check to see if the grain can fall to a lower location
        if lowerNeighborhood.len() > 0 {
            // pick a location at random from the lower neighborhood and fall to it.
            let mut rnd = rand::thread_rng();
            let locationIndex = rnd.gen_range(0..lowerNeighborhood.len());

            // move the grain to the new location
            self.x = lowerNeighborhood[locationIndex].0;
            self.y = lowerNeighborhood[locationIndex].1;
            self.z = lowerNeighborhood[locationIndex].2;
            self.energy += 1;
            
            movement += 1;
            
            
            if DEBUG && DEBUG_AVALANCHE {
                println!( "Grain {} rolled to x: {}, y: {}, z: {}", self.id, self.x, self.y, self.z);
            }

            // check for out of bounds and remove the grain from the system (it fell off the edge)
            if self.x <= -1 || self.y <= -1 || self.z <= -1 || self.x >= X_SIZE || self.y >= Y_SIZE {
                self.state = GrainState::OffPile;
            }
            else {
                self.state = GrainState::Impact;
            }
            
            
        } else {
            // the grain has made it all the way to the lowest level, check the pier neighboorhood at the same level
            let pierNeighborhood: Vec<(i32, i32, i32)> = crate::models::location::Location::getLowerNeighborhood(self.x, self.y, self.z);
            
            movement += 1;

            self.state = GrainState::Stationary;
        }
        return movement;

    }

    pub fn displayAllGrainsLocations() {
        let grains_by_id_guard = GRAINS_BY_ID.lock().unwrap();
        let mut grains: Vec<_> = grains_by_id_guard.iter().collect();
    
        // Sort the grains by their ID in ascending order
        grains.sort_by_key(|&(id, _)| id);
    
        for (id, grain) in grains {
            println!( "Grain {} is at x: {}, y: {}, z: {}", id, grain.x, grain.y, grain.z);
        }
    
        // Display the total number of grains
        println!( "Total grains: {}", grains_by_id_guard.len());
    }

    /**
     * Add a grain to the system
     * Handles adding the grain to the grains_by_location and grains_by_id HashMaps
     */
    fn addGrain(grain: Grain) {
        let mut grains_by_location: std::sync::MutexGuard<'_, HashMap<(i32, i32, i32), Vec<Grain>>> = GRAINS_BY_LOCATION.lock().unwrap();
        let mut grains_by_id: std::sync::MutexGuard<'_, HashMap<u32, Grain>> = GRAINS_BY_ID.lock().unwrap();

        let location_key: (i32, i32, i32) = (grain.x, grain.y, grain.z);
        grains_by_location.entry(location_key).or_insert_with(Vec::new).push(grain.clone());

        grains_by_id.insert(grain.id, grain);
    }
    
    


    /**
     * Determine the initial position of the grain
     * 
     * @param id - the id of the grain
     * @return (x, y, z) - the initial position of the grain
     */
    fn determineInitialPosition(id: u32) -> (i32, i32, i32) {

        let mut rnd = rand::thread_rng();

        // start with center of the array
        let mut x = X_SIZE / 2;
        let mut y = Y_SIZE / 2;

        // find the gains landing variance from center with more variance in the center
        // using an alpha of 1.5
        let mut xVariance = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_LANDING, &mut rnd);
        let mut yVariance = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_LANDING, &mut rnd);

        // check that the variance is within the bounds of the array
        if xVariance > x as f64 {
            xVariance = x as f64;
        }
        if yVariance > y as f64 {
            yVariance = y as f64;
        }

        // find the gains landing direction
        let xDirection = rnd.gen_range(0..2);
        let yDirection = rnd.gen_range(0..2);

        // compute the new location of the grain given the variance and direction
        if xDirection == 0 {
            x -= xVariance as i32;
        } else {
            x += xVariance as i32;
        }
        if yDirection == 0 {
            y -= yVariance as i32;
        } else {
            y += yVariance as i32;
        }

        let z = (Z_SIZE - 1) as i32;

        if DEBUG && DEBUG_LOCATION {
            println!( "Grain {} initialized at: {}, y: {}, z: {}", id, x, y, z);
        }

        (x, y, z)
    }
    
}
