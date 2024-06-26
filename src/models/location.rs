// external modules
extern crate rand;
use rand::Rng;
use std::vec::Vec;
use std::fs::File;
use std::io::{self, BufWriter, Write, Read};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

// internal modules
use crate::util::sandpileUtil::normalizedPowerLawByOrdersOfMagnitude;
use crate::util::sandpileUtil::normalizedPowerLawByOrdersOfMagnitudeWithAlpha;

// Constants
use crate::util::constants::DEBUG;
use crate::util::constants::DEBUG_AVALANCHE;
use crate::util::constants::DEBUG_LOCATION;
use crate::util::constants::DEBUG_LOCAL_NEIGHBORS;
use crate::util::constants::DEBUG_GRAIN_IMPACT;
use crate::util::constants::DEBUG_DISPLAY_PILE;
use crate::util::constants::DEBUG_INIT;
use crate::util::constants::BASE_CAPACITY;
use crate::util::constants::BASE_RESILIENCE;
use crate::util::constants::ALPHA_EXTRA_ENERGY;
use crate::util::constants::ALPHA_AVALANCHE_SIZE;
use crate::util::constants::ALPHA_LOCATION_EXTRA_CAPACITY;
use crate::util::constants::ALPHA_LOCATION_EXTRA_RESILIENCE;
use crate::util::constants::X_SIZE;
use crate::util::constants::Y_SIZE;
use crate::util::constants::Z_SIZE;
use crate::util::constants::BASE_AVALANCHE_SIZE;
use crate::util::constants::BASE_AVALANCHE_METHOD;
use crate::util::constants::BASE_AVALANCHE_SIZE_PERCENT;

// internal models
use crate::models::grain::Grain;
use crate::models::avalanche::Avalanche;
use crate::models::grain::GrainState;

use super::avalanche;


//Static HashMap to store all the locations in the sandpile
lazy_static! { // Require the lazy_static crate to handle static Mutex
    // create a static mutex HashMap to store all the locations, use the location coordinates as the key for constant time access
    static ref LOCATIONS: Mutex<HashMap<(i32, i32, i32), Location>> = Mutex::new(HashMap::new());
}


/**
 * Model for a location in the sandpile
 * Locations are static and do not move, they represent a point in the 3D space
 * They have a capacity for grains and a resilience to perturbations which is 
 * determined as a random value between 1 and 6
 */
#[derive(Clone)]
pub struct Location {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub capacity: usize,
    pub grainIds: Vec::<u32>,
    pub resilience: usize,
}

impl Location {
    pub fn new(id: u32, x: i32, y: i32, z: i32, rnd: &mut impl Rng ) -> Self {

        // get the order of magnitude of a random power-law distribution
        let additionalCap = normalizedPowerLawByOrdersOfMagnitudeWithAlpha( ALPHA_LOCATION_EXTRA_CAPACITY, rnd ) as usize;
        let additionalRes = normalizedPowerLawByOrdersOfMagnitudeWithAlpha( ALPHA_LOCATION_EXTRA_RESILIENCE, rnd ) as usize;
        Location {
            id,
            x,
            y,
            z,
            capacity: BASE_CAPACITY + additionalCap,  
            grainIds: Vec::<u32>::new(),    // Initialize as empty vector
            resilience: BASE_RESILIENCE + additionalRes,  
        }
    }
    pub fn emptySpace(id: u32, x: i32, y: i32, z: i32) -> Self {

        Location {
            id,
            x,
            y,
            z,
            capacity: 0,  
            grainIds: Vec::<u32>::new(),    // Initialize as empty vector
            resilience: 0,  
        }
    }

    /**
     * retrieve a location by its id from the static HashMap
     */

    // Modify addLocation to use coordinates as the key
    fn addLocation(location: Location) {
        let mut locations = LOCATIONS.lock().unwrap();
        locations.insert((location.x, location.y, location.z), location);
    }

    // Add getLocationByLocation to retrieve a location by coordinates
    pub fn getLocationByXyz(x: i32, y: i32, z: i32) -> Option<Location> {
        let locations = LOCATIONS.lock().unwrap();
        locations.get(&(x, y, z)).cloned()
    }

    pub fn saveLocation(&mut self) {
        let mut locations = LOCATIONS.lock().unwrap();
        let location_key = (self.x, self.y, self.z);
        locations.insert(location_key, self.clone());

    }

