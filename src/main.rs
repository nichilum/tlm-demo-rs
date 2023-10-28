use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_pixel_buffer::prelude::*;
use na::{vector, Vector4};
use rayon::prelude::*;

extern crate nalgebra as na;

const SIMULATION_WIDTH: u32 = 700;
const SIMULATION_HEIGHT: u32 = 700;
const PIXEL_SIZE: u32 = 1;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum NodeType {
    #[default]
    Normal,
    Wall,
    End,
    Source,
}

#[derive(Debug, Default, Clone)]
struct Node {
    current: Vector4<f64>,
    next: Vector4<f64>,
    node_type: NodeType,
}

impl Node {
    fn get_pressure(&self) -> f64 {
        (self.current.x + self.current.y + self.current.z + self.current.w) / 2.
    }

    fn update(&mut self) {
        self.current = self.next;
    }

    fn set_next(&mut self, next: Vector4<f64>) {
        self.next = next;
    }

    fn calc(
        &self,
        left: Option<&Node>,
        right: Option<&Node>,
        top: Option<&Node>,
        bottom: Option<&Node>,
    ) -> Vector4<f64> {
        match (left, right, top, bottom) {
            (Some(left), Some(right), Some(top), Some(bottom)) => {
                vector![
                    0.5 * (-bottom.current.z + left.current.w + top.current.x + right.current.y),
                    0.5 * (bottom.current.z - left.current.w + top.current.x + right.current.y),
                    0.5 * (bottom.current.z + left.current.w - top.current.x + right.current.y),
                    0.5 * (bottom.current.z + left.current.w + top.current.x - right.current.y),
                ]
            }
            _ => Vector4::zeros(),
        }
    }
}

#[derive(Debug, Resource)]
struct Grid(Vec<Node>);

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<&Node> {
        if x > SIMULATION_WIDTH as i32 - 1 || y > SIMULATION_HEIGHT as i32 - 1 || x < 0 || y < 0 {
            return None;
        }

        Some(&self.0[(y * SIMULATION_WIDTH as i32 + x) as usize])
    }

    fn get_mut(&mut self, x: u32, y: u32) -> &mut Node {
        &mut self.0[(y * SIMULATION_WIDTH + x) as usize]
    }

    fn set(&mut self, x: u32, y: u32, node: Node) {
        self.0[(y * SIMULATION_WIDTH + x) as usize] = node;
    }
}

#[derive(Resource)]
struct GradientResource(colorgrad::Gradient);

fn main() {
    let size: PixelBufferSize = PixelBufferSize {
        size: UVec2::new(SIMULATION_WIDTH, SIMULATION_HEIGHT),
        pixel_size: UVec2::new(PIXEL_SIZE, PIXEL_SIZE),
    };

    let mut grid = Grid(vec![
        Node::default();
        (SIMULATION_WIDTH * SIMULATION_HEIGHT) as usize
    ]);

    for x in 0..SIMULATION_WIDTH {
        grid.set(
            x,
            SIMULATION_HEIGHT / 2,
            Node {
                node_type: NodeType::Source,
                ..Default::default()
            },
        );
    }

    // grid.set(
    //     SIMULATION_WIDTH / 2,
    //     SIMULATION_HEIGHT / 2,
    //     Node {
    //         node_type: NodeType::Source,
    //         ..Default::default()
    //     },
    // );

    // let source = grid.get_mut(SIMULATION_WIDTH / 2, SIMULATION_HEIGHT / 2);
    // source.current = vector![1., 1., 1., 1.];

    let gradient = GradientResource(colorgrad::magma());

    App::new()
        .add_plugins((
            DefaultPlugins,
            PixelBufferPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(grid)
        .insert_resource(gradient)
        .add_systems(Startup, pixel_buffer_setup(size))
        .add_systems(Update, (update_nodes_system, draw_colors_system))
        .run();
}

fn draw_colors_system(mut pb: QueryPixelBuffer, grid: Res<Grid>, _gradient: Res<GradientResource>) {
    //TODO: replace bevy_pixel_buffer with bevy_pixels for gpu rendering?
    pb.frame().per_pixel_par(|coords, _| {
        let p = grid
            .get(coords.x as i32, coords.y as i32)
            .expect("grid matches canvas size")
            .get_pressure();
        // let color = gradient.0.at(p);

        Pixel {
            r: (p * 255.) as u8,
            g: (p * 255.) as u8,
            b: (p * 255.) as u8,
            a: 255,
        }
    })
}

fn index_to_coords(index: usize) -> (i32, i32) {
    let x = index % SIMULATION_WIDTH as usize;
    let y = index / SIMULATION_WIDTH as usize;

    (x as i32, y as i32)
}

fn update_nodes_system(mut grid: ResMut<Grid>, time: Res<Time>) {
    //TODO: make this parallel (without borrowing issues on grid)
    (0..(SIMULATION_HEIGHT * SIMULATION_WIDTH) as usize).for_each(|i| {
        let (x, y) = index_to_coords(i);

        let left = grid.get(x - 1, y);
        let right = grid.get(x + 1, y);
        let top = grid.get(x, y - 1);
        let bottom = grid.get(x, y + 1);

        let node = grid.0[i].clone();

        grid.0[i].next = node.calc(left, right, top, bottom);
        if let NodeType::Source = node.node_type {
            sin_source(time.elapsed_seconds_f64(), x as u32, y as u32, &mut grid);
        }
    });

    grid.0.par_iter_mut().for_each(|node| {
        node.update();
    });
}

fn sin_source(t: f64, x: u32, y: u32, grid: &mut ResMut<Grid>) {
    let source = grid.get_mut(x, y);
    let sin = (10. * t).sin();
    source.set_next(vector![sin, sin, sin, sin]);
}
