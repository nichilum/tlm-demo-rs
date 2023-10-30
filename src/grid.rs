use std::f32::consts::PI;

use array2d::{Array2D, Error};
use bevy::prelude::*;
use smallvec::SmallVec;

use crate::components::{Source, SourceType, Wall};
use crate::constants::*;

#[derive(Debug, Resource)]
pub struct Grid {
    /// full grid: [cur_bottom, cur_left, cur_top, cur_right, next_bottom, next_left, next_top, next_right, pressure]
    pub cur_bottom: Array2D<f32>,
    pub cur_left: Array2D<f32>,
    pub cur_top: Array2D<f32>,
    pub cur_right: Array2D<f32>,
    pub next_bottom: Array2D<f32>,
    pub next_left: Array2D<f32>,
    pub next_top: Array2D<f32>,
    pub next_right: Array2D<f32>,
    pub pressure: Array2D<f32>,
    /// A list of boundary nodes
    pub boundaries: Boundary,
}

#[derive(Debug, Default)]
pub struct Boundary {
    /// indecies of bottom boundary nodes
    bottom: SmallVec<[usize; SIMULATION_WIDTH]>,
    /// indecies of left boundary nodes
    left: SmallVec<[usize; SIMULATION_HEIGHT]>,
    /// indecies of top boundary nodes
    top: SmallVec<[usize; SIMULATION_WIDTH]>,
    /// indecies of right boundary nodes
    right: SmallVec<[usize; SIMULATION_HEIGHT]>,
}

impl Default for Grid {
    fn default() -> Self {
        let mut grid = Self {
            cur_bottom: Array2D::filled_with(0. as f32, SIMULATION_HEIGHT, SIMULATION_WIDTH),
            cur_left: Array2D::filled_with(0. as f32, SIMULATION_HEIGHT, SIMULATION_WIDTH),
            cur_top: Array2D::filled_with(0. as f32, SIMULATION_HEIGHT, SIMULATION_WIDTH),
            cur_right: Array2D::filled_with(0. as f32, SIMULATION_HEIGHT, SIMULATION_WIDTH),
            next_bottom: Array2D::filled_with(0. as f32, SIMULATION_HEIGHT, SIMULATION_WIDTH),
            next_left: Array2D::filled_with(0. as f32, SIMULATION_HEIGHT, SIMULATION_WIDTH),
            next_top: Array2D::filled_with(0. as f32, SIMULATION_HEIGHT, SIMULATION_WIDTH),
            next_right: Array2D::filled_with(0. as f32, SIMULATION_HEIGHT, SIMULATION_WIDTH),
            pressure: Array2D::filled_with(0. as f32, SIMULATION_HEIGHT, SIMULATION_WIDTH),
            boundaries: Default::default(),
        };
        grid.init_boundaries();
        grid
    }
}

impl Grid {
    /// moves next into current positions and calculates pressure
    fn update(&mut self) {
        for x in 0..SIMULATION_WIDTH {
            for y in 0..SIMULATION_HEIGHT {
                self.cur_bottom[(x, y)] = self.next_bottom[(x, y)];
                self.cur_left[(x, y)] = self.next_left[(x, y)];
                self.cur_top[(x, y)] = self.next_top[(x, y)];
                self.cur_right[(x, y)] = self.next_right[(x, y)];

                self.pressure[(x, y)] = 0.5
                    * (self.cur_bottom[(x, y)]
                        + self.cur_left[(x, y)]
                        + self.cur_top[(x, y)]
                        + self.cur_right[(x, y)]);
            }
        }
    }

    fn calc(&mut self) {
        for x in 1..SIMULATION_WIDTH - 1 {
            for y in 1..SIMULATION_HEIGHT - 1 {
                self.calc_cell(x, y);
            }
        }
    }

    fn calc_cell(&mut self, x: usize, y: usize) {
        self.next_bottom[(x, y)] = 0.5
            * (-self.cur_top[(x, y + 1)]
                + self.cur_right[(x - 1, y)]
                + self.cur_bottom[(x, y - 1)]
                + self.cur_left[(x + 1, y)]);
        self.next_left[(x, y)] = 0.5
            * (self.cur_top[(x, y + 1)] - self.cur_right[(x - 1, y)]
                + self.cur_bottom[(x, y - 1)]
                + self.cur_left[(x + 1, y)]);
        self.next_top[(x, y)] = 0.5
            * (self.cur_top[(x, y + 1)] + self.cur_right[(x - 1, y)] - self.cur_bottom[(x, y - 1)]
                + self.cur_left[(x + 1, y)]);
        self.next_right[(x, y)] = 0.5
            * (self.cur_top[(x, y + 1)] + self.cur_right[(x - 1, y)] + self.cur_bottom[(x, y - 1)]
                - self.cur_left[(x + 1, y)]);
    }

    fn apply_sources(&mut self, time: f32, sources: &Query<&Source>) {
        for source in sources.iter() {
            //? maybe needs to be optimized
            let calc = match source.r#type {
                SourceType::Sin => {
                    source.amplitude * (2. * PI * source.frequency * (time - source.phase)).sin()
                }
                SourceType::Gauss => {
                    Source::periodic_gaussian(time, source.frequency, source.amplitude, 5., 1.)
                }
            };

            self.next_bottom[(source.x, source.y)] = calc;
            self.next_left[(source.x, source.y)] = calc;
            self.next_top[(source.x, source.y)] = calc;
            self.next_right[(source.x, source.y)] = calc;
        }
    }

    fn apply_walls(&mut self, walls: &Query<&Wall>) {
        for wall in walls.iter() {
            self.next_bottom[(wall.x, wall.y)] = WALL_FAC * self.cur_top[(wall.x, wall.y + 1)];
            self.next_left[(wall.x, wall.y)] = WALL_FAC * self.cur_right[(wall.x - 1, wall.y)];
            self.next_top[(wall.x, wall.y)] = WALL_FAC * self.cur_bottom[(wall.x, wall.y - 1)];
            self.next_right[(wall.x, wall.y)] = WALL_FAC * self.cur_left[(wall.x + 1, wall.y)];
        }
    }

    fn apply_boundaries(&mut self) {}

    pub fn init_boundaries(&mut self) {
        // TOP
        for x in 0..SIMULATION_WIDTH {
            // self.boundaries.top.push(Grid::coords_to_index(x, 0))
        }
        // BOTTOM
        for x in 0..SIMULATION_WIDTH {
            // self.boundaries
            //     .bottom
            //     .push(Grid::coords_to_index(x, SIMULATION_HEIGHT - 1))
        }
        // LEFT
        for y in 0..SIMULATION_HEIGHT {
            // self.boundaries.left.push(Grid::coords_to_index(0, y))
        }
        // RIGHT
        for y in 0..SIMULATION_HEIGHT {
            // self.boundaries
            //     .right
            //     .push(Grid::coords_to_index(SIMULATION_WIDTH - 1, y))
        }
    }
}

pub fn calc_system(mut grid: ResMut<Grid>) {
    grid.calc();
}

pub fn apply_system(
    mut grid: ResMut<Grid>,
    time: Res<Time>,
    sources: Query<&Source>,
    walls: Query<&Wall>,
) {
    grid.apply_sources(time.elapsed_seconds(), &sources);
    grid.apply_walls(&walls);
    grid.apply_boundaries();
}

pub fn update_system(mut grid: ResMut<Grid>) {
    grid.update();
}
