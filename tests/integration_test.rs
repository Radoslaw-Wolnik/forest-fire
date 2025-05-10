use project_forest_fire::{config::Config, simulation::run_simulations};

#[test]
fn test_full_workflow() {
    let args = [
        "program_name".to_string(),
        "-s".to_string(), "50".to_string(),
        "-d".to_string(), "0.6".to_string(),
        "-c".to_string(), "10".to_string(),
    ];

    let config = Config::new(&args).unwrap();
    let results = run_simulations(&config);

    assert!(results.average_burned > 0.0);
    assert!(results.optimal_density >= 0.0 && results.optimal_density <= 1.0);
}