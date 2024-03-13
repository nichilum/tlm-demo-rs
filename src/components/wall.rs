use std::cmp::Ordering;

use bevy::prelude::*;
use egui::epaint::CircleShape;
use egui::{Color32, Pos2, Rect};
use serde::{Deserialize, Serialize};

use super::gizmo::GizmoComponent;
use crate::math::constants::{SIMULATION_HEIGHT, SIMULATION_WIDTH};
use crate::math::rect::WRect;
use crate::math::transformations::grid_to_image;
use crate::ui::state::ToolType;

#[derive(Debug, Default, Clone)]
pub struct WallCell {
    pub is_wall: bool,
    pub reflection_factor: f32,
}

#[derive(Component, PartialEq)]
pub enum WResize {
    Draw,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    // Top,
    // Right,
    // Bottom,
    // Left,
    Radius,
}

pub trait Wall: Sync + Send {
    fn contains(&self, x: u32, y: u32) -> bool;

    fn edge_contains(&self, x: u32, y: u32) -> bool;

    /// If width or height equals one, the wall can be deleted
    fn is_deletable(&self) -> bool;

    fn set_center(&mut self, x: u32, y: u32);

    fn get_center(&self) -> UVec2;

    fn get_reflection_factor(&self) -> f32;

    fn get_resize_point(&self, resize_type: &WResize) -> UVec2;

    fn resize(&mut self, resize_type: &WResize, x: u32, y: u32);
}

#[derive(Component, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct RectWall {
    // between 0 and SIM_WIDTH
    // between 0 and SIM_HEIGHT
    pub rect: WRect,
    pub is_hollow: bool,
    pub reflection_factor: f32,
    pub id: usize,
}

impl Wall for RectWall {
    fn get_center(&self) -> UVec2 {
        self.rect.center()
    }

    fn get_resize_point(&self, resize_type: &WResize) -> UVec2 {
        debug_assert!(
            resize_type != &WResize::Radius,
            "RectWall cannot be resized with WResize::Radius"
        );
        match resize_type {
            WResize::Draw => self.rect.max,
            WResize::TopLeft => self.rect.min,
            WResize::TopRight => UVec2::new(self.rect.max.x, self.rect.min.y),
            WResize::BottomRight => self.rect.max,
            WResize::BottomLeft => UVec2::new(self.rect.min.x, self.rect.max.y),
            WResize::Radius => unreachable!(),
        }
    }

    fn contains(&self, x: u32, y: u32) -> bool {
        if self.is_hollow {
            return self.edge_contains(x, y);
        }
        x >= self.rect.min.x && x <= self.rect.max.x && y >= self.rect.min.y && y <= self.rect.max.y
    }

    fn edge_contains(&self, x: u32, y: u32) -> bool {
        ((x == self.rect.min.x || x == self.rect.max.x)
            && (y >= self.rect.min.y && y <= self.rect.max.y))
            || ((y == self.rect.min.y || y == self.rect.max.y)
                && (x >= self.rect.min.x && x <= self.rect.max.x))
    }

    fn is_deletable(&self) -> bool {
        self.rect.width() == 1 || self.rect.height() == 1
    }

    fn set_center(&mut self, x: u32, y: u32) {
        let current_center = self.rect.center();

        let mut x_offset = x as i32 - current_center.x as i32;
        let mut y_offset = y as i32 - current_center.y as i32;

        match x_offset.cmp(&0) {
            Ordering::Less => {
                x_offset = if x_offset.abs() > self.rect.min.x as i32 {
                    self.rect.min.x as i32
                } else {
                    x_offset
                };
                self.rect.min.x -= x_offset.unsigned_abs();
                self.rect.max.x -= x_offset.unsigned_abs();
            }
            Ordering::Greater => {
                // minus 1 because wall-bounds are inclusive
                x_offset = if x_offset > SIMULATION_WIDTH as i32 - self.rect.max.x as i32 - 1 {
                    SIMULATION_WIDTH as i32 - self.rect.max.x as i32 - 1
                } else {
                    x_offset
                };
                self.rect.min.x += x_offset as u32;
                self.rect.max.x += x_offset as u32;
            }
            _ => {}
        }

        match y_offset.cmp(&0) {
            Ordering::Less => {
                y_offset = if y_offset.abs() > self.rect.min.y as i32 {
                    self.rect.min.y as i32
                } else {
                    y_offset
                };
                self.rect.min.y -= y_offset.unsigned_abs();
                self.rect.max.y -= y_offset.unsigned_abs();
            }
            Ordering::Greater => {
                // minus 1 because wall-bounds are inclusive
                y_offset = if y_offset > SIMULATION_HEIGHT as i32 - self.rect.max.y as i32 - 1 {
                    SIMULATION_HEIGHT as i32 - self.rect.max.y as i32 - 1
                } else {
                    y_offset
                };
                self.rect.min.y += y_offset as u32;
                self.rect.max.y += y_offset as u32;
            }
            _ => {}
        }
    }

    fn get_reflection_factor(&self) -> f32 {
        self.reflection_factor
    }

