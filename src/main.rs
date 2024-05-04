#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

/*

 - Discrete sandpile targeting criticality
  Starts with everything from 2-sandpile-basic-random
  Additions / Changes:
   * Density for each pile location
     * Sandpile locations have a capacity of 4 grains plus the output of the order of
     *  magnitude power-law distribution
     *
   * Moment
     * Gains move with a magnitude (speed) and direction
       * Initial speed starts at 1 but kinetic energy can be transferred in collisions
       * speed increases as grain falls
     * Direction of impacted grain movement is determined by direction of impacting grain
   * Energy from impacts radiate through surrounding grains

*/

// external modules
extern crate rand;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write, Read};
use std::vec::Vec;
use rand::Rng;
use chrono::Local;


// internal modules
pub mod models;
pub mod util;

// external structs and functions
use models::avalanche::Avalanche;
use models::grain::Grain;
use models::location::Location;
use models::avalanche;


use util::sandpileUtil::normalizedPowerLawByOrdersOfMagnitudeWithAlpha;


// Constants
use util::constants::ALPHA_LANDING;
use util::constants::DEBUG;
use util::constants::DEBUG_AVALANCHE;
use util::constants::DEBUG_INIT;
use util::constants::DEBUG_DISPLAY_PILE;
use util::constants::DEBUG_LOCAL_NEIGHBORS;
use util::constants::X_SIZE;
use util::constants::Y_SIZE;
use util::constants::Z_SIZE;
use util::constants::TERMINAL_FREE_FALL_SPEED;
use util::constants::BASE_CAPACITY;
use util::constants::TOTAL_GRAINS;
use util::constants::BASE_RESILIENCE;
use util::constants::ALPHA_MAIN;
use util::constants::ALPHA_EXTRA_ENERGY;
use util::constants::ALPHA_AVALANCHE_SIZE;
use util::constants::ALPHA_LOCATION_EXTRA_CAPACITY;
use util::constants::ALPHA_LOCATION_EXTRA_RESILIENCE;
use util::constants::BASE_AVALANCHE_METHOD;
use util::constants::BASE_AVALANCHE_SIZE;
use util::constants::BASE_AVALANCHE_SIZE_PERCENT;



