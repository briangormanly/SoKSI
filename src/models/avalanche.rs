extern crate rand;
use rand::Rng;

use crate::models::grain::Grain;
use crate::models::grain::GrainState;
use crate::models::location::Location;

use crate::util::constants::{DEBUG, DEBUG_AVALANCHE};


/**
 * Model for an avalanche in the sandpile
 * An avalanche is a collection of grains that have been preturbed and are moving
 */
pub struct Avalanche {
    pub id: u32,
    // Grains that are currently part of the avalanche
    pub grainIds: Vec::<u32>,
    // List of all locations that have been affected by any of the gains in the avalanche
    pub locationIds: Vec::<u32>,

    // total count of grain movement in the avalanche
    pub totalMovement: usize,

    // total count of grains involved in the avalanche
    pub totalGrainsInvolved: usize,
    
    // direction of the avalanche, determines which
    pub direction: usize,
}

impl Avalanche {
    pub fn new(id: u32) -> Self {
        Avalanche {
            id,
            grainIds: Vec::<u32>::new(),
            locationIds: Vec::<u32>::new(),
            direction: 0,
            totalMovement: 0,
            totalGrainsInvolved: 0,
        }
    }

    pub fn addGrain(&mut self, grainId: u32) {
        self.grainIds.push(grainId);
        self.totalGrainsInvolved += 1;
    }

    // update the movement of all the grains currently in the avalanche
    pub fn update( &mut self, grainId: u32) {

        // keep track of grains that need to be removed from the avalanche
        let mut toRemove = Vec::new();

        // get the grain from the grain list
        let mut grain = crate::models::grain::Grain::getGrainById(grainId).unwrap();


        if DEBUG && DEBUG_AVALANCHE { println!( "\n|{:?}| START Update for Grain {} at location | x: {}, y: {}, z: {} | has energy {}", grain.state, grain.id, grain.x, grain.y, grain.z, grain.energy) };
        match grain.state {
            GrainState::Unknown => {
                //println!( "Grain {} is responding to {:?} state", grain.id, grain.state);
                grain.state = GrainState::Falling;
                //grain.fall();
                grain.saveGrain();
            },
            GrainState::Falling => {
                //println!( "Grain {} is responding to {:?} state", grain.id, grain.state);
                // let the grain fall until it imparts a location
                self.totalMovement += grain.fall();
                grain.saveGrain();
            },
            GrainState::Impact => {
                // get the location with the same x, y, z as the gain
                //println!( "Grain {} is responding to {:?} state with xyz {}, {}, {}", grain.id, grain.state, grain.x, grain.y, grain.z);
                let mut location = crate::models::location::Location::getLocationByXyz(grain.x, grain.y, grain.z).unwrap();
                if DEBUG && DEBUG_AVALANCHE { println!( "------- IMPACT Location {} is starting with {} grains which are: {:?}", location.id, location.grainIds.len(), location.grainIds) };  

                // get the impact energy from the grain
                let impactEnergy: usize = grain.energy;

                location.incomingGrain(grain.id);
                location.saveLocation();

                
                if DEBUG && DEBUG_AVALANCHE { println!( "------- IMPACT Location {} is ending with {} grains, avalanche now has {} grains", location.id, location.grainIds.len(), self.grainIds.len()) }; 

                // if the location has more then 1 grain, check to see if the location has been perturbed by the impact
                // call the location perturbation method
                let mut rnd = rand::thread_rng();
                let perturbedGrains: Vec<u32> = location.perturbation(impactEnergy, &mut rnd);

                // if there are grains that have been perturbed, add them to the avalanche
                for perGrainId in perturbedGrains {
                    // add the perturbed grain to the avalanche if it is not already in the avalanche
                    if !self.grainIds.contains(&perGrainId) {
                        self.addGrain(perGrainId);
                    }
                    
                }           
                if DEBUG && DEBUG_AVALANCHE { println!( "------- IMPACT Avalanche now has {} grains", self.grainIds.len()) }; 

                
            },
            GrainState::Rolling => {
                self.totalMovement += grain.roll();
                grain.saveGrain();
            },
            GrainState::Stationary => {
                // remove the grain from the avalanche
                toRemove.push(grain.id);

                // ensure the grains energy is set to 0
                grain.energy = 0;
                grain.saveGrain();
            },
            GrainState::OffPile => {
                // remove the grain from the avalanche
                toRemove.push(grain.id);
            },
        }

        //println!( "|{:?}| END Update for Grain {} at location | x: {}, y: {}, z: {} | has energy {}", grain.state, grain.id, grain.x, grain.y, grain.z, grain.energy);
        // Remove the grains that were marked for removal
        //println!( "Removing grains {:?} avalanche contains before removal: {} grains", toRemove, self.grainIds.len());
        self.grainIds.retain(|id| !toRemove.contains(id));
        //println!( "Avalanche now has {} grains", self.grainIds.len());
        
    }

}