    fn resize(&mut self, resize_type: &WResize, mut x: u32, mut y: u32) {
        debug_assert!(
            resize_type != &WResize::Radius,
            "RectWall cannot be resized with WResize::Radius"
        );
        match resize_type {
            WResize::TopLeft => {
                if x > self.rect.max.x - 1 {
                    x = self.rect.max.x - 1;
                }
                if y > self.rect.max.y - 1 {
                    y = self.rect.max.y - 1;
                }

                self.rect.min.x = x;
                self.rect.min.y = y;
            }
            WResize::TopRight => {
                if x < self.rect.min.x + 1 {
                    x = self.rect.min.x + 1;
                }
                if y > self.rect.max.y - 1 {
                    y = self.rect.max.y - 1;
                }

                self.rect.max.x = x;
                self.rect.min.y = y;
            }
            WResize::BottomRight => {
                // make sure x and y are never less than min
                // wall is always 2 pixel tall and wide
                if x < self.rect.min.x + 1 {
                    x = self.rect.min.x + 1;
                }
                if y < self.rect.min.y + 1 {
                    y = self.rect.min.y + 1;
                }

                self.rect.max.x = x;
                self.rect.max.y = y;
            }
            WResize::BottomLeft => {
                if x > self.rect.max.x - 1 {
                    x = self.rect.max.x - 1;
                }
                if y < self.rect.min.y + 1 {
                    y = self.rect.min.y + 1;
                }

                self.rect.min.x = x;
                self.rect.max.y = y;
            }
            WResize::Draw => {
                // I want to be able to drag into each quadrant here
                if x < self.rect.min.x {
                    x = self.rect.min.x;
                }
                if y < self.rect.min.y {
                    y = self.rect.min.y;
                }

                self.rect.max.x = x;
                self.rect.max.y = y;
            }
            WResize::Radius => unreachable!(),
        }
    }
}

impl GizmoComponent for RectWall {
    fn get_gizmo_positions(&self, tool_type: &ToolType) -> Vec<Pos2> {
        match tool_type {
            ToolType::ResizeWall => {
                let top_left = Pos2 {
                    x: self.rect.min.x as f32,
                    y: self.rect.min.y as f32,
                };
                let top_right = Pos2 {
                    x: self.rect.max.x as f32,
                    y: self.rect.min.y as f32,
                };
                let bottom_left = Pos2 {
                    x: self.rect.min.x as f32,
                    y: self.rect.max.y as f32,
                };
                let bottom_right = Pos2 {
                    x: self.rect.max.x as f32,
                    y: self.rect.max.y as f32,
                };

                vec![top_left, top_right, bottom_left, bottom_right]
            }
            ToolType::MoveWall => {
                let center = self.get_center();
                vec![Pos2 {
                    x: center.x as f32,
                    y: center.y as f32,
                }]
            }
            _ => {
                unreachable!()
            }
        }
    }

    fn draw_gizmo(
        &self,
        painter: &egui::Painter,
        tool_type: &ToolType,
        highlight: bool,
        image_rect: &Rect,
    ) {
        match tool_type {
            ToolType::ResizeWall | ToolType::MoveWall => {
                for pos in self.get_gizmo_positions(tool_type) {
                    painter.add(egui::Shape::Circle(CircleShape::filled(
                        grid_to_image(pos, image_rect),
                        if highlight { 10. } else { 5. },
                        Color32::LIGHT_RED,
                    )));
                }
            }
            _ => {}
        }
    }
}

impl RectWall {
    pub fn new(
        x0: u32,
        y0: u32,
        x1: u32,
        y1: u32,
        is_hollow: bool,
        reflection_factor: f32,
        id: usize,
    ) -> Self {
        RectWall {
            rect: WRect::new(x0, y0, x1, y1),
            is_hollow,
            reflection_factor,
            id,
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct CircWall {
    pub center: UVec2,
    /// Radius excludes center point
    pub radius: u32,
    pub is_hollow: bool,
    pub reflection_factor: f32,
    pub id: usize,
}

impl Wall for CircWall {
    fn get_center(&self) -> UVec2 {
        self.center
    }

    fn get_resize_point(&self, resize_type: &WResize) -> UVec2 {
        match resize_type {
            WResize::Radius => UVec2 {
                // here I want to implement offset
                // either left or right depending on radius size
                x: self.center.x,
                y: self.center.y,
            },
            _ => {
                unreachable!()
            }
        }
    }

    fn contains(&self, x: u32, y: u32) -> bool {
        if self.is_hollow {
            return self.edge_contains(x, y);
        }
        // very crude implementation
        let r_squared = self.radius * self.radius;

        (self.center.x - x).pow(2) + (self.center.y - y).pow(2) <= r_squared
    }

    fn edge_contains(&self, x: u32, y: u32) -> bool {
        let r_squared = self.radius * self.radius;
        // This is legit stupid, but not the focus right now
        (self.center.x - x).pow(2) + (self.center.y - y).pow(2) <= r_squared
            && (self.center.x - x).pow(2) + (self.center.y - y).pow(2)
                >= (r_squared as f32 * 0.8) as u32
    }

    fn is_deletable(&self) -> bool {
        // self.radius == 0
        false
    }

    fn set_center(&mut self, x: u32, y: u32) {
        self.center.x = x;
        self.center.y = y;
    }

    fn get_reflection_factor(&self) -> f32 {
        self.reflection_factor
    }

    fn resize(&mut self, resize_type: &WResize, x: u32, y: u32) {
        match resize_type {
            WResize::Radius => {
                // scale only on x-axis from center.x
                let x_offset = self.center.x as i32 - x as i32;
                self.radius = x_offset.abs() as u32;
            }
            _ => {
                panic!("Circular walls cannot be resized by radius.");
            }
        }
    }
}

impl CircWall {
    pub fn new(
        x: u32,
        y: u32,
        radius: u32,
        is_hollow: bool,
        reflection_factor: f32,
        id: usize,
    ) -> Self {
        CircWall {
            center: UVec2 { x, y },
            radius,
            is_hollow,
            reflection_factor,
            id,
        }
    }
}