fn main() {
    
    
    // Each run's data is stored in a folder named with the current timestamp-number of grains-size of pile
    
    // Generate the current timestamp as a folder name
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let start_time: chrono::prelude::DateTime<Local> = Local::now(); 
    
    //let timestamp = format!("{}-{}-{}", timestamp, TOTAL_GRAINS, X_SIZE * Y_SIZE * Z_SIZE);
    let folder_path = format!("./data/{}", timestamp + "-gs-" + &TOTAL_GRAINS.to_string() + "-ps-" + &X_SIZE.to_string() + "-" + &Y_SIZE.to_string() + "-" + &Z_SIZE.to_string());

    // Create the directory using the path
    let _ = fs::create_dir_all(&folder_path);


    // create a random number generator
    let mut rnd = rand::thread_rng();

    // initialize the locations as a static mutex hashmap
    models::location::Location::initializeLocations(&mut rnd);


    // initialize a vec of all grains
    //let mut grains: Vec<Grain> = Vec::with_capacity(TOTAL_GRAINS);
    models::grain::Grain::initializeGrains();


    // initialize all the grains in the array
    //initializeGrains(&mut grains, &mut rnd);

    // initialize a vec of all avalanches
    let mut avalanches: Vec<Avalanche> = Vec::with_capacity(TOTAL_GRAINS);

    // initialize all the avalanches in the array each grain causes an avalanche
    // of some size, might be as small as joining the first location it lands on
    // or as big
    initializeAvalanches(&mut avalanches);

    if DEBUG && DEBUG_INIT {
        println!( "---------------- Avalanches created with count: {} ----------------", avalanches.len());
    }


    // for each grain, create an avalanche
    for i in 0..TOTAL_GRAINS {

        // Add the new falling grain to the avalanche, this is grain 0
        avalanches[i].addGrain(i as u32);

        if DEBUG && DEBUG_AVALANCHE { println!( "\n\n----------------------------------------------------------------------------------------------") };
        if DEBUG && DEBUG_AVALANCHE { println!( "Avalanche {} START", i) };

        // print out all of the states of the grains in the avalanche
        for grainId in &avalanches[i].grainIds {
            let grain = models::grain::Grain::getGrainById(*grainId).unwrap();
        }

        // Run through the avalanche until all grains have come to rest
        // first get the initial number of grains in the avalanche
        let mut totalGrains = avalanches[i].grainIds.len();

        // while the number of grains in the avalanche is greater than 0, this avalanche is still active
        while totalGrains > 0 {
            // determine the number of grains in the avalanche at this point in time
            totalGrains = avalanches[i].grainIds.len();

            // for each grain currently in the avalanche, update the grain at this time period
            let previous_len = totalGrains;
            for mut j in 0..totalGrains {
                // get the grains id
                //println!( "about to look for avalanche index {} with grain index {}, the total grains in the avalanche is {} and the previous index was {}", i, j, avalanches[i].grainIds.len(), previous_len);

                // if the number of grains in the avalanche has changed, decrease the index
                if avalanches[i].grainIds.len() < previous_len && j > 0 {
                    j = avalanches[i].grainIds.len() -1;
                }
                let grainId = avalanches[i].grainIds[j];
                // get the amount of grains in the avalanche before the update
                let previous_len = avalanches[i].grainIds.len();

                // perform the update on the grain
                avalanches[i].update( grainId );

            }
        }

        if DEBUG && DEBUG_AVALANCHE { println!( "Avalanche {} END: total movement: {}, total grains involved: {}", i, avalanches[i].totalMovement, avalanches[i].totalGrainsInvolved) };
        if DEBUG && DEBUG_AVALANCHE { println!( "/n/n----------------------------------------------------------------------------------------------") };
    }

    //draw the pile
    if DEBUG && DEBUG_DISPLAY_PILE {

        // output the run configuration to a file
        let _ = displayApplicationRunConfiguration(folder_path.clone());
        
        println!("Final breakdown of grains at all locations export --------------------------------------------------------------");
        let _ = models::location::Location::displayAllLocationFinalPositions(folder_path.clone());
        //models::grain::Grain::displayAllGrainsLocations();
        println!("Visual pile export ---------------------------------------------------------------------------------------------");
        let _ = models::location::Location::displayPile(folder_path.clone());

        // print the total movement of the avalanche
        println!("Total movement export ------------------------------------------------------------------------------------------");
        let _ = displayAvalancheTotalMovementStats(&avalanches, folder_path.clone());
        println!("Grain stats export ---------------------------------------------------------------------------------------------");
        let _ = displayAvalancheTotalGrainsStats(&avalanches, folder_path.clone());
        println!("Magnitude export------------------------------------------------------------------------------------------------");
        let _ = displayAvalancheTotalMagnitude(&avalanches, folder_path.clone());
        println!("Exporting data -------------------------------------------------------------------------------------------------");
        let _ = recordExportedData(&avalanches, folder_path.clone());
    }

    // output the total running time of the program using the start_time
    let end_time: chrono::prelude::DateTime<Local> = Local::now();
    let duration: chrono::TimeDelta = end_time.signed_duration_since(start_time);
    println!("Total time: {:?}", duration);

}


fn initializeAvalanches(avalanches: &mut Vec<Avalanche>) {
    for i in 0..TOTAL_GRAINS {
        // create a grain
        let avalanche = Avalanche::new(i as u32);
        avalanches.push(avalanche);
    }
}

