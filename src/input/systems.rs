use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::microphone::Microphone;
use crate::components::source::{Source, SourceType};
use crate::components::states::{Move, Selected};
use crate::components::wall::{CircWall, RectWall, WResize, Wall};
use crate::events::{Load, Reset, Save, UpdateWalls};
use crate::math::transformations::{screen_to_grid, screen_to_nearest_grid};
use crate::simulation::plugin::ComponentIDs;
use crate::ui::state::{ClipboardBuffer, PlaceType, ToolType, UiState};

/// This system handles the copy and paste functionality
pub fn copy_paste_system(
    keys: Res<ButtonInput<KeyCode>>,
    selected: Query<Entity, With<Selected>>,
    mut clipboard: ResMut<ClipboardBuffer>,
    mut ids: ResMut<ComponentIDs>,
    mut commands: Commands,
    sources: Query<(Entity, &Source), With<Selected>>,
    rect_walls: Query<(Entity, &RectWall), With<Selected>>,
    circ_walls: Query<(Entity, &CircWall), With<Selected>>,
    mics: Query<(Entity, &Microphone), With<Selected>>,
) {
    #[cfg(not(target_os = "macos"))]
    let ctrl = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    #[cfg(target_os = "macos")]
    let ctrl = keys.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);

    if ctrl && keys.just_pressed(KeyCode::KeyC) {
        if let Some(entity) = selected.iter().next() {
            clipboard.copy(entity);
        }
    }

    if ctrl && keys.just_pressed(KeyCode::KeyV) {
        if let Some(entity) = clipboard.get() {
            if let Ok((_, source)) = sources.get(entity) {
                let mut source = *source;
                source.id = ids.get_new_source_id();
                commands.spawn(source);
            } else if let Ok((_, rect_wall)) = rect_walls.get(entity) {
                let mut rect_wall = *rect_wall;
                rect_wall.id = ids.get_new_wall_id();
                rect_wall.set_center(rect_wall.get_center().x + 5, rect_wall.get_center().y + 5);
                commands.spawn(rect_wall);
            } else if let Ok((_, circ_wall)) = circ_walls.get(entity) {
                let mut circ_wall = *circ_wall;
                circ_wall.id = ids.get_new_wall_id();
                circ_wall.set_center(circ_wall.get_center().x + 5, circ_wall.get_center().y + 5);
                commands.spawn(circ_wall);
            } else if let Ok((_, mic)) = mics.get(entity) {
                let mut mic = mic.clone();
                mic.id = ids.get_new_mic_id();
                commands.spawn(mic);
            }
        }
    }
}

