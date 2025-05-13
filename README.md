# Forest Fire Simulation

A Rust implementation of a forest fire spread simulation based on a grid model. This project simulates the spread of fire after a lightning strike in a procedurally generated forest and calculates statistics such as the percentage of burned trees.

---

## Features
- **Grid-based forest model** with customizable dimensions.
- **Random tree generation** with adjustable density.
- **Fire spread simulation** using Moore neighborhood (8-directional spread).
- **CLI visualization** of the forest before/after and during the fire.
- **Statistical analysis** of burned area percentage.
- **Multiple simulation runs** to calculate min max and avg.
- **Configurable parameters** (grid size, tree spawn probability, etc.).
- *Optional extensions*: Wind direction, tree age/resistance river and rock tiles.

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

```text
USAGE:
    forest_fire_sim [OPTIONS]

OPTIONS:
    -s, --size <size>              Grid size (default: 20)
    -d, --density <density>        Tree density between 0.0 and 1.0 (default: 0.6)
    -c, --simulations <count>      Number of simulations to run (default: 1)
    -b, --burn-pattern <pattern>   Burn pattern: 'moore' or 'vonneumann' (default: moore)
    -g-off, --graphics-off         Disable graphical output (default: enabled)
    -fd, --frame-delay <ms>        Frame delay in milliseconds (1 to 10000, default: 50)
    -h, --help                     Print this help message
```

#### 🔍 Description:

* `size`: Sets the width and height of the grid. For example, `-s 100` creates a 100x100 grid.
* `density`: A float between 0.0 and 1.0 indicating how densely the trees are populated.
* `simulations`: Number of independent simulation runs.
* `burn-pattern`: Choose how fire spreads:

    * `moore` (default) – fire spreads in 8 directions (N, NE, E, SE, S, SW, W, NW)
    * `vonneumann` – fire spreads in 4 directions (N, E, S, W)
* `graphics-off`: Disables graphical visualization of the simulation.
* `frame-delay`: Delay in milliseconds between animation frames when graphics is enabled.

#### Example:
```bash
cargo run -- --size 30 --density 0.4 --simulations 1 
```
This simulates a 30x30 forest with 40% tree density across 1 trial with graphical representation.

```bash
cargo run -- -g-off -c 100 -s 100 -d 0.42
```
This simulates a 100x100 forest with 42% tree density across 100 trials without graphic (for speed).


### Sample Output
```
Initial Forest:
🌲  🌲
  🌲🌲🌲
🌲🔥🌲
    🌲  🌲
        🌲


Burned Forest:
◼️  ◼️
  ◼️◼️◼️
◼️◼️◼️
    ◼️  🌲
        🌲


Simulation Results:
-------------------
Grid size: 5
Tree density: 0.45
Burn pattern: Moore(MooreNeighborhood)
Min burned: 81.82%
Max burned: 81.82%
Average burned: 81.82%

```

---


## Implementation Details
### Grid Representation
- 2D Vec<> of `CellState` enums:
  ```rust
  enum CellState { Empty, Tree, Burning, Burned }
  ```
- Initialized with random trees based on spawn probability.

### Fire Spread Algorithm
1. Random tree is chosen to be ignited.
2. Burning trees spread fire to adjacent cells in each iteration.
3. Simulation continues until no burning trees remain.




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