pub fn displayApplicationRunConfiguration(folder_path: String) -> io::Result<()> {
    // Create a file and wrap it in a BufWriter for efficient writing
    let file = File::create(folder_path + "/run-configuration.txt")?;
    let mut writer = BufWriter::new(file);

    writeln!( writer, "Run configuration")?;
    writeln!( writer, "---------------------------------------------------------------------------------------------------")?;
    writeln!( writer, "Total Grains: {}", TOTAL_GRAINS)?;
    writeln!( writer, "Pile Size: {} x {} x {}", X_SIZE, Y_SIZE, Z_SIZE)?;
    writeln!( writer, "Terminal Free Fall Speed: {}", TERMINAL_FREE_FALL_SPEED)?;
    writeln!( writer, "Base Resilience: {}", BASE_RESILIENCE)?;
    writeln!( writer, "Base Capacity: {}", BASE_CAPACITY)?;
    writeln!( writer, "Base Avalanche Method: {}", BASE_AVALANCHE_METHOD)?;
    writeln!( writer, "Base Avalanche Size: (base for avalanche method=1) {}", BASE_AVALANCHE_SIZE)?;
    writeln!( writer, "Base Avalanche Size Percent (for avalanche method=2): {}", BASE_AVALANCHE_SIZE_PERCENT)?;
    writeln!( writer, "Alpha Main (default - not used currently): {}", ALPHA_MAIN)?;
    writeln!( writer, "Alpha Landing (variance for initial x,y deviation from center): {}", ALPHA_LANDING)?;
    writeln!( writer, "Alpha Extra Energy (amount of additional energy added to impact): {}", ALPHA_EXTRA_ENERGY)?;
    writeln!( writer, "Alpha Avalanche Size (additional grains that join avalanche added to BASE by selected Method): {}", ALPHA_AVALANCHE_SIZE)?;
    writeln!( writer, "Alpha Location Extra Capacity (additional capacity at location): {}", ALPHA_LOCATION_EXTRA_CAPACITY)?;
    writeln!( writer, "Alpha Location Extra Resilience (additional resilience of location): {}", ALPHA_LOCATION_EXTRA_RESILIENCE)?;
    writeln!( writer, "---------------------------------------------------------------------------------------------------")?;

    // flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())
}

pub fn displayAvalancheTotalGrainsStats(avalanches: &Vec<Avalanche>, folder_path: String) -> io::Result<()> {
    
    // Create a file and wrap it in a BufWriter for efficient writing
    let file = File::create(folder_path + "/grain-stats.csv")?;
    let mut writer = BufWriter::new(file);


    // build a hashmap that will store a vector of ids of avalanches for each discrete total grain value within the avalanches vector.
    let mut avalancheTotalGrainsMap: HashMap<usize, Vec<u32>> = HashMap::new();

    // for each avalanche in the vector, add the avalanche id to the vector of ids for the total grain value
    for avalanche in avalanches {
        let totalGrains = avalanche.totalGrainsInvolved;
        if avalancheTotalGrainsMap.contains_key(&totalGrains) {
            avalancheTotalGrainsMap.get_mut(&totalGrains).unwrap().push(avalanche.id);
        } else {
            avalancheTotalGrainsMap.insert(totalGrains, vec![avalanche.id]);
        }
    }

    // print out the total grain value the ids of the avalanches that have that total grain value in ascending order of grain value
    let mut sortedKeys: Vec<usize> = avalancheTotalGrainsMap.keys().cloned().collect();

    sortedKeys.sort();
    writeln!( writer, "Avalanche Grain Count,  Number Avalanches")?;
    for totalGrains in sortedKeys {
        writeln!( writer, "{}, {:?}", totalGrains, avalancheTotalGrainsMap.get(&totalGrains).unwrap().len())?;
    }

    // print out the total grain value and the ids of the avalanches that have that total grain value
    // for (totalGrains, ids) in avalancheTotalGrainsMap {
    //     println!( "Total Grains: {}", totalGrains);
    //     println!( "Avalanche Ids: {:?}", ids);
    // }

    // flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())

}

/** 
 * Create a txt file in the format that can be imported into python for powerlaw analysis using the powerlaw library
 * the file contains only the total movement, each row indicates the number of avalanches with that total movement
 * The first row is number of avalanches with total movement 1, the second row is number of avalanches with total movement 2, etc
 * If no avalanches of a particular movement size existed in the data that size should be included (all movement sizes 
 * should be in the data from 1-> n, n = largest movement), include these rows with the row value 0
 */
pub fn recordExportedData(avalanches: &Vec<Avalanche>, folder_path: String) -> io::Result<()> {
    // Create a file and wrap it in a BufWriter for efficient writing
    let file = File::create(folder_path + "/py-powerlaw-import.txt")?;
    let mut writer = BufWriter::new(file);

    // build a hashmap that will store a vector of ids of avalanches for each discrete total movement value within the avalanches vector.
    let mut avalancheTotalMovementMap: HashMap<usize, Vec<u32>> = HashMap::new();

    // add the total movement of each avalanche to the hashmap, keep track of the largest movement
    let mut largestMovement = 0;
    for avalanche in avalanches {
        let totalMovement = avalanche.totalMovement;
        if totalMovement > largestMovement {
            largestMovement = totalMovement;
        }
        if avalancheTotalMovementMap.contains_key(&totalMovement) {
            avalancheTotalMovementMap.get_mut(&totalMovement).unwrap().push(avalanche.id);
        } else {
            avalancheTotalMovementMap.insert(totalMovement, vec![avalanche.id]);
        }
    }

    // loop from 1 to the largest movement size, if the movement size is in the hashmap print out the number of avalanches with that movement size, otherwise print 0
    for i in 1..largestMovement+1 {
        if avalancheTotalMovementMap.contains_key(&i) {
            writeln!( writer, "{}", avalancheTotalMovementMap.get(&i).unwrap().len())?;
        } else {
            writeln!( writer, "0")?;
        }
    }
    
    // flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())
}

