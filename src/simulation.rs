use crate::{config::Config, forest::Forest};
use crate::config::BurnPattern;
use crate::fire_spread::FireSpreadStrategy;

#[derive(Debug)]
pub struct SimulationResults {
    pub average_burned: f64,
    pub optimal_density: f64,
}

pub fn run_simulations(config: &Config) -> SimulationResults {
    let mut total_burned = 0.0;
    let mut optimal_density = 0.0;
    let mut min_burned = f64::MAX;

    let strategy: &dyn FireSpreadStrategy = match &config.burn_pattern {
        BurnPattern::Moore(s) => s,
        BurnPattern::VonNeumann(s) => s,
    };

    for _ in 0..config.simulations {
        let mut forest = Forest::new(
            config.size,
            config.density,
        );


        let burned_percent = forest.simulate_fire(strategy, None) * 100.0;
        total_burned += burned_percent;

        if burned_percent < min_burned {
            min_burned = burned_percent;
            optimal_density = config.density;
        }
    }

    SimulationResults {
        average_burned: total_burned / config.simulations as f64,
        optimal_density,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::fire_spread::MooreNeighborhood;

    #[test]
    fn test_single_simulation() {
        let config = Config {
            size: 10,
            density: 0.5,
            simulations: 1,
            burn_pattern: BurnPattern::Moore(MooreNeighborhood),
            display_grid: false,
        };

        let results = run_simulations(&config);
        println!("burned {}", results.average_burned);
        assert!(results.average_burned >= 0.0 && results.average_burned <= 100.0);
    }

    #[test]
    fn test_optimal_density_tracking() {
        let config = Config {
            size: 10,
            density: 0.3,
            simulations: 5,
            burn_pattern: BurnPattern::Moore(MooreNeighborhood),
            display_grid: false,
        };

        let results = run_simulations(&config);
        assert_relative_eq!(results.optimal_density, 0.3);
    }

    #[test]
    fn test_zero_simulations() {
        let config = Config {
            size: 10,
            density: 0.5,
            simulations: 0,
            burn_pattern: BurnPattern::Moore(MooreNeighborhood),
            display_grid: false,
        };

        let results = run_simulations(&config);
        assert!(results.average_burned.is_nan());
    }
}
