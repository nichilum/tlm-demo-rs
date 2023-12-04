use std::f32::consts::PI;

use crate::constants::*;
use bevy::{prelude::*, transform::commands};

use crate::{
    constants::{SIMULATION_HEIGHT, SIMULATION_WIDTH},
    grid::Grid,
};

#[derive(Resource)]
pub struct GradientResource(pub colorgrad::Gradient);

impl GradientResource {
    pub fn with_custom() -> Self {
        Self(
            colorgrad::CustomGradient::new()
                .colors(&[
                    colorgrad::Color::from_rgba8(250, 172, 168, 255),
                    colorgrad::Color::from_rgba8(0, 0, 0, 255),
                    colorgrad::Color::from_rgba8(221, 214, 243, 255),
                ])
                .domain(&[-2.0, 2.0])
                .build()
                .unwrap(),
        )
    }
}

#[derive(Component)]
pub struct Drag;

#[derive(Debug, Default, Component)]
/// A sound source on the grid
pub struct Source {
    pub x: u32,
    pub y: u32,
    /// phase shift of the function in degrees
    pub phase: f32,
    /// frequency of the function (in Hz)
    pub frequency: f32,
    /// amplitude of the function (currently unitless)
    pub amplitude: f32,
    /// type of the source
    pub r#type: SourceType,
}

#[derive(Debug, Default, PartialEq)]
pub enum SourceType {
    #[default]
    Sin,
    Gauss,
}

impl Source {
    pub fn new(
        x: u32,
        y: u32,
        amplitude: f32,
        phase: f32,
        frequency: f32,
        r#type: SourceType,
    ) -> Self {
        Self {
            x,
            y,
            phase,
            frequency,
            amplitude,
            r#type,
        }
    }

    /// generated by chat gpt, not sure if it's correct
    pub fn periodic_gaussian(x: f32, period: f32, amplitude: f32, mean: f32, variance: f32) -> f32 {
        // Ensure x is within the periodic domain [0, period)
        let x = (x % period + period) % period;

        // Calculate the Gaussian function value
        let exp_term = (-0.5 * ((x - mean) / variance).powi(2)).exp();
        let scaling_factor = 1.0 / (variance * (2.0 * PI).sqrt());

        amplitude * scaling_factor * exp_term
    }

    pub fn spawn_initial_sources(mut commands: Commands) {
        commands.spawn(Source::new(
            (SIMULATION_WIDTH + 2 * E_AL) / 2,
            (SIMULATION_HEIGHT + 2 * E_AL) / 2,
            10.,
            0.0,
            10000.0,
            SourceType::Sin,
        ));
        commands.spawn(Source::new(
            (SIMULATION_WIDTH + 2 * E_AL) / 3,
            (SIMULATION_HEIGHT + 2 * E_AL) / 3,
            10.,
            0.0,
            10000.0,
            SourceType::Sin,
        ));
    }
}

#[derive(Debug, Component)]
/// A wall component containing the index of the corresponding cell in the grid
pub struct Wall(pub usize);

#[derive(Default, Resource)]
pub struct GameTicks {
    pub ticks_since_start: u64,
}

#[derive(Debug, Default, Component)]
/// A sound source on the grid
pub struct Microphone {
    pub x: u32,
    pub y: u32,
    pub record: Vec<[f64; 2]>,
}

impl Microphone {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            record: vec![],
        }
    }

    pub fn spawn_initial_microphones(mut commands: Commands) {
        commands.spawn(Microphone::new(250, 250));
        commands.spawn(Microphone::new(100, 100));
        commands.spawn(Microphone::new(650, 650));
    }
}
