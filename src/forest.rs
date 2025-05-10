use std::collections::VecDeque;
use std::fmt;
use rand::Rng;
use crate::fire_spread::FireSpreadStrategy;


// Cell states
#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Tree,
    Burning,
    Burned,
}

pub struct Forest {
    grid: Vec<Vec<CellState>>,
    size: usize,
}

impl Forest {
    pub fn new(size: usize, density: f64) -> Self {
        let mut rng = rand::rng();
        let grid = (0..size)
            .map(|_| (0..size)
                .map(|_| if rng.random_bool(density) {
                    CellState::Tree
                } else {
                    CellState::Empty
                })
                .collect())
            .collect();

        Forest { grid, size }
    }

    pub fn simulate_fire(
        &mut self,
        strategy: &dyn FireSpreadStrategy,
        start_pos: Option<(usize, usize)>,
    ) -> f64 {
        let total_trees = self.count_trees();
        if total_trees == 0 {
            return 0.0;
        }

        let mut fire_front = VecDeque::new();
        let (x, y) = start_pos.unwrap_or_else(|| self.random_strike());

        if self.grid[x][y] == CellState::Tree {
            self.grid[x][y] = CellState::Burning;
            fire_front.push_back((x, y));
        }

        let mut total_burned = 0;

        while !fire_front.is_empty() {
            let current_burning = fire_front.drain(..).collect::<Vec<_>>();

            for (x, y) in current_burning {
                total_burned += 1;
                self.grid[x][y] = CellState::Burned;

                for (nx, ny) in strategy.spread(x, y, self.size) {
                    if self.grid[nx][ny] == CellState::Tree {
                        self.grid[nx][ny] = CellState::Burning;
                        fire_front.push_back((nx, ny));
                    }
                }
            }
        }

        total_burned as f64 / total_trees as f64
    }

    fn count_trees(&self) -> usize {
        self.grid.iter()
            .flatten()
            .filter(|&&cell| cell == CellState::Tree)
            .count()
    }

    fn random_strike(&self) -> (usize, usize) {
        let mut rng = rand::rng();
        let (x, y) = (rng.random_range(0..self.size), rng.random_range(0..self.size));
        (x, y)
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", match cell {
                    CellState::Empty => " ",
                    CellState::Tree => "🌲",
                    CellState::Burning => "🔥",
                    CellState::Burned => "◼️",
                })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::fire_spread::{MooreNeighborhood, VonNeumannNeighborhood};

    #[test]
    fn test_empty_forest() {
        let mut forest = Forest::new(10, 0.0);
        let burned = forest.simulate_fire(&MooreNeighborhood, None);
        assert_relative_eq!(burned, 0.0);
    }

    #[test]
    fn test_full_burn_moore() {
        let mut forest = Forest::new(3, 1.0, );
        let burned = forest.simulate_fire(&MooreNeighborhood, Some((1, 1)));
        assert_relative_eq!(burned, 1.0);
    }

    #[test]
    fn test_von_neumann_spread() {
        let mut forest = Forest {
            grid: vec![
                vec![CellState::Tree; 3],
                vec![CellState::Tree, CellState::Tree, CellState::Tree],
                vec![CellState::Tree, CellState::Tree, CellState::Tree],
            ],
            size: 3
        };

        // Manually set center cell on fire
        // forest.grid[1][1] = CellState::Burning;

        let burned = forest.simulate_fire(&MooreNeighborhood, Some((1, 1)));
        assert_relative_eq!(burned, 1.0);
    }

    #[test]
    fn test_edge_ignition() {
        let mut forest = Forest::new(3, 1.0);
        let burned = forest.simulate_fire(&VonNeumannNeighborhood, Some((0, 0)));
        assert_relative_eq!(burned, 1.0);
    }

    #[test]
    fn test_tree_count() {
        let forest = Forest::new(100, 0.5);
        let tree_count = forest.count_trees();
        assert!(tree_count > 4000 && tree_count < 6000);
    }
}