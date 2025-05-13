// use std::env;
use crate::fire_spread::{MooreNeighborhood, VonNeumannNeighborhood};

// Burn pattern options
#[derive(Debug)]
pub enum BurnPattern {
    Moore(MooreNeighborhood),      // 8-directional
    VonNeumann(VonNeumannNeighborhood), // 4-directional
}


// Configuration structure
pub struct Config {
    pub size: usize,
    pub density: f64,
    pub simulations: usize,
    pub burn_pattern: BurnPattern,
    pub graphics: bool,
    pub frame_delay_ms: u64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, String> {
        let mut config = Config {
            size: 20,
            density: 0.6,
            simulations: 1,
            burn_pattern: BurnPattern::Moore(MooreNeighborhood),
            graphics: true,
            frame_delay_ms: 50,
        };

        // .skip(1) to ignore the program name
        let mut args_iter = args.iter().skip(1);

        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-s" | "--size" => {
                    // parse_arg will take <T> based on the type the variable is
                    // so instead of doing:
                    // config.size = parse_arg::<usize>(&mut args_iter, "size")?;
                    // we can just do:
                    config.size = parse_arg(&mut args_iter, "size")?;
                    // and the compiler will tell the parse_arg that it should take <usize>
                }
                "-d" | "--density" => {
                    config.density = parse_arg(&mut args_iter, "density")?;
                    if !(0.0..=1.0).contains(&config.density) {
                        return Err("Density must be between 0.0 and 1.0".into());
                    }
                }
                "-c" | "--simulations" => {
                    config.simulations = parse_arg(&mut args_iter, "simulations")?;
                }
                "-b" | "--burn-pattern" => {
                    let pattern = parse_arg::<String>(&mut args_iter, "burn-pattern")?;
                    config.burn_pattern = match pattern.to_lowercase().as_str() {
                        "moore" => BurnPattern::Moore(MooreNeighborhood),
                        "vonneumann" => BurnPattern::VonNeumann(VonNeumannNeighborhood),
                        _ => return Err("Invalid burn pattern. Use 'moore' or 'vonneumann'".into()),
                    };
                }
                "-g-off" | "--graphics-off" => {
                    config.graphics = false;
                }
                "-fd" | "--frame-delay" => {
                    config.frame_delay_ms = parse_arg(&mut args_iter, "frame-delay")?;
                    if !(1..=10_000).contains(&config.frame_delay_ms) {
                        return Err("Frame delay must be between 1 and 10 000 ms".into());
                    }
                }

                "-h" | "--help" => {
                    return Err(
                    "Usage: forest_fire_sim [OPTIONS]

                    Options:
                        -s, --size <size>              Grid size (default: 20)
                        -d, --density <density>        Tree density between 0.0 and 1.0 (default: 0.6)
                        -c, --simulations <count>      Number of simulations to run (default: 1)
                        -b, --burn-pattern <pattern>   Burn pattern: 'moore' or 'vonneumann' (default: moore)
                        -g-off, --graphics-off         Disable graphical output (default: enabled)
                        -fd, --frame-delay <ms>        Frame delay in milliseconds (1 to 10000, default: 50)
                        -h, --help                     Print this help message"
                    .into()
                    );
                }
                _ => return Err(format!("Unknown argument: {}", arg)),
            }
        }

        Ok(config)
    }
}

/// Helper function: takes an iterator over arguments and a name for error messages.
///
/// It advances the iterator to get the next element and attempts to parse it into T.
fn parse_arg<T: std::str::FromStr>(
    args_iter: &mut dyn Iterator<Item = &String>,
    arg_name: &str,
) -> Result<T, String> { // Result<T, &'static str>
    args_iter.next()
        .ok_or_else(|| format!("Missing value for {}", arg_name))?
        .parse::<T>()// parse into the requested type
        .map_err(|_| format!("Invalid value for {}", arg_name))
}


#[cfg(test)]
mod tests {
    use super::*;

    fn mock_args(args: &[&str]) -> Vec<String> {
        std::iter::once("program_name".to_string())
            .chain(args.iter().map(|s| s.to_string()))
            .collect()
    }

    #[test]
    fn test_default_config() {
        let args = mock_args(&[]);
        let config = Config::new(&args).unwrap();
        assert_eq!(config.size, 50);
        assert_eq!(config.density, 0.6);
        assert_eq!(config.simulations, 100);
        assert!(matches!(config.burn_pattern, BurnPattern::Moore(MooreNeighborhood)));
        assert!(!config.graphics);
    }

    #[test]
    fn test_full_config() {
        let args = mock_args(&[
            "-s", "100",
            "-d", "0.7",
            "-c", "500",
            "-b", "vonneumann",
            "-g"
        ]);
        let config = Config::new(&args).unwrap();

        assert_eq!(config.size, 100);
        assert!((config.density - 0.7).abs() < f64::EPSILON);
        assert_eq!(config.simulations, 500);
        assert!(matches!(config.burn_pattern, BurnPattern::VonNeumann(VonNeumannNeighborhood)));
        assert!(config.graphics);
    }

    #[test]
    fn test_invalid_density() {
        let args = mock_args(&["-d", "1.5"]);
        let result = Config::new(&args);
        assert!(result.is_err());
    }
}