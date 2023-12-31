use std::f32::consts::PI;

use bevy::prelude::*;

use crate::grid::plugin::ComponentIDs;
use crate::math::constants::*;

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
    pub source_type: SourceType,
    pub id: usize,
}

#[derive(Debug, Default, PartialEq)]
pub enum SourceType {
    #[default]
    Sin,
    Gauss,
    WhiteNoise,
}

impl Source {
    pub fn new(
        x: u32,
        y: u32,
        amplitude: f32,
        phase: f32,
        frequency: f32,
        source_type: SourceType,
        id: usize,
    ) -> Self {
        Self {
            x,
            y,
            phase,
            frequency,
            amplitude,
            source_type,
            id,
        }
    }

    pub fn calc(&self, time: f32) -> f32 {
        match self.source_type {
            SourceType::Sin => self.sin(time),
            SourceType::Gauss => self.periodic_gaussian(time, 4., 0., 0.45),
            SourceType::WhiteNoise => rand::random::<f32>() * self.amplitude,
        }
    }

    fn sin(&self, time: f32) -> f32 {
        self.amplitude * (2. * PI * self.frequency * (time - self.phase * PI / 180.)).sin()
    }

    fn periodic_gaussian(&self, time: f32, period: f32, mean: f32, standard_deviation: f32) -> f32 {
        // Ensure x is within the periodic domain (-period/2 ; period/2)
        let x = ((2. * PI * self.frequency * time) % period) - (period / 2.);

        // Calculate the Gaussian function value
        let exp_term = (-0.5 * ((x - mean) / standard_deviation).powi(2)).exp();
        let scaling_factor = 1.0 / (standard_deviation * (2.0 * PI).sqrt());

        self.amplitude * scaling_factor * exp_term
    }

    pub fn spawn_initial_sources(mut commands: Commands, mut component_ids: ResMut<ComponentIDs>) {
        commands.spawn(Source::new(
            (SIMULATION_WIDTH + 2 * E_AL) / 2,
            (SIMULATION_HEIGHT + 2 * E_AL) / 2,
            10.,
            0.0,
            10000.0,
            SourceType::Sin,
            component_ids.get_current_source_id(),
        ));
        commands.spawn(Source::new(
            (SIMULATION_WIDTH + 2 * E_AL) / 3,
            (SIMULATION_HEIGHT + 2 * E_AL) / 3,
            10.,
            0.0,
            10000.0,
            SourceType::Sin,
            component_ids.get_current_source_id(),
        ));
    }
}