pub fn displayAvalancheTotalMovementStats(avalanches: &Vec<Avalanche>, folder_path: String) -> io::Result<()> {

    // Create a file and wrap it in a BufWriter for efficient writing
    let file = File::create(folder_path + "/avalanche-movement-stats.csv")?;
    let mut writer = BufWriter::new(file);
    
    // build a hashmap that will store a vector of ids of avalanches for each discrete total movement value within the avalanches vector.
    let mut avalancheTotalMovementMap: HashMap<usize, Vec<u32>> = HashMap::new();

    // for each avalanche in the vector, add the avalanche id to the vector of ids for the total movement value
    for avalanche in avalanches {
        let totalMovement = avalanche.totalMovement;
        if avalancheTotalMovementMap.contains_key(&totalMovement) {
            avalancheTotalMovementMap.get_mut(&totalMovement).unwrap().push(avalanche.id);
        } else {
            avalancheTotalMovementMap.insert(totalMovement, vec![avalanche.id]);
        }
    }

    // print out the total movment value the ids of the avalanches that have that total movement value in ascending order of movement value
    let mut sortedKeys: Vec<usize> = avalancheTotalMovementMap.keys().cloned().collect();

    sortedKeys.sort();
    writeln!( writer, "Avalanche Movement, Number Avalanches")?;
    for totalMovement in sortedKeys {
        writeln!( writer, "{}, {:?}", totalMovement, avalancheTotalMovementMap.get(&totalMovement).unwrap().len())?;
    }

    // print out the total movement value and the ids of the avalanches that have that total movement value
    // for (totalMovement, ids) in avalancheTotalMovementMap {
    //     println!( "Total Movement: {}", totalMovement);
    //     println!( "Avalanche Ids: {:?}", ids);
    // }

    // flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())

}

/**
 *  Experimental function to display the total magnitude of the avalanche
 * given as the total grains involved times the total movement of the avalanche
 */
pub fn displayAvalancheTotalMagnitude(avalanches: &Vec<Avalanche>, folder_path: String) -> io::Result<()> {

    // Create a file and wrap it in a BufWriter for efficient writing
    let file = File::create(folder_path + "/avalanche-total-magnitude.csv")?;
    let mut writer = BufWriter::new(file);

    // build a hashmap that will store a vector of ids of avalanches for each discrete total movement value within the avalanches vector.
    let mut avalancheTotalMagnitudeMap: HashMap<usize, Vec<u32>> = HashMap::new();

    // for each avalanche in the vector, add the avalanche id to the vector of ids for the total movement value
    for avalanche in avalanches {
        let totalMagnitude = avalanche.totalGrainsInvolved * avalanche.totalMovement;
        if avalancheTotalMagnitudeMap.contains_key(&totalMagnitude) {
            avalancheTotalMagnitudeMap.get_mut(&totalMagnitude).unwrap().push(avalanche.id);
        } else {
            avalancheTotalMagnitudeMap.insert(totalMagnitude, vec![avalanche.id]);
        }
    }

    // print out the total movement value the ids of the avalanches that have that total movement value in ascending order of movement value
    let mut sortedKeys: Vec<usize> = avalancheTotalMagnitudeMap.keys().cloned().collect();

    sortedKeys.sort();
    writeln!( writer, "Avalanche Magnitude, Number Avalanches")?;
    for totalMagnitude in sortedKeys {
        writeln!( writer, "{}, {:?}", totalMagnitude, avalancheTotalMagnitudeMap.get(&totalMagnitude).unwrap().len())?;
    }
    
    // flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())

}