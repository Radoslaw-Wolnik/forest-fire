mod config;
mod forest;
mod simulation;
mod fire_spread;
mod display;

use config::Config;
use simulation::run_simulations;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = match Config::new(&args) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let results = run_simulations(&config);

    println!("Simulation Results:");
    println!("-------------------");
    println!("Grid size: {}", config.size);
    println!("Tree density: {:.2}", config.density);
    println!("Burn pattern: {:?}", config.burn_pattern);
    println!("Min burned: {:.2}%", results.min_burned);
    println!("Max burned: {:.2}%", results.max_burned);
    println!("Average burned: {:.2}%", results.average_burned);

}