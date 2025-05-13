use std::io::{stdout, Stdout, Write};
use crate::forest::{CellState, Forest};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct ForestDisplay {
    out: Stdout,
}

impl ForestDisplay {
    pub fn new() -> Self {
        ForestDisplay {
            out: stdout(),
        }
    }

    pub fn prepare_animation(&mut self) {
        // clear_screen();
        execute!(self.out, EnterAlternateScreen, Hide).unwrap();
    }

    pub fn render_frame(&mut self, forest: &Forest) {

        // let mut out = stdout();
        // clear_screen();
        execute!(self.out, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        // draw the grid
        for row in &forest.grid {
            for cell in row {
                let symbol = match cell {
                    CellState::Empty   => "  ",
                    CellState::Tree    => "🌲", // note the space
                    CellState::Burning => "🔥",
                    CellState::Burned  => "◼️",
                    // CellState::Lightning => "⚡️",
                };

                write!(self.out, "{}", symbol).unwrap(); // ?;
            }
            writeln!(self.out).unwrap(); // ?;
        }

        self.out.flush().unwrap(); // Ensure immediate flush
        // Ok();
    }

    pub fn tidy_up(&mut self) {
        execute!(self.out, LeaveAlternateScreen, Show).unwrap();
    }

}

