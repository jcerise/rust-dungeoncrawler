mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::map::*;
    pub use crate::systems::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);

        // Spawn a monster/enemy entity in each room, except the room the player starts in
        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler()
        }
    }
}

impl GameState for State {
    fn tick (&mut self, context: &mut BTerm) {
        // Clear the consoles
        context.set_active_console(0);
        context.cls();
        context.set_active_console(1);
        context.cls();

        // TODO: Execute systems and render
        self.resources.insert(context.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);

        render_draw_buffer(context).expect("Render Error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("DungeonCrawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(16, 16)
        .with_resource_path("resources/")
        .with_font("cheepicus_16x16.png", 16, 16)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "cheepicus_16x16.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "cheepicus_16x16.png")
        .build()?;

    main_loop(context, State::new())
}