    /**
     * Initialize all of the locations in the sandpile
     */
    pub fn initializeLocations(rnd: &mut impl Rng) {
        let mut count = 0;
        for x in 0..X_SIZE {
            for y in 0..Y_SIZE {
                for z in 0..Z_SIZE {

                    let location = if x>=z && x<=X_SIZE-z-1 && y>=z && y<=Y_SIZE-z-1 {
                        Location::new(count as u32, x as i32, y as i32, z as i32, rnd)
                    } else {
                        Location::emptySpace(count as u32, x as i32, y as i32, z as i32)
                    };

                    Location::addLocation(location); // Add location to the HashMap
                    count += 1;
                    
                }                
            }
        }

        if DEBUG && DEBUG_INIT {
            let locations = LOCATIONS.lock().unwrap();
            let length = locations.len();
            println!("---------------- Array of locations created with length: {} ----------------", length);
        }
    }

    /**
     * Attempt to add a grain to the location
     * 
     */
    pub fn incomingGrain(&mut self, grainId: u32) -> usize {

        // Check if the location has capacity to add a grain
        if self.grainIds.len() < self.capacity {
            if DEBUG && DEBUG_AVALANCHE { println!("Location x: {}, y: {}, z: {} has capacity to add grain {}, at impact grain total: {} and capacity {}", self.x, self.y, self.z, grainId, self.grainIds.len(), self.capacity) };
            // the location is not full, add the grain
            self.grainIds.push(grainId);

            // get the grain by its id
            let mut grain = Grain::getGrainById(grainId as u32).unwrap();

            // set the grain state to stationary
            grain.state = GrainState::Stationary;

            // remove the grains energy
            let energy = grain.energy;
            grain.energy = 0;

            // note that the grain stopped at this location
            //println!("Grain {} stopped at location x: {}, y: {}, z: {} Grain x: {}, y: {}, z: {}", grain.id, self.x, self.y, self.z, grain.x, grain.y, grain.z);

            // save the grain
            grain.saveGrain();


            return energy;
            

        } else {
            if DEBUG && DEBUG_AVALANCHE { println!("Location x: {}, y: {}, z: {} is full, grain {} will roll down the pile , at impact grain total: {} and capacity {}", self.x, self.y, self.z, grainId, self.grainIds.len(), self.capacity) };
            // if full the grain will roll down the pile
            // get the grain by its id
            let mut grain = Grain::getGrainById(grainId as u32).unwrap();

            // set the grain state back to rolling
            grain.state = GrainState::Rolling;

            let energy: usize = grain.energy;
            // // reduce the grains energy from the impact
            // if grain.energy > 1 {
            //     grain.energy = 1;
            // }
            // save the grain state
            grain.saveGrain();

            return energy;
        }
        
    }


    pub fn perturbation(&mut self, incomingGrainEnergy: usize, rnd: &mut impl Rng) -> Vec<u32> {
        // get the order of magnitude of a random power-law distribution
        // as random additional energy representing a perturbation of the location
        // add this value to the grains current energy
        let additionalEnergy = normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_EXTRA_ENERGY, rnd);
        let totalEnergy = incomingGrainEnergy + additionalEnergy as usize;

        // determine if this perturbation will cause an avalanche
        if DEBUG && DEBUG_AVALANCHE { 
            println!("resilience {} < total energy: {} ({} + {}) for location {}, {}, {}", self.resilience, totalEnergy, incomingGrainEnergy, additionalEnergy, self.x, self.y, self.z); 
        }

        if self.resilience < totalEnergy && self.z > 0 {
            // start an avalanche
            if DEBUG && DEBUG_AVALANCHE { println!("**************************!! Avalanche started at location x: {}, y: {}, z: {} location contains {} grains (before perturbation)", self.x, self.y, self.z, self.grainIds.len()) };
            // set the size of the avalanche
            let mut avalancheSize;
            if BASE_AVALANCHE_METHOD == 1 {
                // use a fixed size for the avalanche
                avalancheSize = BASE_AVALANCHE_SIZE + normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_AVALANCHE_SIZE, rnd) as usize;
                //if DEBUG && DEBUG_AVALANCHE { println!("+++++ Avalanche size: {}", avalancheSize) };
            } else {
                // use a percentage of the grains at the location for the avalanche
                avalancheSize = (self.grainIds.len() as f64 * BASE_AVALANCHE_SIZE_PERCENT) as usize + normalizedPowerLawByOrdersOfMagnitudeWithAlpha(ALPHA_AVALANCHE_SIZE, rnd) as usize;
                //if DEBUG && DEBUG_AVALANCHE { println!("+++++ Avalanche size: {}", avalancheSize) };
            }
            
            // ensure that the base avalanche size is not larger than the number of grains
            if self.grainIds.len() < avalancheSize {
                avalancheSize = self.grainIds.len();
            }

