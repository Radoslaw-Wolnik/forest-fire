use crate::{config::Config, forest::Forest};
use crate::config::BurnPattern;
use crate::display::ForestDisplay;
use crate::fire_spread::FireSpreadStrategy;

#[derive(Debug)]
pub struct SimulationResults {
    pub average_burned: f64,
    pub min_burned: f64,
    pub max_burned: f64,
}

pub fn run_simulations(config: &Config) -> SimulationResults {
    let mut total_burned = 0;
    let mut total_trees = 0;
    let mut min_burned = f64::MAX;
    let mut max_burned = f64::MIN;

    let strategy: &dyn FireSpreadStrategy = match &config.burn_pattern {
        BurnPattern::Moore(s) => s,
        BurnPattern::VonNeumann(s) => s,
    };

    for _ in 0..config.simulations {

        let mut forest = Forest::new(
            config.size,
            config.density,
        );

        if forest.total_trees == 0 {
            continue;
            // or return - not sure
        }

        forest.ignite(forest.pick_random_tree());

        if config.graphics
        {
            let mut display = ForestDisplay::new();


            println!("forest at the beginning:");
            println!("{}", forest);
            println!("{}", forest.density());
            println!("\n");
            std::thread::sleep(std::time::Duration::from_millis(1000));

            display.prepare_animation();
            loop {
                display.render_frame(&forest);

                if forest.fire_spread(strategy){
                    break;
                }

                std::thread::sleep(std::time::Duration::from_millis(config.frame_delay_ms));
            }
            display.render_frame(&forest);

            display.tidy_up();

            println!("forest at the end:");
            println!("{}", forest);
            println!("\n");
        } else {
            loop {
                if forest.fire_spread(strategy) {
                    break;
                }
            }

        }

        let burned_percent = forest.burned_count as f64 / forest.total_trees as f64;

        total_burned += forest.burned_count;
        total_trees += forest.total_trees;

        if burned_percent < min_burned {
            min_burned = burned_percent;
        }
        if burned_percent > max_burned {
            max_burned = burned_percent;
        }
    }

    SimulationResults {
        average_burned: ( total_burned as f64 / total_trees as f64 ) / config.simulations as f64 * 100.0,
        min_burned: min_burned * 100.0,
        max_burned: max_burned * 100.0,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::fire_spread::MooreNeighborhood;

    #[test]
    fn test_single_simulation() {
        let config = Config {
            size: 10,
            density: 0.5,
            simulations: 1,
            burn_pattern: BurnPattern::Moore(MooreNeighborhood),
            graphics: false,
            frame_delay_ms: 50,
        };

        let results = run_simulations(&config);
        println!("burned {}", results.average_burned);
        assert!(results.average_burned >= 0.0 && results.average_burned <= 100.0);
    }

    #[test]
    fn test_zero_simulations() {
        let config = Config {
            size: 10,
            density: 0.5,
            simulations: 0,
            burn_pattern: BurnPattern::Moore(MooreNeighborhood),
            graphics: false,
            frame_delay_ms: 50,
        };

        let results = run_simulations(&config);
        assert!(results.average_burned.is_nan());
    }
}
