mod config;
mod forest;
mod simulation;
mod fire_spread;
mod display;

use std::time::Instant;
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
    let start = Instant::now();
    if config.auto_sweep {
        // Handle missing step parameter
        let step = config.sweep_step.unwrap_or_else(|| {
            eprintln!("Auto sweep requires a step size. Using default 0.1");
            0.1
        });

        // Loop through densities from 0.0 to 1.0 in increments
        let mut density = 0.01;
        while density <= 1.0 {
            // Create a new config with current density
            let sweep_config = Config {
                density,
                ..config.clone()
            };

            let results = run_simulations(&sweep_config);

            // Print density and result for data collection
            println!("density: {:.2}, average burned: {:.5} across {} simulaitons", density, results.average_burned, config.simulations);

            // Increment density, handling floating-point precision
            density = (density + step).min(1.0);

            // Break if we've reached 1.0 or step is too small
            if (density - 1.0).abs() < f64::EPSILON || step < f64::EPSILON {
                break;
            }
        }
        let elapsed = start.elapsed();
        println!("Elapsed time: {:.2?}", elapsed);
        std::process::exit(0); // Exit after sweep completes
    }

    let results = run_simulations(&config);

    if config.quiet {
        println!("{:.5}", results.average_burned);
    } else {
        println!("Simulation Results:");
        println!("-------------------");
        println!("Grid size: {}", config.size);
        println!("Tree density: {:.2}", config.density);
        println!("Burn pattern: {:?}", config.burn_pattern);
        println!("Min burned: {:.2}%", results.min_burned);
        println!("Max burned: {:.2}%", results.max_burned);
        println!("Average burned: {:.2}%", results.average_burned);
        let elapsed = start.elapsed();
        println!("Elapsed time: {:.2?}", elapsed);
    }
}