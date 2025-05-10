// src/fire_spread.rs

pub trait FireSpreadStrategy {
    fn spread(&self, x: usize, y: usize, size: usize) -> Vec<(usize, usize)>;
}

#[derive(Debug)]
pub struct MooreNeighborhood;
#[derive(Debug)]
pub struct VonNeumannNeighborhood;

impl FireSpreadStrategy for MooreNeighborhood {
    fn spread(&self, x: usize, y: usize, size: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                self.check_boundaries(x, y, dx, dy, size, &mut neighbors);
            }
        }
        neighbors
    }
}

impl FireSpreadStrategy for VonNeumannNeighborhood {
    fn spread(&self, x: usize, y: usize, size: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            self.check_boundaries(x, y, *dx, *dy, size, &mut neighbors);
        }
        neighbors
    }
}

trait NeighborhoodHelper {
    fn check_boundaries(
        &self,
        x: usize,
        y: usize,
        dx: i32,
        dy: i32,
        size: usize,
        neighbors: &mut Vec<(usize, usize)>
    );
}

impl<T> NeighborhoodHelper for T where T: FireSpreadStrategy {
    fn check_boundaries(
        &self,
        x: usize,
        y: usize,
        dx: i32,
        dy: i32,
        size: usize,
        neighbors: &mut Vec<(usize, usize)>
    ) {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx >= 0 && ny >= 0 && nx < size as i32 && ny < size as i32 {
            neighbors.push((nx as usize, ny as usize));
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moore_neighborhood_center() {
        let moore = MooreNeighborhood;
        let neighbors = moore.spread(1, 1, 3);
        let expected = vec![
            (0, 0), (0, 1), (0, 2),
            (1, 0),         (1, 2),
            (2, 0), (2, 1), (2, 2)
        ];
        assert_eq!(neighbors.len(), 8);
        assert_eq!(sorted(neighbors), sorted(expected));
    }

    #[test]
    fn test_moore_neighborhood_edge() {
        let moore = MooreNeighborhood;
        let neighbors = moore.spread(0, 1, 3);
        let expected = vec![
            (0, 0), (0, 2),
            (1, 0), (1, 1), (1, 2)
        ];
        assert_eq!(neighbors.len(), 5);
        assert_eq!(sorted(neighbors), sorted(expected));
    }

    #[test]
    fn test_moore_neighborhood_corner() {
        let moore = MooreNeighborhood;
        let neighbors = moore.spread(0, 0, 3);
        let expected = vec![
            (0, 1),
            (1, 0), (1, 1)
        ];
        assert_eq!(neighbors.len(), 3);
        assert_eq!(sorted(neighbors), sorted(expected));
    }

    #[test]
    fn test_von_neumann_center() {
        let vn = VonNeumannNeighborhood;
        let neighbors = vn.spread(1, 1, 3);
        let expected = vec![
            (0, 1),
            (1, 0), (1, 2),
            (2, 1)
        ];
        assert_eq!(neighbors.len(), 4);
        assert_eq!(sorted(neighbors), sorted(expected));
    }

    #[test]
    fn test_von_neumann_edge() {
        let vn = VonNeumannNeighborhood;
        let neighbors = vn.spread(0, 1, 3);
        let expected = vec![
            (1, 1), // From (-1, 0) is invalid
            (0, 0),
            (0, 2)
        ];
        assert_eq!(neighbors.len(), 3);
        assert_eq!(sorted(neighbors), sorted(expected));
    }

    #[test]
    fn test_von_neumann_corner() {
        let vn = VonNeumannNeighborhood;
        let neighbors = vn.spread(0, 0, 3);
        let expected = vec![
            (1, 0),  // From (0, -1) is invalid
            (0, 1)   // From (-1, 0) is invalid
        ];
        assert_eq!(neighbors.len(), 2);
        assert_eq!(sorted(neighbors), sorted(expected));
    }

    #[test]
    fn test_boundary_check() {
        let moore = MooreNeighborhood;
        let mut neighbors = Vec::new();
        moore.check_boundaries(2, 2, 1, 1, 3, &mut neighbors);
        assert_eq!(neighbors, vec![]);  // (3,3) is out of bounds
    }

    // Helper to sort coordinates for comparison
    fn sorted(mut vec: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        vec.sort();
        vec
    }
}