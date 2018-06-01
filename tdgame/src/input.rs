use {
    cgmath::{Point2, Vector3, Point3, InnerSpace},
    ggez::{
        event::{Keycode},
        Context,
    },

    lagato::{
        camera::{OrbitingCamera, RenderCamera},
        grid::{Voxels},
        DirectionalInput, rotate_vector
    },
    blockengine::{cast_ray},
    blockengine_rendering::{Mesh, Object},
};

pub struct InputHandler {
    directional: DirectionalInput,
    cursor_position: Point2<i32>,
    pointer_object: usize,
}

impl InputHandler {
    pub fn new(ctx: &mut Context, objects: &mut Vec<Object>) -> Self {
        // Create the object we'll use to show where the cursor is pointing
        objects.push(Object {
            position: Point3::new(0.0, 0.0, 0.0),
            mesh: Mesh::cube(ctx),
        });
        let pointer_object = objects.len() - 1;

        InputHandler {
            directional: DirectionalInput::new(),
            cursor_position: Point2::new(0, 0),
            pointer_object,
        }
    }

    pub fn update(
        &mut self,
        world: &Voxels<bool>, objects: &mut Vec<Object>,
        camera: &mut OrbitingCamera, last_camera: &RenderCamera,
        delta: f32,
    ) {
        let ray = last_camera.pixel_to_ray(self.cursor_position);
        let result = cast_ray(&ray, 1000.0, &world);
        if let Some((position, normal)) = result {
            let place_position = position.cast().unwrap() + normal;
            objects[self.pointer_object].position = place_position;
        }

        // Calculate which direction we need to move based on the current input
        let mut input = self.directional.to_vector();
        input = rotate_vector(input, -camera.yaw);

        if input.x != 0.0 || input.y != 0.0 {
            input = input.normalize();
        }

        const SPEED: f32 = 10.0;
        camera.focus += Vector3::new(input.x, 0.0, input.y) * delta * SPEED;
    }

    pub fn key_down_event(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::S => self.directional.backward = true,
            Keycode::W => self.directional.forward = true,
            Keycode::A => self.directional.left = true,
            Keycode::D => self.directional.right = true,
            _ => {}
        }
    }

    pub fn key_up_event(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::S => self.directional.backward = false,
            Keycode::W => self.directional.forward = false,
            Keycode::A => self.directional.left = false,
            Keycode::D => self.directional.right = false,
            _ => {}
        }
    }

    pub fn mouse_motion_event(&mut self, x: i32, y: i32) {
        self.cursor_position = Point2::new(x, y);
    }
}
