use {
    cgmath::{Point2, Vector2},
    slog::{Logger},

    lagato::{grid::{Tiles}},

    object_class::{ObjectClasses},
    state::ship::{Unit, Tile, TaskQueue},
    Error,
};

#[derive(Deserialize, Serialize)]
pub struct Ship {
    units: Vec<Unit>,

    pub tiles: Tiles<Tile>,
    pub task_queue: TaskQueue,

    tiles_with_behaviors: Vec<Point2<i32>>,
    tiles_changed: bool,
}

impl Ship {
    pub fn empty(size: Vector2<i32>) -> Self {
        Ship {
            units: Vec::new(),

            tiles: Tiles::empty(size),
            task_queue: TaskQueue::new(),

            tiles_with_behaviors: Vec::new(),
            tiles_changed: false,
        }
    }

    pub fn starter(log: &Logger) -> Self {
        info!(log, "Creating starter ship");
        let mut ship = Ship::empty(Vector2::new(100, 100));
        for y in 47..53 {
            for x in 48..52 {
                ship.tiles.get_mut(Point2::new(x, y)).unwrap().floor = true;
            }
        }
        ship.add_unit(Unit::new(Point2::new(50.5, 50.5)));
        ship.add_unit(Unit::new(Point2::new(49.5, 49.5)));

        ship
    }

    pub fn units(&self) -> &Vec<Unit> {
        &self.units
    }

    pub fn add_unit(&mut self, unit: Unit) {
        self.units.push(unit);
    }

    pub fn update(
        &mut self, log: &Logger, object_classes: &ObjectClasses, delta: f32,
    ) -> Result<(), Error> {
        if self.handle_tiles_changed(object_classes)? {
            // Since the world has changed, we can mark all tasks as being possible again
            self.task_queue.clear_unreachable();
        }

        for i in &self.tiles_with_behaviors {
            let object = self.tiles.get_mut(*i)?
                .object.as_mut()
                    .expect("Found tile without object in tiles with behaviors");
            let behavior = object_classes.get(object.class)?
                .behavior.as_ref()
                    .expect("Found tile class without behavior in tiles with behaviors");

            behavior.update(object, delta);
        }

        for unit in &mut self.units {
            unit.update(log, object_classes, &mut self.tiles, &mut self.task_queue, delta)?;
        }

        self.task_queue.update(log);

        Ok(())
    }

    fn handle_tiles_changed(&mut self, object_classes: &ObjectClasses) -> Result<bool, Error> {
        Ok(if self.tiles_changed {
            // Find any tiles that ask for update events
            self.tiles_with_behaviors.clear();
            for position in self.tiles.iter_pos() {
                let tile = self.tiles.get(position)?;
                if let Some(ref object) = tile.object {
                    let class = object_classes.get(object.class)?;
                    if class.behavior.is_some() {
                        self.tiles_with_behaviors.push(position);
                    }
                }
            }

            self.tiles_changed = false;
            true
        } else {
            false
        })
    }
}