            let mut looseGrainIds: Vec<u32> = Vec::new();

            // add the grains to the avalanche and remove them from the location
            for i in 0..avalancheSize {
                looseGrainIds.push(self.grainIds.pop().unwrap());
            }

            // check all of the locations above the current location, any grains in these locations should join the avalanche
            let mut z_level = self.z + 1;
            //println!("\n\n-------\n");
            while z_level < Z_SIZE - 1 {
                
                //let ceilingGrains = Location::getCeilingLocations();
                let mut above_location = Location::getLocationByXyz(self.x, self.y, z_level).unwrap();
                
                // verify the location above has grains and is in the slope of criticality
                if above_location.capacity > 0 && above_location.grainIds.len() > 0 {
                    //println!("\nLocation x: {}, y: {}, z: {} has grains in the ceiling at x: {}, y: {}, z: {}", self.x, self.y, self.z, self.x, self.y, z_level);
                    //println!("ceiling grain had grains: {}", above_location.grainIds.len());
                    // list all the ids of the grains in the location above
                    //above_location.grainIds.iter().for_each(|id| println!("ceiling grain id: {}", id));
                    // show the loose grains before adding the grains from the location above
                    //looseGrainIds.iter().for_each(|id| println!(":: Loose grains before adding ceiling grains: {}", id));
                    // iterate through the grains in the location above and add them to the avalanche
                    for i in 0..above_location.grainIds.len() {
                        //println!("ceiling grain id found: {}", i);
                        looseGrainIds.push(above_location.grainIds.pop().unwrap());
                    }
                    // show the loose grains after adding the grains from the location above
                    //looseGrainIds.iter().for_each(|id| println!(":: Loose grains after adding ceiling grains: {}", id));

                    // save the above location
                    above_location.saveLocation();

                    // show the above location after removing the grains
                    //println!("ceiling grain had grains after (should be 0): {}", above_location.grainIds.len());

                    // // add the grains to the avalanche and remove them from the location
                    // for i in 0..above_location.grainIds.len() {
                    //     println!("ceiling grain id found: {}", i);
                    //     looseGrainIds.push(above_location.grainIds.pop().unwrap());
                    // }
                    //Location::displayPileToConsole();
                }

                
                //println!("after adding grains to avalanche, loc: {}, {}, {} checking above {}, {}, {} has # grains {}", self.x, self.y, self.z, self.x, self.y, z_level, above_location.grainIds.len());
                
                //println!("\n------------------\n");
                //}
                z_level += 1;
            }

            // update the grains that are now in the avalanche, set their state to rolling
            for grainId in &looseGrainIds {
                let mut grain = Grain::getGrainById(*grainId).unwrap();
                grain.state = GrainState::Rolling;
                grain.energy += 1;
                //additionalGrains.push(grain.id);
                

                grain.saveGrain();
            }

            // save the location
            self.saveLocation();

            if DEBUG && DEBUG_AVALANCHE { println!("+++++ Avalanche size: {}", avalancheSize) };

