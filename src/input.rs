use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::{Drag, Source, Wall};
use crate::constants::*;

fn screen_to_grid(x: f32, y: f32, screen_width: f32, screen_height: f32) -> Option<(u32, u32)> {
    let x = (x - (screen_width - (SIMULATION_WIDTH as u32 / PIXEL_SIZE) as f32) / 2.) as u32;
    let y = (y - (screen_height - (SIMULATION_HEIGHT as u32 / PIXEL_SIZE) as f32) / 2.) as u32;

    if x >= SIMULATION_WIDTH as u32 || y >= SIMULATION_HEIGHT as u32 {
        return None;
    }

    Some((x, y))
}

pub fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    sources: Query<(Entity, &Source), Without<Drag>>,
    mut drag_sources: Query<(Entity, &mut Source), With<Drag>>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = q_windows.single();
        if let Some(position) = window.cursor_position() {
            if let Some((x, y)) =
                screen_to_grid(position.x, position.y, window.width(), window.height())
            {
                for (entity, source) in sources.iter() {
                    if source.x.abs_diff(x as usize) <= 10 && source.y.abs_diff(y as usize) <= 10 {
                        commands.entity(entity).insert(Drag);
                    }
                }
            }
        }
    }
    if buttons.just_released(MouseButton::Left) {
        drag_sources.iter_mut().for_each(|(entity, _)| {
            commands.entity(entity).remove::<Drag>();
        });
    }
    if buttons.pressed(MouseButton::Left) && drag_sources.iter_mut().count() >= 1 {
        let window = q_windows.single();
        if let Some(position) = window.cursor_position() {
            if let Some((x, y)) =
                screen_to_grid(position.x, position.y, window.width(), window.height())
            {
                drag_sources.iter_mut().for_each(|(_, mut source)| {
                    source.x = x as usize;
                    source.y = y as usize;
                });
            }
        }
    }
    if buttons.pressed(MouseButton::Right) {
        let window = q_windows.single();
        if let Some(position) = window.cursor_position() {
            if let Some((x, y)) =
                screen_to_grid(position.x, position.y, window.width(), window.height())
            {
                //TODO: because of the brush size, the indices may be out of bounds
                //TODO: make bush size variable
                commands.spawn(Wall {
                    x: x as usize,
                    y: y as usize,
                });
                commands.spawn(Wall {
                    x: x as usize + 1,
                    y: y as usize,
                });
                commands.spawn(Wall {
                    x: x as usize - 1,
                    y: y as usize,
                });
                commands.spawn(Wall {
                    x: x as usize,
                    y: y as usize + 1,
                });
                commands.spawn(Wall {
                    x: x as usize + 1,
                    y: y as usize + 1,
                });
                commands.spawn(Wall {
                    x: x as usize,
                    y: y as usize - 1,
                });
                commands.spawn(Wall {
                    x: x as usize - 1,
                    y: y as usize - 1,
                });
                commands.spawn(Wall {
                    x: x as usize + 1,
                    y: y as usize - 1,
                });
                commands.spawn(Wall {
                    x: x as usize - 1,
                    y: y as usize + 1,
                });
            }
        }
    }

    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {}
}
