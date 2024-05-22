use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_file_dialog::prelude::*;
use bevy_pixel_buffer::bevy_egui::egui::{Color32, Frame, Margin, Vec2};
use bevy_pixel_buffer::bevy_egui::EguiContexts;
use bevy_pixel_buffer::prelude::*;

use super::help::draw_help;
use super::loading::SaveFileContents;
use super::preferences::draw_preferences;
use super::tabs::{DockState, PlotTabs};
use crate::components::gizmo::GizmoComponent;
use crate::components::microphone::*;
use crate::components::source::*;
use crate::components::states::{MenuSelected, Selected};
use crate::components::wall::{CircWall, RectWall, WResize};
use crate::events::{Load, Reset, Save, UpdateWalls};
use crate::math::constants::*;
use crate::math::transformations::coords_to_index;
use crate::render::gradient::Gradient;
use crate::simulation::grid::Grid;
use crate::ui::state::*;
use crate::undo::{UndoEvent, UndoRedo};

#[derive(SystemParam)]
pub struct EventSystemParams<'w> {
    // ---- required lifetimes: ----
    // Query<'w, 's, Entity>,
    // Res<'w, SomeResource>,
    // ResMut<'w, SomeOtherResource>,
    // Local<'s, u8>,
    // Commands<'w, 's>,
    // EventReader<'w, 's, SomeEvent>,
    // EventWriter<'w, SomeEvent>
    pub wall_update_ev: EventWriter<'w, UpdateWalls>,
    pub reset_ev: EventWriter<'w, Reset>,
    pub undo_ev: EventWriter<'w, UndoEvent>,
    pub save_ev: EventWriter<'w, Save>,
    pub load_ev: EventWriter<'w, Load>,
}