            if DEBUG && DEBUG_AVALANCHE { println!("**************************!! Avalanche at location x: {}, y: {}, z: {} location contains {} grains (after pertubation)", self.x, self.y, self.z, self.grainIds.len()) };
            //Location::displayPileToConsole();
            return looseGrainIds;

        } else {
            if DEBUG && DEBUG_AVALANCHE { println!("Location x: {}, y: {}, z: {} was not perturbed", self.x, self.y, self.z) };
            Vec::new() // Return an empty vector
        }
    }

    /**
     * Get the lower neighborhood of a location by its x, y, z coordinates
     */
    pub fn getLowerNeighborhood( x: i32, y: i32, z: i32 ) -> Vec<(i32, i32, i32)> {
        let mut lowerNeighborhood: Vec<(i32, i32, i32)> = Vec::with_capacity(9);

        let minX = if x == 0 { 0 } else { x-1 } as i32;
        let maxX = if x+1 < X_SIZE { x+1 } else { X_SIZE } as i32;
        let minY = if y == 0 { 0 } else { y-1 } as i32;
        let maxY = if y+1 < Y_SIZE { y+1 } else { Y_SIZE } as i32;
        if DEBUG && DEBUG_LOCAL_NEIGHBORS { println!("Neighborhood to check - minX: {}, maxX: {}, minY: {}, maxY: {} for z:: {}", minX, maxX, minY, maxY, z-1); }

        // keep track of how many locations are not at capacity in the lower neighborhood
        //let belowNumberOpen = 0;

        for i in minX..=maxX {
            for j in minY..=maxY {
                if z > 0 {
                    // If not at the ground level, normal neighborhood logic
                    lowerNeighborhood.push((i, j, z - 1));
                } else {
                    // Handling edge cases where grain might "fall off"
                    if i == x && j == y {
                        // Do not add the current location itself when z is 0
                        lowerNeighborhood.push((i, j, z - 1));
                    }
                    if i == 0 || i == X_SIZE - 1 || j == 0 || j == Y_SIZE - 1 {
                        // Marked locations indicating falling off the pile
                        lowerNeighborhood.push((-1, -1, -1)); // Use an invalid location (-1, -1, -1) to indicate falling off
                    } else {
                        // Add surrounding locations at the same level
                        lowerNeighborhood.push((i, j, z));
                    }
                }
            }
        }
        
        return lowerNeighborhood;
    }

    fn getCeilingLocations(x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
        let mut ceilingLocations: Vec<(i32, i32, i32)> = Vec::with_capacity(Z_SIZE as usize);

        // any grains located in locations above the current location should join the avalanche by falling down
        if z < Z_SIZE - 2 {
            for i in (z + 1)..Z_SIZE {
                ceilingLocations.push((x, y, i));
            }
        }
        return ceilingLocations;
    }

    /**
     * Display the contents of the sandpile
     */
    pub fn displayPile( folder_path: String ) -> io::Result<()> {

        // Create a file and wrap it in a BufWriter for efficient writing
        let file = File::create(folder_path + "/display-pile.txt")?;
        let mut writer = BufWriter::new(file);

        // show the contents of all the locations in the sandpile
        let mut grandTotal = 0;
        for z in (0..Z_SIZE).rev() {
            for y in 0..Y_SIZE {
                write!( writer, "\n")?;
                for x in 0..X_SIZE {

                    // get the location at this x, y, z
                    let location = Location::getLocationByXyz(x, y, z).unwrap();

                    // check to see if the location is within the slope of criticality
                    if location.capacity > 0 {
                        write!( writer, "{}", location.getNumberOfGrains())?;
                        grandTotal += location.getNumberOfGrains();
                    }
                    else {
                        write!( writer, " ")?;
                    }

                    //print!("x:{}, y:{}, z:{} value:{}", x, y, z, );
                    
                }
            }
            write!( writer, "\n")?;
        }
        writeln!( writer, " ")?;
        writeln!( writer, "Total grains in the pile: {}", grandTotal)?;

        // flush the writer to ensure all data is written to the file
        writer.flush()?;

        Ok(())

    }

    pub fn displayPileToConsole() {
        // show the contents of all the locations in the sandpile
        let mut grandTotal = 0;
        for z in (0..Z_SIZE).rev() {
            for y in 0..Y_SIZE {
                print!("\n");
                for x in 0..X_SIZE {

                    // get the location at this x, y, z
                    let location = Location::getLocationByXyz(x, y, z).unwrap();

                    // check to see if the location is within the slope of criticality
                    if location.capacity > 0 {
                        print!("{}", location.getNumberOfGrains());
                        grandTotal += location.getNumberOfGrains();
                    }
                    else {
                        print!(" ");
                    }

                    //print!("x:{}, y:{}, z:{} value:{}", x, y, z, );
                    
                }
            }
            print!("\n");
        }
        println!(" ");
        println!("Total grains in the pile: {}", grandTotal);
    }

    pub fn displayAllLocationFinalPositions( folder_path: String ) -> io::Result<()> {

        // Create a file and wrap it in a BufWriter for efficient writing
        let file = File::create(folder_path + "/display-all-locations.txt")?;
        let mut writer = BufWriter::new(file);

        // show the contents of all the locations in the sandpile
        for z in (0..Z_SIZE).rev() {
            for y in 0..Y_SIZE {
                for x in 0..X_SIZE {
                    // get the location at this x, y, z
                    let location = Location::getLocationByXyz(x, y, z).unwrap();

                    // print the location information
                    writeln!( writer, "\nx:{}, y:{}, z:{} grains: {:?}", x, y, z, location.grainIds)?;
                    // get all of the grains at this location and print their information
                    for grainId in &location.grainIds {
                        let grain = Grain::getGrainById(*grainId).unwrap();
                        writeln!( writer, " Grain id: {}, x: {}, y: {}, z: {}, energy: {}", grain.id, grain.x, grain.y, grain.z, grain.energy)?;
                    }
                }
            }
        }

        // flush the writer to ensure all data is written to the file
        writer.flush()?;

        Ok(())
    }

    pub fn getNumberOfGrains(&self) -> usize {
        return self.grainIds.len();
    }
}