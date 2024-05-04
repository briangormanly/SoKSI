/**
 * Master Debug flag
 */
pub const DEBUG: bool = true;
pub const DEBUG_INIT: bool = false;
pub const DEBUG_AVALANCHE: bool = false;
pub const DEBUG_LOCATION: bool = false;
pub const DEBUG_LOCAL_NEIGHBORS: bool = false;
pub const DEBUG_GRAIN_IMPACT: bool = false;

pub const DEBUG_DISPLAY_PILE: bool = true;

// minimum value (multiplier) for the power-law distribution
// can be used to set a lower bound.
pub const X_MIN: f64 = 1.0;

// Power-law distribution parameters
// Default value for the main power-law distribution, only used if not overridden
// currently no calls are using the default value
pub const ALPHA_MAIN: f64 = 1.6;

// Power-law distribution parameters
// Amount of variation in the initial x,y coordinates of the grain as it enters the system (off center)
pub const ALPHA_LANDING: f64 = 1.4;
// Amount of additional energy added to the grains current energy in impact
pub const ALPHA_EXTRA_ENERGY: f64 = 0.8;
// Amount of additional grains to add to an avalanche in addition to the base size as determined by the avalanche method (see BASE_AVALANCHE_METHOD)
pub const ALPHA_AVALANCHE_SIZE: f64 = 1.2;
// Additional possible capacity of location
pub const ALPHA_LOCATION_EXTRA_CAPACITY: f64 = 2.2;
// Additional possible resilience of location
pub const ALPHA_LOCATION_EXTRA_RESILIENCE: f64 = 0.8;

// total allowed dimensions of the pile
pub const X_SIZE: i32 = 21;
pub const Y_SIZE: i32 = 21;
pub const Z_SIZE: i32 = 16;

// Physics constants
pub const TERMINAL_FREE_FALL_SPEED: usize = 3;
pub const BASE_RESILIENCE: usize = 3;
pub const BASE_CAPACITY: usize = 4;
// chose between size or percent for the avalanche size
pub const BASE_AVALANCHE_METHOD: usize = 2; // 1 = size, 2 = percent
// IF BASE_AVALANCHE_METHOD=1: base size of avalanche (will have result of ALPHA_AVALANCHE_SIZE added to it)
pub const BASE_AVALANCHE_SIZE: usize = 2;
// IF BASE_AVALANCHE_METHOD=2: base percent of grains in the avalanche from location (will have result of ALPHA_AVALANCHE_SIZE added to it)
pub const BASE_AVALANCHE_SIZE_PERCENT: f64 = 0.75;

// Total gains to be introduced into the system
pub const TOTAL_GRAINS: usize = 100000;