type RectWalls<'w, 's> = Query<'w, 's, (Entity, &'static RectWall)>;
type CircWalls<'w, 's> = Query<'w, 's, (Entity, &'static CircWall)>;
type Mics<'w, 's> = Query<'w, 's, (Entity, &'static Microphone)>;
type Sources<'w, 's> = Query<'w, 's, (Entity, &'static Source)>;

type ResizeRectWalls<'w, 's> =
    Query<'w, 's, (Entity, &'static WResize, &'static mut RectWall), With<WResize>>;
type ResizeCircWalls<'w, 's> =
    Query<'w, 's, (Entity, &'static WResize, &'static mut CircWall), With<WResize>>;

type MoveRectWalls<'w, 's> = Query<'w, 's, (Entity, &'static mut RectWall), With<Move>>;
type MoveCircWalls<'w, 's> = Query<'w, 's, (Entity, &'static mut CircWall), With<Move>>;
type MoveMics<'w, 's> = Query<'w, 's, (Entity, &'static mut Microphone), With<Move>>;
type MoveSources<'w, 's> = Query<'w, 's, (Entity, &'static mut Source), With<Move>>;

type UnselectedRectWalls<'w, 's> = Query<'w, 's, (Entity, &'static RectWall), Without<Selected>>;
type UnselectedCircWalls<'w, 's> = Query<'w, 's, (Entity, &'static CircWall), Without<Selected>>;
type UnselectedMics<'w, 's> = Query<'w, 's, (Entity, &'static Microphone), Without<Selected>>;
type UnselectedSources<'w, 's> = Query<'w, 's, (Entity, &'static Source), Without<Selected>>;

pub fn button_input(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut wall_update_ev: EventWriter<UpdateWalls>,
    mut component_ids: ResMut<ComponentIDs>,
    mut ui_state: ResMut<UiState>,
    mut selected: Query<Entity, With<Selected>>,
    // Param Sets
    mut source_set: ParamSet<(Sources, UnselectedSources, MoveSources)>,
    mut mic_set: ParamSet<(Mics, UnselectedMics, MoveMics)>,
    mut rect_wall_set: ParamSet<(
        RectWalls,
        UnselectedRectWalls,
        MoveRectWalls,
        ResizeRectWalls,
    )>,
    mut circ_wall_set: ParamSet<(
        CircWalls,
        UnselectedCircWalls,
        MoveCircWalls,
        ResizeCircWalls,
    )>,
) {
    #[cfg(not(target_os = "macos"))]
    let ctrl = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    #[cfg(target_os = "macos")]
    let ctrl = keys.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);

    if mouse_buttons.just_pressed(MouseButton::Left)
        && ui_state.tools_enabled
        && ui_state.tool_use_enabled
    {
        let window = q_windows.single();
        selected.iter_mut().for_each(|entity| {
            commands.entity(entity).remove::<Selected>();
        });
        if let Some(position) = window.cursor_position() {
            match ui_state.current_tool {
                ToolType::Select => {
                    if let Some((x, y)) =
                        screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                    {
                        // This should only allow for one object to be selected
                        'outer: {
                            for (entity, source) in source_set.p1().iter() {
                                if (source.x).abs_diff(x) <= 10 && (source.y).abs_diff(y) <= 10 {
                                    commands.entity(entity).insert(Selected);
                                    break 'outer;
                                }
                            }
                            for (entity, mic) in mic_set.p1().iter() {
                                if (mic.x).abs_diff(x) <= 10 && (mic.y).abs_diff(y) <= 10 {
                                    commands.entity(entity).insert(Selected);
                                    break 'outer;
                                }
                            }
                            for (entity, rect_wall) in rect_wall_set.p1().iter() {
                                let center = rect_wall.get_center();
                                if (center.x).abs_diff(x) <= 10 && (center.y).abs_diff(y) <= 10 {
                                    commands.entity(entity).insert(Selected);
                                    break 'outer;
                                }
                            }
                            for (entity, circ_wall) in circ_wall_set.p1().iter() {
                                if (circ_wall.center.x).abs_diff(x) <= 10
                                    && (circ_wall.center.y).abs_diff(y) <= 10
                                {
                                    commands.entity(entity).insert(Selected);
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
                ToolType::Place(t) => match t {
                    PlaceType::Source => {
                        if let Some((x, y)) =
                            screen_to_grid(position.x, position.y, ui_state.image_rect)
                        {
                            commands.spawn(Source::new(
                                x,
                                y,
                                SourceType::default(),
                                component_ids.get_new_source_id(),
                            ));
                        }
                    }
                    PlaceType::RectWall => {
                        if let Some((x, y)) =
                            screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                        {
                            commands.spawn((
                                RectWall::new(
                                    x,
                                    y,
                                    x,
                                    y,
                                    ui_state.wall_is_hollow,
                                    ui_state.wall_reflection_factor,
                                    component_ids.get_new_wall_id(),
                                ),
                                WResize::Draw,
                            ));
                        }
                    }
                    PlaceType::CircWall => {
                        if let Some((x, y)) =
                            screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                        {
                            commands.spawn((
                                CircWall::new(
                                    x,
                                    y,
                                    0,
                                    ui_state.wall_is_hollow,
                                    ui_state.wall_reflection_factor,
                                    component_ids.get_new_wall_id(),
                                ),
                                WResize::Radius,
                            ));
                        }
                    }
                    PlaceType::Mic => {
                        if let Some((x, y)) =
                            screen_to_grid(position.x, position.y, ui_state.image_rect)
                        {
                            commands.spawn(Microphone::new(x, y, component_ids.get_new_mic_id()));
                        }
                    }
                },
                ToolType::Move => {
                    // This should only allow for one object to be selected
                    'outer: {
                        if let Some((x, y)) =
                            screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                        {
                            for (entity, source) in source_set.p0().iter() {
                                let (s_x, s_y) = (source.x, source.y);
                                if s_x.abs_diff(x) <= 10 && s_y.abs_diff(y) <= 10 {
                                    //values should change depending on image size (smaller image -> greater radius)
                                    commands.entity(entity).insert((Move, Selected));
                                    break; // only drag one at a time
                                }
                            }
                        }
                        if let Some((x, y)) =
                            screen_to_grid(position.x, position.y, ui_state.image_rect)
                        {
                            let rect_walls = rect_wall_set.p0();
                            let circ_walls = circ_wall_set.p0();
                            let walls = rect_walls
                                .iter()
                                .map(|(e, w)| (e, w as &dyn Wall))
                                .chain(circ_walls.iter().map(|(e, w)| (e, w as &dyn Wall)));
                            for (entity, wall) in walls {
                                let center = wall.get_center();
                                if (center.x).abs_diff(x) <= 10 && (center.y).abs_diff(y) <= 10 {
                                    commands.entity(entity).insert((Move, Selected));
                                    break;
                                }
                            }
                        }
                        if let Some((x, y)) =
                            screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                        {
                            for (entity, mic) in mic_set.p0().iter() {
                                let (m_x, m_y) = (mic.x, mic.y);
                                if m_x.abs_diff(x) <= 10 && m_y.abs_diff(y) <= 10 {
                                    //values should change depending on image size (smaller image -> greater radius)
                                    commands.entity(entity).insert((Move, Selected));
                                    break; // only drag one at a time
                                }
                            }
                        }
                    }
                }
                // ToolType::MoveSource => {
                // if let Some((x, y)) =
                //     screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                // {
                //     for (entity, source) in sources.iter() {
                //         let (s_x, s_y) = (source.x, source.y);
                //         if s_x.abs_diff(x) <= 10 && s_y.abs_diff(y) <= 10 {
                //             //values should change depending on image size (smaller image -> greater radius)
                //             commands.entity(entity).insert((Move, Selected));
                //             break; // only drag one at a time
                //         }
                //     }
                // }
                // }
                // ToolType::MoveWall => {
                //     if let Some((x, y)) =
                //         screen_to_grid(position.x, position.y, ui_state.image_rect, &ui_state)
                //     {
                //         let rect_walls = rect_wall_set.p0();
                //         let circ_walls = circ_wall_set.p0();
                //         let walls = rect_walls
                //             .iter()
                //             .map(|(e, w)| (e, w as &dyn Wall))
                //             .chain(circ_walls.iter().map(|(e, w)| (e, w as &dyn Wall)));
                //         for (entity, wall) in walls {
                //             let center = wall.get_center();
                //             if (center.x).abs_diff(x) <= 10 && (center.y).abs_diff(y) <= 10 {
                //                 commands.entity(entity).insert((Move, Selected));
                //                 break;
                //             }
                //         }
                //     }
                // }
                // ToolType::MoveMic => {
                // if let Some((x, y)) =
                //     screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                // {
                //     for (entity, mic) in microphones.iter() {
                //         let (m_x, m_y) = (mic.x, mic.y);
                //         if m_x.abs_diff(x) <= 10 && m_y.abs_diff(y) <= 10 {
                //             //values should change depending on image size (smaller image -> greater radius)
                //             commands.entity(entity).insert((Move, Selected));
                //             break; // only drag one at a time
                //         }
                //     }
                // }
                // }
                ToolType::ResizeWall => {
                    if let Some((x, y)) =
                        screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                    {
                        for (entity, wall) in rect_wall_set.p0().iter() {
                            for resize_type in [
                                WResize::TopLeft,
                                WResize::TopRight,
                                WResize::BottomLeft,
                                WResize::BottomRight,
                            ] {
                                let resize_point = wall.get_resize_point(&resize_type);
                                if (resize_point.x).abs_diff(x) <= 10
                                    && (resize_point.y).abs_diff(y) <= 10
                                {
                                    commands.entity(entity).insert(resize_type);
                                    break;
                                }
                            }
                        }
                        for (entity, wall) in circ_wall_set.p0().iter() {
                            let resize_point = wall.get_resize_point(&WResize::Radius);
                            if (resize_point.x).abs_diff(x) <= 10
                                && (resize_point.y).abs_diff(y) <= 10
                            {
                                commands.entity(entity).insert(WResize::Radius);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    if mouse_buttons.just_released(MouseButton::Left) {
        source_set.p2().iter_mut().for_each(|(entity, _)| {
            commands.entity(entity).remove::<Move>();
        });
        mic_set.p2().iter_mut().for_each(|(entity, _)| {
            commands.entity(entity).remove::<Move>();
        });
        rect_wall_set
            .p0()
            .iter_mut()
            .for_each(|(entity, rect_wall)| {
                if rect_wall.is_deletable() {
                    commands.entity(entity).despawn();
                    component_ids.decrement_wall_ids();
                }
                commands.entity(entity).remove::<(WResize, Move)>();
            });
        circ_wall_set
            .p0()
            .iter_mut()
            .for_each(|(entity, circ_wall)| {
                if circ_wall.is_deletable() {
                    commands.entity(entity).despawn();
                    component_ids.decrement_wall_ids();
                }
                commands.entity(entity).remove::<(WResize, Move)>();
            });

        wall_update_ev.send(UpdateWalls);
    }

    if mouse_buttons.pressed(MouseButton::Left) && ui_state.tools_enabled {
        let window = q_windows.single();

        if let Some(position) = window.cursor_position() {
            match ui_state.current_tool {
                // ToolType::MoveSource => {
                //     if let Some((x, y)) =
                //         screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                //     {
                //         drag_sources.iter_mut().for_each(|(_, mut source)| {
                //             source.x = x;
                //             source.y = y;
                //         });
                //     }
                // }
                ToolType::Move => {
                    if let Some((x, y)) =
                        screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                    {
                        source_set.p2().iter_mut().for_each(|(_, mut source)| {
                            source.x = x;
                            source.y = y;
                        });
                        rect_wall_set.p2().iter_mut().for_each(|(_, mut wall)| {
                            wall.set_center(x, y);
                        });
                        circ_wall_set.p2().iter_mut().for_each(|(_, mut wall)| {
                            wall.set_center(x, y);
                        });
                        mic_set.p2().iter_mut().for_each(|(_, mut mic)| {
                            mic.x = x;
                            mic.y = y;
                        });

                        if ctrl {
                            // snap all four corners to grid
                            rect_wall_set.p2().iter_mut().for_each(|(_, mut wall)| {
                                let min = UVec2 {
                                    x: (wall.rect.min.x as f32 / 10.).round() as u32 * 10,
                                    y: (wall.rect.min.y as f32 / 10.).round() as u32 * 10,
                                };
                                let max = UVec2 {
                                    x: (wall.rect.max.x as f32 / 10.).round() as u32 * 10,
                                    y: (wall.rect.max.y as f32 / 10.).round() as u32 * 10,
                                };

                                wall.resize(&WResize::TopLeft, min.x, min.y);
                                wall.resize(&WResize::TopRight, max.x, min.y);
                                wall.resize(&WResize::BottomLeft, min.x, max.y);
                                wall.resize(&WResize::BottomRight, max.x, max.y);
                            });
                        }
                    }
                }
                ToolType::Place(PlaceType::RectWall)
                | ToolType::Place(PlaceType::CircWall)
                | ToolType::ResizeWall => {
                    if let Some((x, y)) =
                        screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                    {
                        rect_wall_set
                            .p3()
                            .iter_mut()
                            .for_each(|(_, wall_resize, mut wall)| wall.resize(wall_resize, x, y));
                        circ_wall_set
                            .p3()
                            .iter_mut()
                            .for_each(|(_, wall_resize, mut wall)| wall.resize(wall_resize, x, y));

                        if ctrl {
                            // snap all four corners to grid
                            rect_wall_set.p3().iter_mut().for_each(|(_, _, mut wall)| {
                                let min = UVec2 {
                                    x: (wall.rect.min.x as f32 / 10.).round() as u32 * 10,
                                    y: (wall.rect.min.y as f32 / 10.).round() as u32 * 10,
                                };
                                let max = UVec2 {
                                    // - 1 because wall bounds are inclusive
                                    x: (wall.rect.max.x as f32 / 10.).round() as u32 * 10 - 1,
                                    y: (wall.rect.max.y as f32 / 10.).round() as u32 * 10 - 1,
                                };
                                wall.resize(&WResize::TopLeft, min.x, min.y);
                                wall.resize(&WResize::TopRight, max.x, min.y);
                                wall.resize(&WResize::BottomLeft, min.x, max.y);
                                wall.resize(&WResize::BottomRight, max.x, max.y);
                            });
                        }
                    }
                }
                // ToolType::MoveWall => {
                //     if let Some((x, y)) =
                //         screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                //     {
                //         rect_wall_set.p1().iter_mut().for_each(|(_, mut wall)| {
                //             wall.set_center(x, y);
                //         });
                //         circ_wall_set.p1().iter_mut().for_each(|(_, mut wall)| {
                //             wall.set_center(x, y);
                //         });

                //         if ctrl {
                //             // snap all four corners to grid
                //             rect_wall_set.p1().iter_mut().for_each(|(_, mut wall)| {
                //                 let min = UVec2 {
                //                     x: (wall.rect.min.x as f32 / 10.).round() as u32 * 10,
                //                     y: (wall.rect.min.y as f32 / 10.).round() as u32 * 10,
                //                 };
                //                 let max = UVec2 {
                //                     x: (wall.rect.max.x as f32 / 10.).round() as u32 * 10,
                //                     y: (wall.rect.max.y as f32 / 10.).round() as u32 * 10,
                //                 };

                //                 wall.resize(&WResize::TopLeft, min.x, min.y);
                //                 wall.resize(&WResize::TopRight, max.x, min.y);
                //                 wall.resize(&WResize::BottomLeft, min.x, max.y);
                //                 wall.resize(&WResize::BottomRight, max.x, max.y);
                //             });
                //         }
                //     }
                // }
                // ToolType::MoveMic => {
                //     if let Some((x, y)) =
                //         screen_to_nearest_grid(position.x, position.y, ui_state.image_rect)
                //     {
                //         drag_microphones.iter_mut().for_each(|(_, mut mic)| {
                //             mic.x = x;
                //             mic.y = y;
                //         });
                //     }
                // }
                _ => {}
            }
        }
    }

    if keys.just_pressed(KeyCode::Space) {
        ui_state.is_running = !ui_state.is_running;
    }
}

/// This system handles all inputs that dispatch events
pub fn event_input(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    ui_state: ResMut<UiState>,
    mut reset_ev: EventWriter<Reset>,
    mut save_ev: EventWriter<Save>,
    mut load_ev: EventWriter<Load>,
    mut wall_update_ev: EventWriter<UpdateWalls>,
    mut selected: Query<Entity, With<Selected>>,
    mut commands: Commands,
) {
    #[cfg(not(target_os = "macos"))]
    let ctrl = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    #[cfg(target_os = "macos")]
    let ctrl = keys.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);

    if keys.just_pressed(KeyCode::Delete) || keys.just_pressed(KeyCode::Backspace) {
        selected.iter_mut().for_each(|entity| {
            commands.entity(entity).despawn();
            wall_update_ev.send(UpdateWalls);
            reset_ev.send(Reset::default());
        });
    }

    // reset when clicking (somewhere) on the image
    if mouse_buttons.just_released(MouseButton::Left)
        && ui_state.tool_use_enabled
        && ui_state.tools_enabled
    {
        reset_ev.send(Reset::default());
    }

    // load file
    if ctrl && keys.just_pressed(KeyCode::KeyL) {
        load_ev.send(Load);
    }
    // save file
    if ctrl && keys.just_pressed(KeyCode::KeyS) {
        save_ev.send(Save);
    }
}
