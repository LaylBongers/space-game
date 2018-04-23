pub mod ship;
mod build;
mod camera;

pub use self::{
    build::{BuildState, BuildDrag, BuildChoice, normalize_area},
    camera::{Camera},
};

use {
    nalgebra::{Vector2, Point2},
    slog::{Logger},

    ObjectClasses,
    state::ship::{Ship},
};

#[derive(Deserialize, Serialize)]
pub struct GameState {
    pub build_input_state: BuildState,
    pub camera: Camera,
    pub ship: Ship,
}

impl GameState {
    pub fn new(log: &Logger) -> Self {
        // Set up the game world camera
        let mut camera = Camera::new(64, Vector2::new(1280, 720));
        camera.set_position(Point2::new(50.0, 50.0));

        // Create the starter ship
        let ship = Ship::starter(&log);

        GameState {
            build_input_state: BuildState {
                drag: BuildDrag::Hovering { position: None, },
                choice: BuildChoice::None,
            },
            camera,
            ship,
        }
    }

    pub fn update(&mut self, log: &Logger, delta: f32, object_classes: &ObjectClasses) {
        self.ship.update(log, delta, object_classes);
    }
}