type AllRectWallsMut<'w, 's> = Query<'w, 's, (Entity, &'static mut RectWall)>;
type AllRectWallsSelected<'w, 's> = Query<'w, 's, (Entity, &'static mut RectWall), With<Selected>>;
type AllRectWallsMenuSelected<'w, 's> =
    Query<'w, 's, (Entity, &'static mut RectWall), With<MenuSelected>>;
type AllRectWalls<'w, 's> = Query<'w, 's, &'static RectWall>;

type AllCircWallsMut<'w, 's> = Query<'w, 's, (Entity, &'static mut CircWall)>;
type AllCircWallsSelected<'w, 's> = Query<'w, 's, (Entity, &'static mut CircWall), With<Selected>>;
type AllCircWallsMenuSelected<'w, 's> =
    Query<'w, 's, (Entity, &'static mut CircWall), With<MenuSelected>>;
type AllCircWalls<'w, 's> = Query<'w, 's, &'static CircWall>;

type AllSourcesMut<'w, 's> = Query<'w, 's, (Entity, &'static mut Source)>;
type AllSourcesSelected<'w, 's> = Query<'w, 's, (Entity, &'static mut Source), With<Selected>>;
type AllSourcesMenuSelected<'w, 's> =
    Query<'w, 's, (Entity, &'static mut Source), With<MenuSelected>>;
type AllSources<'w, 's> = Query<'w, 's, &'static Source>;

type AllMicsMut<'w, 's> = Query<'w, 's, (Entity, &'static mut Microphone)>;
type AllMicsSelected<'w, 's> = Query<'w, 's, (Entity, &'static mut Microphone), With<Selected>>;
type AllMicsMenuSelected<'w, 's> =
    Query<'w, 's, (Entity, &'static mut Microphone), With<MenuSelected>>;
type AllMics<'w, 's> = Query<'w, 's, &'static Microphone>;

#[derive(SystemParam)]
pub struct QuerySystemParams<'w, 's> {
    rect_wall_set: ParamSet<
        'w,
        's,
        (
            AllRectWallsMut<'w, 's>,
            AllRectWallsSelected<'w, 's>,
            AllRectWallsMenuSelected<'w, 's>,
            AllRectWalls<'w, 's>,
        ),
    >,
    circ_wall_set: ParamSet<
        'w,
        's,
        (
            AllCircWallsMut<'w, 's>,
            AllCircWallsSelected<'w, 's>,
            AllCircWallsMenuSelected<'w, 's>,
            AllCircWalls<'w, 's>,
        ),
    >,
    source_set: ParamSet<
        'w,
        's,
        (
            AllSourcesMut<'w, 's>,
            AllSourcesSelected<'w, 's>,
            AllSourcesMenuSelected<'w, 's>,
            AllSources<'w, 's>,
        ),
    >,
    mic_set: ParamSet<
        'w,
        's,
        (
            AllMicsMut<'w, 's>,
            AllMicsSelected<'w, 's>,
            AllMicsMenuSelected<'w, 's>,
            AllMics<'w, 's>,
        ),
    >,
}

pub const CTRL_KEY_TEXT: &'static str = if cfg!(target_os = "macos") {
    "Cmd"
} else {
    "Ctrl"
};

pub fn draw_egui(
    mut commands: Commands,
    mut pixel_buffers: QueryPixelBuffer,
    mut egui_context: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut grid: ResMut<Grid>,
    mut gradient: ResMut<Gradient>,
    mut events: EventSystemParams,
    sets: QuerySystemParams,
    mut dock_state: ResMut<DockState>,
    mut fft_mic: ResMut<FftMicrophone>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    sim_time: Res<SimTime>,
    time: Res<Time>,
    mut fixed_timestep: ResMut<Time<Fixed>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let QuerySystemParams {
        mut rect_wall_set,
        mut circ_wall_set,
        mut source_set,
        mut mic_set,
    } = sets;

    let ctx = egui_context.ctx_mut();
    egui_extras::install_image_loaders(ctx);

    let images = [
        egui::include_image!("../../assets/select.png"),
        egui::include_image!("../../assets/place.png"),
        egui::include_image!("../../assets/move.png"),
        egui::include_image!("../../assets/resize_wall.png"),
    ];

    if ui_state.show_help {
        draw_help(&mut ui_state, ctx);
    }

    if ui_state.show_preferences {
        let mut show_preferences = ui_state.show_preferences;

        draw_preferences(
            &mut show_preferences,
            ctx,
            &mut ui_state,
            &mut events,
            &mut grid,
            &mut pixel_buffers,
            &mut gradient,
        );

        ui_state.show_preferences = show_preferences;
    }

    if ui_state.show_about {
        egui::Window::new("About")
            .open(&mut ui_state.show_about)
            .default_size(Vec2::new(400., 400.))
            .resizable(false)
            .collapsible(false)
            .constrain(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("wavefront");
                    ui.strong(format!("Version: {}", env!("CARGO_PKG_VERSION")));
                });

                ui.add_space(5.);
                ui.label("A wave simulation tool using the Transmission Line Matrix method.");

                ui.add_space(10.);

                ui.heading("Created by");
                ui.hyperlink("https://github.com/JonathanKr");
                ui.hyperlink("https://github.com/ecrax");

                ui.add_space(5.);
                ui.heading("Source");
                //TODO: maybe add links to papers?
                ui.hyperlink("https://github.com/nichilum/wavefront");
            });
    }

    if ui_state.show_epilepsy_warning {
        let mut read_epilepsy_warning = ui_state.read_epilepsy_warning;
        egui::Window::new("Epilepsy Warning")
            .default_size(Vec2::new(400., 400.))
            .resizable(false)
            .collapsible(false)
            .constrain(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("WARNING");
                });
                ui.strong("Increasing the speed may potentially trigger seizures for people with photosensitive epilepsy. Discretion is advised.");

                if ui.checkbox(&mut read_epilepsy_warning, "Understood").clicked() {
                    ui_state.show_epilepsy_warning = false;
                }
            });
        ui_state.read_epilepsy_warning = read_epilepsy_warning;
    }

    egui::TopBottomPanel::top("top_menu")
        .frame(
            Frame::default()
                .inner_margin(Margin {
                    left: 5.,
                    right: 5.,
                    top: 2.,
                    bottom: 2.,
                })
                .fill(Color32::from_rgb(15, 15, 15)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.visuals_mut().button_frame = false;
                ui.menu_button("File", |ui| {
                    if ui
                        .add(egui::Button::new("Save").shortcut_text(format!("{CTRL_KEY_TEXT}+S")))
                        .on_hover_text("Save the current state of the simulation")
                        .clicked()
                    {
                        ui.close_menu();
                        events.save_ev.send(Save);
                    }

                    if ui
                        .add(egui::Button::new("Load").shortcut_text(format!("{CTRL_KEY_TEXT}+L")))
                        .on_hover_text("Load a previously saved state of the simulation")
                        .clicked()
                    {
                        ui.close_menu();
                        events.load_ev.send(Load);
                    }

                    if ui
                        .button("Screenshot")
                        .on_hover_text("Save a screenshot of the simulation")
                        .clicked()
                    {
                        ui.close_menu();

                        let mut pixels: Vec<u8> = Vec::new();

                        for y in
                            ui_state.boundary_width..(SIMULATION_WIDTH + ui_state.boundary_width)
                        {
                            for x in ui_state.boundary_width
                                ..(SIMULATION_HEIGHT + ui_state.boundary_width)
                            {
                                let current_index = coords_to_index(x, y, ui_state.boundary_width);
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

                                    let color = gradient.at(pressure, -2., 2.);

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

                        let image =
                            image::RgbImage::from_raw(SIMULATION_WIDTH, SIMULATION_HEIGHT, pixels)
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

                    if ui
                        .add(egui::Button::new("Quit").shortcut_text("Esc"))
                        .on_hover_text("Quit the application")
                        .clicked()
                    {
                        app_exit_events.send(bevy::app::AppExit);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui
                        .add(egui::Button::new("Undo").shortcut_text(format!("{CTRL_KEY_TEXT}+Z")))
                        .clicked()
                    {
                        ui.close_menu();
                        events.undo_ev.send(UndoEvent(UndoRedo::Undo));
                    }
                    if ui
                        .add(
                            egui::Button::new("Redo")
                                .shortcut_text(format!("{CTRL_KEY_TEXT}+Shift+Z")),
                        )
                        .clicked()
                    {
                        ui.close_menu();
                        events.undo_ev.send(UndoEvent(UndoRedo::Redo));
                    }
                    if ui.button("Preferences").clicked() {
                        ui_state.show_preferences = true;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("Keybinds").clicked() {
                        ui_state.show_help = true;
                        ui.close_menu();
                    }
                    if ui.button("About").clicked() {
                        ui_state.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });

    // Side Panel (Sources, Mic, Walls, Tool Options, Settings)
    egui::SidePanel::left("left_panel")
        .default_width(420.)
        .resizable(false)
        .show(ctx, |ui| {
            ui_state.tools_enabled = !ui.rect_contains_pointer(ui.available_rect_before_wrap())
                && !ui_state.render_abc_area;

            ui.spacing_mut().slider_width = 200.0;

            ui.add_space(3.);

            // Sources
            egui::ScrollArea::vertical()
                .id_source("side_scroll_area")
                .max_height(400.)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());

                    let binding = source_set.p1();
                    let selected_source = binding.iter().next();
                    let selected_source = if selected_source.is_some() {
                        selected_source.unwrap().1.id as i32
                    } else {
                        -1_i32
                    };

                    let mut binding = source_set.p0();
                    let mut source_vec = binding.iter_mut().collect::<Vec<_>>();
                    source_vec.sort_by_cached_key(|(_, source)| source.id);

                    source_vec.iter_mut().for_each(|(entity, ref mut source)| {
                        let collapse = egui::CollapsingHeader::new(format!("Source {}", source.id))
                            .open(if selected_source == source.id as i32 {
                                Some(true)
                            } else if ui_state.collapse_header {
                                Some(false)
                            } else {
                                None
                            })
                            .show(ui, |ui| {
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
                                        events.reset_ev.send(Reset::default());
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
                                        events.reset_ev.send(Reset::default());
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
                                            events.reset_ev.send(Reset::default());
                                        }
                                        if ui
                                            .add(
                                                egui::Slider::new(amplitude, 0.0..=25.0)
                                                    .text("Amplitude"),
                                            )
                                            .changed()
                                        {
                                            events.reset_ev.send(Reset::default());
                                        }
                                        if ui
                                            .add(
                                                egui::Slider::new(phase, 0.0..=360.0)
                                                    .text("Phase (°)"),
                                            )
                                            .changed()
                                        {
                                            events.reset_ev.send(Reset::default());
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
                                            events.reset_ev.send(Reset::default());
                                        }
                                        if ui
                                            .add(
                                                egui::Slider::new(amplitude, 0.0..=25.0)
                                                    .text("Amplitude"),
                                            )
                                            .changed()
                                        {
                                            events.reset_ev.send(Reset::default());
                                        }
                                        if ui
                                            .add(
                                                egui::Slider::new(phase, 0.0..=360.0)
                                                    .text("Phase (°)"),
                                            )
                                            .changed()
                                        {
                                            events.reset_ev.send(Reset::default());
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
                                            events.reset_ev.send(Reset::default());
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

                    if !source_set.p0().is_empty() {
                        ui.separator();
                    }

                    // Microphones

                    let binding = mic_set.p1();
                    let selected_mic = binding.iter().next();
                    let selected_mic = if selected_mic.is_some() {
                        selected_mic.unwrap().1.id as i32
                    } else {
                        -1_i32
                    };

                    let mut binding = mic_set.p0();
                    let mut mic_vec = binding.iter_mut().collect::<Vec<_>>();
                    mic_vec.sort_by_cached_key(|(_, mic)| mic.id);

                    mic_vec.iter_mut().for_each(|(entity, ref mut mic)| {
                        let collapse =
                            egui::CollapsingHeader::new(format!("Microphone {}", mic.id))
                                .open(if selected_mic == mic.id as i32 {
                                    Some(true)
                                } else if ui_state.collapse_header {
                                    Some(false)
                                } else {
                                    None
                                })
                                .show(ui, |ui| {
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

                    if !mic_set.p0().is_empty() {
                        ui.separator();
                    }

                    // Rect Walls

                    let binding = rect_wall_set.p1();
                    let selected_rect_wall = binding.iter().next();
                    let selected_rect_wall = if selected_rect_wall.is_some() {
                        selected_rect_wall.unwrap().1.id as i32
                    } else {
                        -1_i32
                    };

                    let mut rect_binding = rect_wall_set.p0();
                    let mut wall_vec = rect_binding.iter_mut().collect::<Vec<_>>();
                    wall_vec.sort_by_cached_key(|(_, wall)| wall.id);

                    wall_vec.iter_mut().for_each(|(entity, ref mut wall)| {
                        let collapse =
                            egui::CollapsingHeader::new(format!("Rectangular Wall {}", wall.id))
                                .open(if selected_rect_wall == wall.id as i32 {
                                    Some(true)
                                } else if ui_state.collapse_header {
                                    Some(false)
                                } else {
                                    None
                                })
                                .show(ui, |ui| {
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
                                            events.reset_ev.send(Reset::default());
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
                                            events.reset_ev.send(Reset::default());
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
                                            events.reset_ev.send(Reset::default());
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
                                            events.reset_ev.send(Reset::default());
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
                                            egui::Slider::new(
                                                &mut wall.reflection_factor,
                                                0.01..=1.0,
                                            )
                                            .text("Wall Reflection Factor"),
                                        )
                                        .changed()
                                    {
                                        events.reset_ev.send(Reset::default());
                                    }

                                    if ui.checkbox(&mut wall.is_hollow, "Hollow").changed() {
                                        events.wall_update_ev.send(UpdateWalls);
                                        events.reset_ev.send(Reset::default());
                                    };

                                    if ui
                                        .add(egui::Button::new("Delete").fill(Color32::DARK_RED))
                                        .clicked()
                                    {
                                        commands.entity(*entity).despawn();
                                        events.wall_update_ev.send(UpdateWalls);
                                        events.reset_ev.send(Reset::default());
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

                    // Circ Walls

                    let binding = circ_wall_set.p1();
                    let selected_circ_wall = binding.iter().next();
                    let selected_circ_wall = if selected_circ_wall.is_some() {
                        selected_circ_wall.unwrap().1.id as i32
                    } else {
                        -1_i32
                    };

                    let mut circ_binding = circ_wall_set.p0();
                    let mut wall_vec = circ_binding.iter_mut().collect::<Vec<_>>();
                    wall_vec.sort_by_cached_key(|(_, wall)| wall.id);

                    wall_vec.iter_mut().for_each(|(entity, ref mut wall)| {
                        let collapse =
                            egui::CollapsingHeader::new(format!("Circular Wall {}", wall.id))
                                .open(if selected_circ_wall == wall.id as i32 {
                                    Some(true)
                                } else if ui_state.collapse_header {
                                    Some(false)
                                } else {
                                    None
                                })
                                .show(ui, |ui| {
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
                                            events.reset_ev.send(Reset::default());
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
                                            events.reset_ev.send(Reset::default());
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
                                            events.reset_ev.send(Reset::default());
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
                                                    0f32..=180f32,
                                                )
                                                .text("Open Circle Arc"),
                                            )
                                            .changed()
                                        {
                                            commands.entity(*entity).try_insert(WResize::Menu);
                                            events.reset_ev.send(Reset::default());
                                        }

                                        if ui
                                            .add(
                                                egui::Slider::new(
                                                    &mut wall.rotation_angle,
                                                    0f32..=360f32,
                                                )
                                                .text("Rotation Angle"),
                                            )
                                            .changed()
                                        {
                                            commands.entity(*entity).try_insert(WResize::Menu);
                                            events.reset_ev.send(Reset::default());
                                        }
                                    }

                                    if ui
                                        .add(
                                            egui::Slider::new(
                                                &mut wall.reflection_factor,
                                                0.01..=1.0,
                                            )
                                            .text("Wall Reflection Factor"),
                                        )
                                        .changed()
                                    {
                                        events.reset_ev.send(Reset::default());
                                    }

                                    if ui.checkbox(&mut wall.is_hollow, "Hollow Wall").changed() {
                                        events.wall_update_ev.send(UpdateWalls);
                                        events.reset_ev.send(Reset::default());
                                    };

                                    if ui
                                        .add(egui::Button::new("Delete").fill(Color32::DARK_RED))
                                        .clicked()
                                    {
                                        commands.entity(*entity).despawn();
                                        events.wall_update_ev.send(UpdateWalls);
                                        events.reset_ev.send(Reset::default());
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
            egui::TopBottomPanel::bottom("quick_settings_bottom_panel").show_inside(ui, |ui| {
                ui.add_space(3.);
                ui.heading("Quick Settings");
                ui.separator();

                ui.horizontal(|ui| {
                    if ui
                        .button(if ui_state.is_running { "Stop" } else { "Start" })
                        .clicked()
                    {
                        ui_state.is_running = !ui_state.is_running;
                    }

                    if ui.button("Reset").clicked() {
                        events.reset_ev.send(Reset { force: true });
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
                        events.wall_update_ev.send(UpdateWalls);
                    }

                    ui.checkbox(&mut ui_state.reset_on_change, "Reset on change");

                    if ui
                        .checkbox(&mut ui_state.show_plots, "Show Plots")
                        .clicked()
                    {
                        for (_, mut mic) in mic_set.p0().iter_mut() {
                            mic.clear();
                        }
                    }
                });

                ui.add_space(5.);

                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::Slider::new(&mut ui_state.framerate, 1f64..=500.)
                                .logarithmic(true),
                        )
                        .changed()
                    {
                        if ui_state.read_epilepsy_warning {
                            fixed_timestep.set_timestep_hz(ui_state.framerate);
                        } else {
                            ui_state.show_epilepsy_warning = true;
                            ui_state.framerate = 60.;
                        }
                    }
                    ui.add_space(5.);
                    ui.label("Simulation Frame Rate");
                });

                ui.add_space(5.);

                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Simulation Time: {:.5} ms",
                        sim_time.time_since_start * 1000.
                    ));

                    ui.add(egui::Separator::default().vertical());
                    ui.label(format!(
                        "FPS: {:.1}",
                        diagnostics
                            .get(&FrameTimeDiagnosticsPlugin::FPS)
                            .and_then(|fps| fps.smoothed())
                            .unwrap_or(0.0)
                    ));
                });

                ui.add_space(5.);
            });

            // Tool Options
            egui::TopBottomPanel::bottom("tool_options_panel").show_inside(ui, |ui| {
                ui.add_space(3.);
                ui.heading("Tool Options");
                ui.separator();

                ui.set_enabled(!ui_state.render_abc_area);

                match ui_state.current_tool {
                    ToolType::Place(_) => {
                        egui::ComboBox::from_label("Select Object to Place")
                            .selected_text(format!("{}", ui_state.cur_place_type))
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.selectable_value(
                                    &mut ui_state.cur_place_type,
                                    PlaceType::Source,
                                    "Source",
                                );
                                ui.selectable_value(
                                    &mut ui_state.cur_place_type,
                                    PlaceType::Mic,
                                    "Microphone",
                                );
                                ui.selectable_value(
                                    &mut ui_state.cur_place_type,
                                    PlaceType::RectWall,
                                    "Rectangle Wall",
                                );
                                ui.selectable_value(
                                    &mut ui_state.cur_place_type,
                                    PlaceType::CircWall,
                                    "Circular Wall",
                                );
                            });

                        if matches!(
                            ui_state.cur_place_type,
                            PlaceType::RectWall | PlaceType::CircWall
                        ) {
                            ui.add(
                                egui::Slider::new(&mut ui_state.wall_reflection_factor, 0.0..=1.0)
                                    .text("Wall Reflection Factor"),
                            );
                            ui.checkbox(&mut ui_state.wall_is_hollow, "Hollow");
                        }
                        ui_state.current_tool = ToolType::Place(ui_state.cur_place_type);
                    }
                    _ => {
                        ui.add_space(10.);
                        ui.vertical_centered(|ui| {
                            ui.label("Select another tool to see its options")
                        });
                    }
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
                let mut binding = mic_set.p0();

                let mut mics = binding
                    .iter_mut()
                    .map(|(_, mic)| mic.into_inner())
                    .collect::<Vec<_>>();
                mics.sort_by_cached_key(|mic| mic.id);

                let mut pb = pixel_buffers.iter_mut().nth(1).expect("two pixel buffers");

                let mut style = egui_dock::Style::from_egui(ui.style());
                style.tab_bar.bg_fill = Color32::from_rgb(27, 27, 27);

                egui_dock::DockArea::new(&mut dock_state.tree)
                    .allowed_splits(egui_dock::AllowedSplits::None)
                    .draggable_tabs(false)
                    .style(style)
                    .show_inside(
                        ui,
                        &mut PlotTabs::new(
                            &mut mics,
                            &mut pb,
                            // &mut fft_mic,
                            &mut commands.reborrow(),
                            grid.delta_t,
                            sim_time.time_since_start as f64,
                            time.delta_seconds_f64(),
                            &mut ui_state,
                        ),
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
            let select_icon = &images[0];
            let place_icon = &images[1];
            let move_icon = &images[2];
            let resize_wall_icon = &images[3];

            if ui
                .add(
                    egui::Button::image(
                        egui::Image::new(select_icon.clone())
                            .fit_to_exact_size(Vec2::new(24., 24.)),
                    )
                    .fill(if matches!(ui_state.current_tool, ToolType::Select) {
                        Color32::DARK_GRAY
                    } else {
                        Color32::TRANSPARENT
                    })
                    .min_size(Vec2::new(0., 35.)),
                )
                .on_hover_text(format!("{}", ToolType::Select))
                .clicked()
            {
                ui_state.current_tool = ToolType::Select;
            }
            ui.add_space(4.);

            if ui
                .add(
                    egui::Button::image(
                        // TODO: change image depending on cur_place_type??
                        egui::Image::new(place_icon.clone()).fit_to_exact_size(Vec2::new(24., 24.)),
                    )
                    .fill(if matches!(ui_state.current_tool, ToolType::Place(..)) {
                        Color32::DARK_GRAY
                    } else {
                        Color32::TRANSPARENT
                    })
                    .min_size(Vec2::new(0., 35.)),
                )
                .on_hover_text(format!("{}", ToolType::Place(PlaceType::Source)))
                .clicked()
            {
                ui_state.current_tool = ToolType::Place(ui_state.cur_place_type);
            }
            ui.add_space(4.);

            if ui
                .add(
                    egui::Button::image(
                        egui::Image::new(move_icon.clone()).fit_to_exact_size(Vec2::new(24., 24.)),
                    )
                    .fill(if matches!(ui_state.current_tool, ToolType::Move) {
                        Color32::DARK_GRAY
                    } else {
                        Color32::TRANSPARENT
                    })
                    .min_size(Vec2::new(0., 35.)),
                )
                .on_hover_text(format!("{}", ToolType::Move))
                .clicked()
            {
                ui_state.current_tool = ToolType::Move;
            }
            ui.add_space(4.);

            if ui
                .add(
                    egui::Button::image(
                        egui::Image::new(resize_wall_icon.clone())
                            .fit_to_exact_size(Vec2::new(24., 24.)),
                    )
                    .fill(if matches!(ui_state.current_tool, ToolType::ResizeWall) {
                        Color32::DARK_GRAY
                    } else {
                        Color32::TRANSPARENT
                    })
                    .min_size(Vec2::new(0., 35.)),
                )
                .on_hover_text(format!("{}", ToolType::ResizeWall))
                .clicked()
            {
                ui_state.current_tool = ToolType::ResizeWall;
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
                //menu gizmos
                if !ui_state.tools_enabled {
                    for (_, wall) in rect_wall_set.p2().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ToolType::Move,
                            true,
                            &ui_state.image_rect,
                            None,
                            ui_state.delta_l,
                        );
                    }
                    for (_, wall) in circ_wall_set.p2().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ToolType::Move,
                            true,
                            &ui_state.image_rect,
                            None,
                            ui_state.delta_l,
                        );
                    }
                    // all mics
                    for (_, mic) in mic_set.p2().iter() {
                        mic.draw_gizmo(
                            painter,
                            &ToolType::Move,
                            true,
                            &ui_state.image_rect,
                            Some(&format!("{}", mic.id)),
                            ui_state.delta_l,
                        );
                    }
                    // all sources
                    for (_, source) in source_set.p2().iter() {
                        source.draw_gizmo(
                            painter,
                            &ToolType::Move,
                            true,
                            &ui_state.image_rect,
                            Some(&format!("{}", source.id)),
                            ui_state.delta_l,
                        );
                    }
                } else {
                    // TODO: drawing selected gizmos on top means that the text is also drawn twice
                    // Tool specific gizmos
                    // all rect walls
                    for wall in rect_wall_set.p3().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            false,
                            &ui_state.image_rect,
                            None,
                            ui_state.delta_l,
                        );
                    }
                    // selected rect walls
                    for (_, wall) in rect_wall_set.p1().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            true,
                            &ui_state.image_rect,
                            None,
                            ui_state.delta_l,
                        );
                    }
                    // all circ walls
                    for wall in circ_wall_set.p3().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            false,
                            &ui_state.image_rect,
                            None,
                            ui_state.delta_l,
                        );
                    }
                    // selected circ walls
                    for (_, wall) in circ_wall_set.p1().iter() {
                        wall.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            true,
                            &ui_state.image_rect,
                            None,
                            ui_state.delta_l,
                        );
                    }
                    // all mics
                    for mic in mic_set.p3().iter() {
                        mic.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            false,
                            &ui_state.image_rect,
                            Some(&format!("{}", mic.id)),
                            ui_state.delta_l,
                        );
                    }
                    // selected mics
                    for (_, mic) in mic_set.p1().iter() {
                        mic.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            true,
                            &ui_state.image_rect,
                            Some(&format!("{}", mic.id)),
                            ui_state.delta_l,
                        );
                    }
                    // all sources
                    for source in source_set.p3().iter() {
                        source.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            false,
                            &ui_state.image_rect,
                            Some(&format!("{}", source.id)),
                            ui_state.delta_l,
                        );
                    }
                    // selected sources
                    for (_, source) in source_set.p1().iter() {
                        source.draw_gizmo(
                            painter,
                            &ui_state.current_tool,
                            true,
                            &ui_state.image_rect,
                            Some(&format!("{}", source.id)),
                            ui_state.delta_l,
                        );
                    }
                }
            }
        });

    ui_state.collapse_header = false;
}
