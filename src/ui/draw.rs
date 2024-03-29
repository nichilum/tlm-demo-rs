use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_file_dialog::prelude::*;
use bevy_pixel_buffer::bevy_egui::egui::{Color32, Frame, Margin, Vec2};
use bevy_pixel_buffer::bevy_egui::EguiContexts;
use bevy_pixel_buffer::prelude::*;

use super::dialog::SaveFileContents;
use super::tabs::{DockState, PlotTabs};
use crate::components::gizmo::GizmoComponent;
use crate::components::microphone::*;
use crate::components::source::*;
use crate::components::states::{MenuSelected, Selected};
use crate::components::wall::{CircWall, RectWall, WResize};
use crate::events::{Reset, UpdateWalls};
use crate::grid::grid::Grid;
use crate::math::constants::*;
use crate::math::transformations::coords_to_index;
use crate::render::gradient::Gradient;
use crate::ui::state::*;

pub fn draw_egui(
    mut commands: Commands,
    diagnostics: Res<DiagnosticsStore>,
    mut pixel_buffers: QueryPixelBuffer,
    mut egui_context: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut grid: ResMut<Grid>,
    mut gradient: ResMut<Gradient>,
    mut wall_update_ev: EventWriter<UpdateWalls>,
    mut reset_ev: EventWriter<Reset>,
    mut rect_wall_set: ParamSet<(
        Query<(Entity, &mut RectWall)>,
        Query<(Entity, &mut RectWall), With<Selected>>,
        Query<(Entity, &mut RectWall), With<MenuSelected>>,
        Query<&RectWall>,
    )>,
    mut circ_wall_set: ParamSet<(
        Query<(Entity, &mut CircWall)>,
        Query<(Entity, &mut CircWall), With<Selected>>,
        Query<(Entity, &mut CircWall), With<MenuSelected>>,
        Query<&CircWall>,
    )>,
    mut source_set: ParamSet<(
        Query<(Entity, &mut Source)>,
        Query<(Entity, &Source), With<Selected>>,
        Query<(Entity, &Source), With<MenuSelected>>,
        Query<&Source>,
    )>,
    mut mic_set: ParamSet<(
        Query<(Entity, &mut Microphone)>,
        Query<(Entity, &Microphone), With<Selected>>,
        Query<(Entity, &Microphone), With<MenuSelected>>,
        Query<&Microphone>,
    )>,
    mut dock_state: ResMut<DockState>,
    mut fft_mic: ResMut<FftMicrophone>,
) {
    let ctx = egui_context.ctx_mut();
    egui_extras::install_image_loaders(ctx);

    let images = [
        (
            ToolType::PlaceSource,
            egui::include_image!("../../assets/place_source.png"),
        ),
        (
            ToolType::MoveSource,
            egui::include_image!("../../assets/move_source.png"),
        ),
        (
            ToolType::DrawWall,
            egui::include_image!("../../assets/draw_wall.png"),
        ),
        (
            ToolType::ResizeWall,
            egui::include_image!("../../assets/resize_wall.png"),
        ),
        (
            ToolType::MoveWall,
            egui::include_image!("../../assets/move_wall.png"),
        ),
        (
            ToolType::PlaceMic,
            egui::include_image!("../../assets/place_mic.png"),
        ),
        (
            ToolType::MoveMic,
            egui::include_image!("../../assets/move_mic.png"),
        ),
    ];

    if ui_state.show_help {
        egui::Window::new("Help")
            .open(&mut ui_state.show_help)
            .default_size(Vec2::new(400., 400.))
            .resizable(true)
            .collapsible(false)
            .constrain(true)
            .show(ctx, |ui| {
                ui.label("- put keybinds here");
                ui.label("- link to manual/docs");
                ui.label("- credits (link to yt?)");
                ui.label("- experimental settings (spectrogram)");
            });
    }

    // Side Panel (Sources, Mic, Walls, Tool Options, Settings)
    egui::SidePanel::left("left_panel")
        .default_width(400.)
        .resizable(false)
        .show(ctx, |ui| {
            ui_state.tools_enabled = !ui.rect_contains_pointer(ui.available_rect_before_wrap())
                && !ui_state.render_abc_area;

            ui.spacing_mut().slider_width = 200.0;

            ui.add_space(3.);
            egui::Grid::new("header_grid")
                .min_col_width(400. / 2.)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("Settings");
                        if let Some(value) = diagnostics
                            .get(&FrameTimeDiagnosticsPlugin::FPS)
                            .and_then(|fps| fps.smoothed())
                        {
                            ui.label(format!("FPS: {:.1}", value));
                        }
                    });
                    ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                        ui.horizontal(|ui| {
                            if ui
                                .button("Save")
                                .on_hover_text("Save the current state of the simulation")
                                .clicked()
                            {
                                // TODO: not super happy with this, would like to move it to the dialog system
                                let source_set = source_set.p3();
                                let mic_set = mic_set.p3();
                                let rect_wall_set = rect_wall_set.p3();
                                let circ_wall_set = circ_wall_set.p3();

                                let sources = source_set.iter().collect::<Vec<_>>();
                                let mics = mic_set.iter().collect::<Vec<_>>();
                                let rect_walls = rect_wall_set.iter().collect::<Vec<_>>();
                                let circ_walls = circ_wall_set.iter().collect::<Vec<_>>();

                                let data = crate::ui::saving::save(
                                    &sources,
                                    &mics,
                                    &rect_walls,
                                    &circ_walls,
                                )
                                .unwrap();

                                commands
                                    .dialog()
                                    .add_filter("JSON", &["json"])
                                    .set_file_name("save.json")
                                    .set_directory("./")
                                    .set_title("Select a file to save to")
                                    .save_file::<SaveFileContents>(data);
                            }

                            if ui
                                .button("Load")
                                .on_hover_text("Load a previously saved state of the simulation")
                                .clicked()
                            {
                                commands
                                    .dialog()
                                    .add_filter("JSON", &["json"])
                                    .set_directory("./")
                                    .set_title("Select a file to load")
                                    .load_file::<SaveFileContents>();
                            }

                            if ui
                                .button("Help")
                                .clicked()
                            {
                                ui_state.show_help = true;
                            }
                        });
                        if ui
                            .button("Screenshot")
                            .on_hover_text("Save a screenshot of the simulation")
                            .clicked()
                        {
                            let mut pixels: Vec<u8> = Vec::new();

                            for y in ui_state.boundary_width
                                ..(SIMULATION_WIDTH + ui_state.boundary_width)
                            {
                                for x in ui_state.boundary_width
                                    ..(SIMULATION_HEIGHT + ui_state.boundary_width)
                                {
                                    let current_index =
                                        coords_to_index(x, y, ui_state.boundary_width);
                                    if grid.wall_cache[current_index].is_wall {
                                        let mut reflection_factor =
                                            grid.wall_cache[current_index].reflection_factor;
                                        if reflection_factor == 0. {
                                            reflection_factor = 1.;
                                        }
                                        pixels.push((reflection_factor * 255.) as u8);
                                        pixels.push((reflection_factor * 255.) as u8);
                                        pixels.push((reflection_factor * 255.) as u8);
                                    } else {
                                        let pressure = grid.pressure[current_index];

                                        let color =
                                            gradient.at(pressure, ui_state.gradient_contrast);

                                        // gamma correction to match the brightness/contrast of the simulation
                                        pixels.push(
                                            ((color.r() as f32 / 255.).powf(1. / 2.2) * 255.) as u8,
                                        );
                                        pixels.push(
                                            ((color.g() as f32 / 255.).powf(1. / 2.2) * 255.) as u8,
                                        );
                                        pixels.push(
                                            ((color.b() as f32 / 255.).powf(1. / 2.2) * 255.) as u8,
                                        );
                                    }
                                }
                            }

                            let mut data = Vec::new();
                            let encoder = image::codecs::png::PngEncoder::new(&mut data);

                            let image = image::RgbImage::from_raw(
                                SIMULATION_WIDTH,
                                SIMULATION_HEIGHT,
                                pixels,
                            )
                            .expect("could not create image");

                            image
                                .write_with_encoder(encoder)
                                .expect("could not write image");

                            commands
                                .dialog()
                                .add_filter("PNG", &["png"])
                                .set_file_name("screenshot.png")
                                .set_directory("./")
                                .set_title("Select a file to save to")
                                .save_file::<SaveFileContents>(data);
                        }
                    })
                });

            ui.separator();

            // Sources
            egui::ScrollArea::vertical()
                .id_source("source_scroll_area")
                .max_height(400.)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());

                    let mut binding = source_set.p0();
                    let mut source_vec = binding.iter_mut().collect::<Vec<_>>();
                    source_vec.sort_by_cached_key(|(_, source)| source.id);

                    source_vec.iter_mut().for_each(|(entity, ref mut source)| {
                        let collapse = ui.collapsing(format!("Source {}", source.id), |ui| {
                            ui.horizontal(|ui| {
                                ui.label("x:");
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut source.x)
                                            .speed(1)
                                            .clamp_range(0.0..=SIMULATION_WIDTH as f32 - 1.),
                                    )
                                    .changed()
                                {
                                    reset_ev.send(Reset);
                                }
                                ui.add_space(10.);
                                ui.label("y:");
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut source.y)
                                            .speed(1)
                                            .clamp_range(0.0..=SIMULATION_HEIGHT as f32 - 1.),
                                    )
                                    .changed()
                                {
                                    reset_ev.send(Reset);
                                }
                            });

                            egui::ComboBox::from_label("Waveform")
                                .selected_text(format!("{}", source.source_type))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut source.source_type,
                                        SourceType::default_sin(),
                                        "Sinus",
                                    );
                                    ui.selectable_value(
                                        &mut source.source_type,
                                        SourceType::default_gauss(),
                                        "Gauss",
                                    );
                                    ui.selectable_value(
                                        &mut source.source_type,
                                        SourceType::default_noise(),
                                        "White Noise",
                                    );
                                });

                            match &mut source.source_type {
                                SourceType::Sin {
                                    phase,
                                    frequency,
                                    amplitude,
                                } => {
                                    if ui
                                        .add(
                                            egui::Slider::new(frequency, 0.0..=20000.0)
                                                .text("Frequency (Hz)"),
                                        )
                                        .changed()
                                    {
                                        reset_ev.send(Reset);
                                    }
                                    if ui
                                        .add(
                                            egui::Slider::new(amplitude, 0.0..=25.0)
                                                .text("Amplitude"),
                                        )
                                        .changed()
                                    {
                                        reset_ev.send(Reset);
                                    }
                                    if ui
                                        .add(
                                            egui::Slider::new(phase, 0.0..=360.0).text("Phase (°)"),
                                        )
                                        .changed()
                                    {
                                        reset_ev.send(Reset);
                                    }
                                }
                                SourceType::Gauss {
                                    phase,
                                    frequency,
                                    amplitude,
                                } => {
                                    if ui
                                        .add(
                                            egui::Slider::new(frequency, 0.0..=20000.0)
                                                .text("Frequency (Hz)"),
                                        )
                                        .changed()
                                    {
                                        reset_ev.send(Reset);
                                    }
                                    if ui
                                        .add(
                                            egui::Slider::new(amplitude, 0.0..=25.0)
                                                .text("Amplitude"),
                                        )
                                        .changed()
                                    {
                                        reset_ev.send(Reset);
                                    }
                                    if ui
                                        .add(
                                            egui::Slider::new(phase, 0.0..=360.0).text("Phase (°)"),
                                        )
                                        .changed()
                                    {
                                        reset_ev.send(Reset);
                                    }
                                }
                                SourceType::WhiteNoise { amplitude } => {
                                    if ui
                                        .add(
                                            egui::Slider::new(amplitude, 0.0..=25.0)
                                                .text("Amplitude"),
                                        )
                                        .changed()
                                    {
                                        reset_ev.send(Reset);
                                    }
                                }
                            }

                            if ui
                                .add(egui::Button::new("Delete").fill(Color32::DARK_RED))
                                .clicked()
                            {
                                commands.entity(*entity).despawn();
                            }
                        });
                        if collapse.header_response.contains_pointer()
                            || collapse.body_response.is_some()
                        {
                            commands.entity(*entity).try_insert(MenuSelected);
                        } else {
                            commands.entity(*entity).remove::<MenuSelected>();
                        }
                    });
                });

            if !source_set.p0().is_empty() {
                ui.separator();
            }

            // Microphones
            egui::ScrollArea::vertical()
                .id_source("mic_scroll_area")
                .max_height(400.)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());

                    let mut binding = mic_set.p0();
                    let mut mic_vec = binding.iter_mut().collect::<Vec<_>>();
                    mic_vec.sort_by_cached_key(|(_, mic)| mic.id);

                    mic_vec.iter_mut().for_each(|(entity, ref mut mic)| {
                        let collapse = ui.collapsing(format!("Microphone {}", mic.id), |ui| {
                            ui.horizontal(|ui| {
                                ui.label("x:");
                                ui.add(
                                    egui::DragValue::new(&mut mic.x)
                                        .speed(1)
                                        .clamp_range(0.0..=SIMULATION_WIDTH as f32 - 1.),
                                );
                                ui.add_space(10.);
                                ui.label("y:");
                                ui.add(
                                    egui::DragValue::new(&mut mic.y)
                                        .speed(1)
                                        .clamp_range(0.0..=SIMULATION_HEIGHT as f32 - 1.),
                                );
                            });
                            if ui
                                .add(egui::Button::new("Delete").fill(Color32::DARK_RED))
                                .clicked()
                            {
                                if let Some(current_id) = fft_mic.mic_id {
                                    if current_id == mic.id {
                                        fft_mic.mic_id = None;
                                    }
                                }
                                commands.entity(*entity).despawn();
                            }
                        });
                        if collapse.header_response.contains_pointer()
                            || collapse.body_response.is_some()
                        {
                            commands.entity(*entity).try_insert(MenuSelected);
                        } else {
                            commands.entity(*entity).remove::<MenuSelected>();
                        }
                    });
                });

                if !mic_set.p0().is_empty() {
                    ui.separator();
                }

            // Rect Walls
            egui::ScrollArea::vertical()
                .id_source("rect_wall_scroll_area")
                .max_height(400.)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());

                    let mut rect_binding = rect_wall_set.p0();
                    let mut wall_vec = rect_binding.iter_mut().collect::<Vec<_>>();
                    wall_vec.sort_by_cached_key(|(_, wall)| wall.id);

                    wall_vec.iter_mut().for_each(|(entity, ref mut wall)| {
                        let collapse =
                            ui.collapsing(format!("Rectangular Wall {}", wall.id), |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("x:");
                                    if ui
                                        .add(
                                            egui::DragValue::new(&mut wall.rect.min.x)
                                                .speed(1)
                                                .clamp_range(0..=SIMULATION_WIDTH - 1),
                                        )
                                        .changed()
                                    {
                                        commands.entity(*entity).try_insert(WResize::Menu);
                                        if wall.rect.min.x > wall.rect.max.x - 1 {
                                            wall.rect.min.x = wall.rect.max.x - 1;
                                        }
                                        reset_ev.send(Reset);
                                    }
                                    ui.add_space(10.);
                                    ui.label("y:");
                                    if ui
                                        .add(
                                            egui::DragValue::new(&mut wall.rect.min.y)
                                                .speed(1)
                                                .clamp_range(0..=SIMULATION_HEIGHT - 1),
                                        )
                                        .changed()
                                    {
                                        commands.entity(*entity).try_insert(WResize::Menu);
                                        if wall.rect.min.y > wall.rect.max.y - 1 {
                                            wall.rect.min.y = wall.rect.max.y - 1;
                                        }
                                        reset_ev.send(Reset);
                                    }
                                    ui.add_space(10.);
                                    ui.label("Top Left Corner");
                                });

                                ui.horizontal(|ui| {
                                    ui.label("x:");
                                    if ui
                                        .add(
                                            egui::DragValue::new(&mut wall.rect.max.x)
                                                .speed(1)
                                                .clamp_range(0..=SIMULATION_WIDTH - 1),
                                        )
                                        .changed()
                                    {
                                        commands.entity(*entity).try_insert(WResize::Menu);
                                        if wall.rect.max.x < wall.rect.min.x + 1 {
                                            wall.rect.max.x = wall.rect.min.x + 1;
                                        }
                                        reset_ev.send(Reset);
                                    }
                                    ui.add_space(10.);
                                    ui.label("y:");
                                    if ui
                                        .add(
                                            egui::DragValue::new(&mut wall.rect.max.y)
                                                .speed(1)
                                                .clamp_range(0..=SIMULATION_HEIGHT - 1),
                                        )
                                        .changed()
                                    {
                                        commands.entity(*entity).try_insert(WResize::Menu);
                                        if wall.rect.max.y < wall.rect.min.y + 1 {
                                            wall.rect.max.y = wall.rect.min.y + 1;
                                        }
                                        reset_ev.send(Reset);
                                    }
                                    ui.add_space(10.);
                                    ui.label("Bottom Right Corner");
                                });

                                ui.horizontal(|ui| {
                                    ui.label(format!(
                                        "Width: {:.3} m",
                                        wall.rect.width() as f32 * ui_state.delta_l
                                    ));

                                    ui.add_space(10.);

                                    ui.label(format!(
                                        "Height: {:.3} m",
                                        wall.rect.height() as f32 * ui_state.delta_l
                                    ));
                                });

                                if ui
                                    .add(
                                        // 0.01 because rendering then draws white
                                        egui::Slider::new(&mut wall.reflection_factor, 0.01..=1.0)
                                            .text("Wall Reflection Factor"),
                                    )
                                    .changed()
                                {
                                    reset_ev.send(Reset);
                                }

                                if ui.checkbox(&mut wall.is_hollow, "Hollow").changed() {
                                    wall_update_ev.send(UpdateWalls);
                                    reset_ev.send(Reset);
                                };

                                if ui
                                    .add(egui::Button::new("Delete").fill(Color32::DARK_RED))
                                    .clicked()
                                {
                                    commands.entity(*entity).despawn();
                                    wall_update_ev.send(UpdateWalls);
                                    reset_ev.send(Reset);
                                }
                            });

                        if collapse.header_response.contains_pointer()
                            || collapse.body_response.is_some()
                        {
                            commands.entity(*entity).try_insert(MenuSelected);
                        } else {
                            commands.entity(*entity).remove::<MenuSelected>();
                        }
                    });
                });
            // Circ Walls
            egui::ScrollArea::vertical()
                .id_source("circ_wall_scroll_area")
                .max_height(400.)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());

                    let mut circ_binding = circ_wall_set.p0();
                    let mut wall_vec = circ_binding.iter_mut().collect::<Vec<_>>();
                    wall_vec.sort_by_cached_key(|(_, wall)| wall.id);

                    wall_vec.iter_mut().for_each(|(entity, ref mut wall)| {
                        let collapse = ui.collapsing(format!("Circular Wall {}", wall.id), |ui| {
                            ui.horizontal(|ui| {
                                ui.label("x:");
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut wall.center.x)
                                            .speed(1)
                                            .clamp_range(0..=SIMULATION_WIDTH - 1),
                                    )
                                    .changed()
                                {
                                    commands.entity(*entity).try_insert(WResize::Menu);
                                    reset_ev.send(Reset);
                                }
                                ui.add_space(10.);
                                ui.label("y:");
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut wall.center.y)
                                            .speed(1)
                                            .clamp_range(0..=SIMULATION_HEIGHT - 1),
                                    )
                                    .changed()
                                {
                                    commands.entity(*entity).try_insert(WResize::Menu);
                                    reset_ev.send(Reset);
                                }
                                ui.add_space(10.);
                                ui.label("Center");

                                ui.add_space(5.);
                                ui.add(egui::Separator::default().vertical());
                                ui.add_space(5.);

                                ui.label("Radius:");
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut wall.radius)
                                            .speed(1)
                                            .clamp_range(1..=1000),
                                    )
                                    .changed()
                                {
                                    commands.entity(*entity).try_insert(WResize::Menu);
                                    reset_ev.send(Reset);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label(format!(
                                    "Radius: {:.3} m",
                                    wall.radius as f32 * ui_state.delta_l
                                ));
                            });

                            if wall.is_hollow {
                                if ui
                                    .add(
                                        egui::Slider::new(
                                            &mut wall.open_circ_segment,
                                            0f32..=180f32.to_radians(),
                                        )
                                        .text("Open Circle Arc"),
                                    )
                                    .changed()
                                {
                                    commands.entity(*entity).try_insert(WResize::Menu);
                                    reset_ev.send(Reset);
                                }

                                if ui
                                    .add(
                                        egui::Slider::new(
                                            &mut wall.rotation_angle,
                                            0f32..=360f32.to_radians(),
                                        )
                                        .text("Rotation Angle"),
                                    )
                                    .changed()
                                {
                                    commands.entity(*entity).try_insert(WResize::Menu);
                                    reset_ev.send(Reset);
                                }
                            }

                            if ui
                                .add(
                                    egui::Slider::new(&mut wall.reflection_factor, 0.01..=1.0)
                                        .text("Wall Reflection Factor"),
                                )
                                .changed()
                            {
                                reset_ev.send(Reset);
                            }

                            if ui.checkbox(&mut wall.is_hollow, "Hollow Wall").changed() {
                                wall_update_ev.send(UpdateWalls);
                                reset_ev.send(Reset);
                            };

                            if ui
                                .add(egui::Button::new("Delete").fill(Color32::DARK_RED))
                                .clicked()
                            {
                                commands.entity(*entity).despawn();
                                wall_update_ev.send(UpdateWalls);
                                reset_ev.send(Reset);
                            }
                        });

                        if collapse.header_response.contains_pointer()
                            || collapse.body_response.is_some()
                        {
                            commands.entity(*entity).try_insert(MenuSelected);
                        } else {
                            commands.entity(*entity).remove::<MenuSelected>();
                        }
                    });
                });

            // General Settings
            egui::TopBottomPanel::bottom("general_settings_bottom_panel").show_inside(ui, |ui| {
                ui.add_space(3.);
                ui.heading("General Settings");
                ui.separator();

                ui.horizontal(|ui| {
                    if ui
                        .button(if ui_state.is_running { "Stop" } else { "Start" })
                        .clicked()
                    {
                        ui_state.is_running = !ui_state.is_running;
                    }

                    if ui.button("Reset").clicked() {
                        grid.reset_cells(ui_state.boundary_width);
                        for (_, mut mic) in mic_set.p0().iter_mut() {
                            mic.clear();
                        }
                    }

                    if ui
                        .add(egui::Button::new("Delete all").fill(Color32::DARK_RED))
                        .clicked()
                    {
                        for (e, _) in source_set.p0().iter() {
                            commands.entity(e).despawn();
                        }
                        for (e, _) in rect_wall_set.p0().iter() {
                            commands.entity(e).despawn();
                        }
                        for (e, _) in circ_wall_set.p0().iter() {
                            commands.entity(e).despawn();
                        }
                        for (e, _) in mic_set.p0().iter() {
                            commands.entity(e).despawn();
                        }

                        fft_mic.mic_id = None;

                        grid.reset_cells(ui_state.boundary_width);
                        wall_update_ev.send(UpdateWalls);
                    }

                    ui.checkbox(&mut ui_state.reset_on_change, "Reset on change")
                });

                if ui
                    .add(
                        egui::Slider::new(&mut ui_state.delta_l, 0.0..=10.0)
                            .text("Delta L (m)")
                            .logarithmic(true),
                    )
                    .on_hover_text("Change the size of one cell in the simulation in meters.")
                    .changed()
                {
                    reset_ev.send(Reset);
                }

                if ui
                    .checkbox(&mut ui_state.show_plots, "Show Plots")
                    .clicked()
                {
                    for (_, mut mic) in mic_set.p0().iter_mut() {
                        mic.clear();
                    }
                }

                ui.collapsing("Gradient", |ui| {
                    ui.label("Adjust the colors used to render the simulation.");
                    ui.add_space(5.);

                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut gradient.0).on_hover_text("The color used to show negative pressure values.");
                        ui.add_space(10.);
                        ui.label("Negative");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut gradient.1).on_hover_text("The color used to show neutral pressure values.");
                        ui.add_space(10.);
                        ui.label("Zero");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut gradient.2).on_hover_text("The color used to show positive pressure values.");
                        ui.add_space(10.);
                        ui.label("Positive");
                    });

                    ui.add(
                        egui::Slider::new(&mut ui_state.gradient_contrast, 0.0..=10.0)
                            .text("Gradient Contrast"),
                    ).on_hover_text("Adjust the contrast of the gradient. (this might lead to clipping the colors)");
                });

                ui.collapsing("Boundary", |ui| {
                    ui.label("Change the outer boundary for the free field simulation.");
                    ui.add_space(5.);

                    if ui
                        .checkbox(&mut ui_state.render_abc_area, "Show absorbing boundary")
                        .clicked()
                    {
                        ui_state.tools_enabled = !ui_state.render_abc_area;
                        let mut pb = pixel_buffers.iter_mut().next().expect("one pixel buffer");

                        pb.pixel_buffer.size = PixelBufferSize {
                            size: if ui_state.render_abc_area {
                                UVec2::new(
                                    SIMULATION_WIDTH + 2 * ui_state.boundary_width,
                                    SIMULATION_HEIGHT + 2 * ui_state.boundary_width,
                                )
                            } else {
                                UVec2::new(SIMULATION_WIDTH, SIMULATION_HEIGHT)
                            },
                            pixel_size: UVec2::new(1, 1),
                        };
                    }

                    if ui
                        .add(
                            egui::Slider::new(&mut ui_state.boundary_width, 2..=200)
                                .text("Boundary Width"),
                        )
                        .on_hover_text("Change the width of the boundary. (higher values lead to slower simulation)")
                        .changed()
                    {
                        grid.reset_cells(ui_state.boundary_width);
                        grid.reset_walls(ui_state.boundary_width);
                        grid.cache_boundaries(ui_state.boundary_width);
                        let mut pb = pixel_buffers.iter_mut().next().expect("one pixel buffer");
                        pb.pixel_buffer.size = PixelBufferSize {
                            size: if ui_state.render_abc_area {
                                UVec2::new(
                                    SIMULATION_WIDTH + 2 * ui_state.boundary_width,
                                    SIMULATION_HEIGHT + 2 * ui_state.boundary_width,
                                )
                            } else {
                                UVec2::new(SIMULATION_WIDTH, SIMULATION_HEIGHT)
                            },
                            pixel_size: UVec2::new(1, 1),
                        };
                    }
                });

                ui.add_space(10.);
            });

            // Tool Options
            egui::TopBottomPanel::bottom("tool_options_panel").show_inside(ui, |ui| {
                ui.add_space(3.);
                ui.heading("Tool Options");
                ui.separator();

                ui.set_enabled(!ui_state.render_abc_area);
                if ui_state.current_tool == ToolType::DrawWall {
                    egui::ComboBox::from_label("Select Wall Type")
                        .selected_text(format!("{:?}", ui_state.wall_type))
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.selectable_value(
                                &mut ui_state.wall_type,
                                WallType::Rectangle,
                                "Rectangle",
                            );
                            ui.selectable_value(
                                &mut ui_state.wall_type,
                                WallType::Circle,
                                "Circle",
                            );
                        });
                    ui.add(
                        egui::Slider::new(&mut ui_state.wall_reflection_factor, 0.0..=1.0)
                            .text("Wall Reflection Factor"),
                    );
                    ui.checkbox(&mut ui_state.wall_is_hollow, "Hollow");
                } else {
                    ui.add_space(10.);
                    ui.vertical_centered(|ui| ui.label("Select another tool to see its options"));
                }
                ui.add_space(10.);
            });
        });

    // Plot tabs
    if ui_state.show_plots {
        egui::TopBottomPanel::bottom("plot_tabs")
            .resizable(true)
            .default_height(400.)
            .max_height(ctx.screen_rect().height() / 2.)
            .frame(Frame::default().inner_margin(Margin {
                left: -1.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            }))
            .show(ctx, |ui| {
                let mut binding = mic_set.p3();
                let mics = binding.iter_mut().collect::<Vec<_>>();
                let mut pb = pixel_buffers.iter_mut().nth(1).expect("two pixel buffers");

                let mut style = egui_dock::Style::from_egui(ui.style());
                style.tab_bar.bg_fill = Color32::from_rgb(27, 27, 27);

                egui_dock::DockArea::new(&mut dock_state.tree)
                    .allowed_splits(egui_dock::AllowedSplits::None)
                    .draggable_tabs(false)
                    .style(style)
                    .show_inside(
                        ui,
                        &mut PlotTabs {
                            mics: &mics[..],
                            pixel_buffer: &mut pb,
                            fft_microphone: &mut fft_mic,
                            commands: &mut commands.reborrow(),
                        },
                    );
            });
    }

    // Tool Panel
    egui::SidePanel::left("tool_panel")
        .frame(
            Frame::default()
                .inner_margin(Margin {
                    left: 8., //looks better
                    right: 10.,
                    top: 10.,
                    bottom: 10.,
                })
                .fill(Color32::from_rgb(25, 25, 25)),
        )
        .default_width(35.)
        .resizable(false)
        .show(ctx, |ui| {
            ui.set_enabled(ui_state.tools_enabled);

            for (tool_type, icon) in images {
                if ui
                    .add(
                        egui::Button::image(
                            egui::Image::new(icon).fit_to_exact_size(Vec2::new(25., 25.)),
                        )
                        .fill(if tool_type == ui_state.current_tool {
                            Color32::DARK_GRAY
                        } else {
                            Color32::TRANSPARENT
                        })
                        .min_size(Vec2::new(0., 35.)),
                    )
                    .on_hover_text(format!("{}", tool_type))
                    .clicked()
                {
                    ui_state.current_tool = tool_type;
                }
                ui.add_space(4.);
            }
        });

    // Main Render Area
    egui::CentralPanel::default()
        .frame(
            Frame::default()
                .inner_margin(Margin {
                    left: 20.,
                    right: 20.,
                    top: 20.,
                    bottom: 20.,
                })
                .fill(Color32::from_rgb(25, 25, 25)),
        )
        .show(ctx, |ui| {
            ui_state.tool_use_enabled = ui.rect_contains_pointer(ui.min_rect().expand(20.));

            ui.set_min_width(100.);
            // Main Simulation Area

            let pb = pixel_buffers.iter().next().expect("first pixel buffer");
            let texture = pb.egui_texture();
            // let image = ui.image(egui::load::SizedTexture::new(texture.id, texture.size));

            let image = ui.add(
                egui::Image::new(egui::load::SizedTexture::new(texture.id, texture.size))
                    .shrink_to_fit(),
            );

            ui_state.image_rect = image.rect;

            // Gizmos

            if !ui_state.render_abc_area {
                let painter = ui.painter();
                let text_color = gradient.get_average();
                //menu gizmos
                if !ui_state.tools_enabled {
                    for (_, wall) in rect_wall_set.p2().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ToolType::MoveWall,
                            true,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                    for (_, wall) in circ_wall_set.p2().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ToolType::MoveWall,
                            true,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                    // all mics
                    for (_, mic) in mic_set.p2().iter() {
                        mic.draw_gizmo(
                            painter,
                            &ToolType::MoveMic,
                            true,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                    // all sources
                    for (_, source) in source_set.p2().iter() {
                        source.draw_gizmo(
                            painter,
                            &ToolType::MoveSource,
                            true,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                } else {
                    // Tool specific gizmos
                    // all walls
                    for wall in rect_wall_set.p3().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            false,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                    // selected walls
                    for (_, wall) in rect_wall_set.p1().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            true,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                    // all circ walls
                    for wall in circ_wall_set.p3().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            false,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                    // selected circ walls
                    for (_, wall) in circ_wall_set.p1().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            true,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                    // all mics
                    for mic in mic_set.p3().iter() {
                        mic.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            false,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                    // selected mics
                    for (_, mic) in mic_set.p1().iter() {
                        mic.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            true,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                    // all sources
                    for source in source_set.p3().iter() {
                        source.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            false,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                    // selected sources
                    for (_, source) in source_set.p1().iter() {
                        source.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            true,
                            &ui_state.image_rect,
                            ui_state.delta_l,
                            text_color,
                        );
                    }
                }
            }
        });
}
