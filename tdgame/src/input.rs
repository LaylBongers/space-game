use {
    cgmath::{Point2, Vector3, Point3, InnerSpace},
    ggez::{
        event::{Keycode, MouseButton},
        Context, GameResult,
    },

    lagato::{
        camera::{OrbitingCamera, RenderCamera},
        DirectionalInput, rotate_vector
    },
    blockengine::{cast_ray},
    blockengine_rendering::{Mesh, Object, Texture, triangulate_voxels},

    World,
};

pub struct InputHandler {
    directional: DirectionalInput,
    cursor_position: Point2<i32>,
    pointer_object: usize,

    pointer_position: Option<Point3<i32>>,
    pointer_position_with_normal: Option<Point3<i32>>,
}

impl InputHandler {
    pub fn new(ctx: &mut Context, objects: &mut Vec<Object>) -> GameResult<Self> {
        // Create the object we'll use to show where the cursor is pointing
        objects.push(Object {
            position: Point3::new(0.0, 0.0, 0.0),
            visible: false,

            mesh: Mesh::cube(ctx),
            texture: Texture::load(ctx, "/pointer.png")?,
        });
        let pointer_object = objects.len() - 1;

        Ok(InputHandler {
            directional: DirectionalInput::new(),
            cursor_position: Point2::new(0, 0),
            pointer_object,

            pointer_position: None,
            pointer_position_with_normal: None,
        })
    }

    pub fn update(
        &mut self,
        world: &World, objects: &mut Vec<Object>,
        camera: &mut OrbitingCamera, last_camera: &RenderCamera,
        delta: f32,
    ) {
        let ray = last_camera.pixel_to_ray(self.cursor_position);
        let result = cast_ray(&ray, 1000.0, &world.voxels);
        if let Some((position, normal)) = result {
            self.pointer_position = Some(position);
            self.pointer_position_with_normal = Some(position + normal);

            let place_position = position.cast().unwrap() + normal.cast().unwrap();
            let obj = &mut objects[self.pointer_object];
            obj.position = place_position;
            obj.visible = true;
        } else {
            self.pointer_position = None;
            self.pointer_position_with_normal = None;
            objects[self.pointer_object].visible = false;
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

    pub fn mouse_button_down_event(&mut self, _button: MouseButton) {
    }

    pub fn mouse_button_up_event(
        &mut self, ctx: &mut Context,
        button: MouseButton, world: &mut World, objects: &mut Vec<Object>
    ) {
        let mut world_changed = false;

        match button {
            MouseButton::Left => {
                if let Some(pointer_position) = self.pointer_position_with_normal {
                    if let Ok(voxel) = world.voxels.get_mut(pointer_position) {
                        *voxel = true;
                        world_changed = true;
                    }
                }
            },
            MouseButton::Right => {
                if let Some(pointer_position) = self.pointer_position {
                    if let Ok(voxel) = world.voxels.get_mut(pointer_position) {
                        *voxel = false;
                        world_changed = true;
                    }
                }
            },
            _ => {},
        }

        if world_changed {
            objects[world.object].mesh =
                Mesh::new(ctx, &triangulate_voxels(&world.voxels));
        }
    }

    pub fn mouse_motion_event(&mut self, position: Point2<i32>) {
        self.cursor_position = position;
    }
}
