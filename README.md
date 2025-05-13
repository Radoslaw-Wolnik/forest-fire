# Forest Fire Simulation

A Rust implementation of a forest fire spread simulation based on a grid model. This project simulates the spread of fire after a lightning strike in a procedurally generated forest and calculates statistics such as the percentage of burned trees.

---

## Features
- **Grid-based forest model** with customizable dimensions.
- **Random tree generation** with adjustable density.
- **Fire spread simulation** using Moore neighborhood (8-directional spread).
- **CLI visualization** of the forest before/after the fire.
- **Statistical analysis** of burned area percentage.
- **Multiple simulation runs** to calculate optimal tree density for minimal fire loss.
- **Configurable parameters** (grid size, tree spawn probability, etc.).
- *Optional extensions*: Wind direction, tree age/resistance, graphical interface.

---

## Installation
1. Ensure [Rust](https://www.rust-lang.org/tools/install) and Cargo are installed.
2. Clone the repository:
   ```bash
   git clone https://github.com/radoslawwolnik/project-forest-fire.git
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

---

## Usage
Run the simulation with optional parameters:
```bash
cargo run -- [GRID_SIZE] [TREE_PROBABILITY] [SIMULATION_COUNT]
```
Example:
```bash
cargo run -- --size 30 --density 0.4 --simulations 1
```
This simulates a 50x50 forest with 60% tree density across 1000 trials.

### Sample Output
```
Initial Forest (5x5):
. T . T T
T . T . T
. T T T .
T . . T .
T T . T .

Burned Forest:
. X . X X
X . X . X
. X X X .
X . . X .
X X . X .

Burned: 68.0% of trees
Optimal density: ~58.2% (minimizes fire loss)
```

---

## Configuration
Adjust these parameters via CLI arguments or a config file (TODO):
- `GRID_SIZE`: Forest dimensions (N x N)
- `TREE_PROBABILITY`: Chance for a cell to contain a tree (0.0-1.0)
- `FIRE_SPREAD_RULE`: Moore (8-directional) or von Neumann (4-directional) neighborhood

---

## Implementation Details
### Grid Representation
- 2D array of `CellState` enums:
  ```rust
  enum CellState { Empty, Tree, Burning, Burned }
  ```
- Initialized with random trees based on spawn probability.

### Fire Spread Algorithm
1. Random lightning strike ignites a tree (if present).
2. Burning trees spread fire to adjacent cells in each iteration.
3. Simulation continues until no burning trees remain.

### Optimization Analysis
- Runs multiple simulations to determine tree density that maximizes:
  ```
  Score = (Forestation %) - (Average Burned %)
  ```


---

## License
MIT
```
MIT License

Copyright (c) 2025 Radoslaw

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```