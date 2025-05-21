use std::collections::VecDeque;
use std::fmt;
use rand::Rng;
use rand::seq::SliceRandom;
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
    pub size: usize,
    pub grid: Vec<Vec<CellState>>,
    fire_front: VecDeque<(usize, usize)>,
    pub burned_count: usize,
    pub total_trees: usize,
}

impl Forest {
    pub fn new(size: usize, density: f64) -> Self {
        let total_cells = size * size;
        // compute exact number of trees (round to nearest)
        let target_trees = (density * total_cells as f64).round() as usize;

        // start with an all-empty grid
        let mut grid = vec![vec![CellState::Empty; size]; size];

        let mut indices: Vec<(usize, usize)> = (0..size)
            .flat_map(|x| (0..size).map(move |y| (x, y)))
            .collect();

        let mut rng = rand::rng();
        indices.shuffle(&mut rng);

        for &(x, y) in indices.iter().take(target_trees) {
            grid[x][y] = CellState::Tree;
        }

        // track the count:
        let total_trees = target_trees;

        Forest {
            size,
            grid,
            fire_front: VecDeque::new(),
            burned_count: 0,
            total_trees,
        }
    }

    pub fn ignite(&mut self, pos: Option<(usize, usize)>) -> bool {
        let (x, y) = pos.unwrap_or_else(|| self.random_strike());
        if self.grid[x][y] == CellState::Tree {
            self.grid[x][y] = CellState::Burning;
            self.fire_front.push_back((x, y));
        }
        !self.fire_front.is_empty()
    }

    pub fn pick_random_tree(&self) -> Option<(usize, usize)> {
        let mut rng = rand::rng();
        let mut selected: Option<(usize, usize)> = None;
        let mut count = 0;

        for (x, row) in self.grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if *cell == CellState::Tree {
                    count += 1;
                    // Replace with probability 1/count
                    if rng.random_range(0..count) == 0 {
                        selected = Some((x, y));
                    }
                }
            }
        }

        selected
    }


    pub fn fire_spread(&mut self, strategy: &dyn FireSpreadStrategy) -> bool {
        if self.fire_front.is_empty() {
            return false;
        }

        let current_burning = self.fire_front.drain(..).collect::<Vec<_>>();

        for (x, y) in current_burning {
            self.burned_count += 1;
            self.grid[x][y] = CellState::Burned;

            for (nx, ny) in strategy.spread(x, y, self.size) {
                if nx < self.size && ny < self.size && self.grid[nx][ny] == CellState::Tree {
                    self.grid[nx][ny] = CellState::Burning;
                    self.fire_front.push_back((nx, ny));
                }
            }
        }

        self.fire_front.is_empty()
    }

    fn random_strike(&self) -> (usize, usize) {
        let mut rng = rand::rng();
        let (x, y) = (rng.random_range(0..self.size), rng.random_range(0..self.size));
        (x, y)
    }

    pub fn density(&self) -> f64 {
         self.total_trees as f64 / (self.size * self.size) as f64
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", match cell {
                    CellState::Empty => "  ", // 🟩❇️🌾🌿🌻
                    CellState::Tree => "🌲", // 🌳🌴
                    CellState::Burning => "🔥",
                    CellState::Burned => "◼️", // 🪨
                    // CellState::Lightning => "⚡️",
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

    #[test]
    fn test_tree_count() {
        let forest:Forest = Forest::new(100, 0.5);
        let tree_count = forest.total_trees;
        assert!(tree_count > 4000 && tree_count < 6000);
    }
}