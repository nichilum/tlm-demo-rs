use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;

use crate::components::{GradientResource, Wall};
use crate::grid::Grid;

pub fn draw_pixels(
    mut pb: QueryPixelBuffer,
    grid: Res<Grid>,
    gradient: Res<GradientResource>,
    walls: Query<&Wall>,
) {
    let mut frame = pb.frame();
    frame.per_pixel_par(|coords, _| {
        let p = grid.pressure[(coords.x as usize, coords.y as usize)];
        let color = gradient.0.at((p) as f64);
        Pixel {
            r: (color.r * 255.) as u8,
            g: (color.g * 255.) as u8,
            b: (color.b * 255.) as u8,
            a: 255,
        }
    });
    // Walls
    for wall in walls.iter() {
        //TODO: handle result
        let _ = frame.set(
            UVec2::new(wall.x as u32, wall.y as u32),
            Pixel {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        );
    }
}
