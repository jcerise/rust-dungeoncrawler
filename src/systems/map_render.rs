use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    // Gather a list of all entities currently on the map. This will be used to determine
    // whether or not a map tile should be drawn or not, as otherwise, the entity glyph gets
    // incorrectly composed onto the map glyph beneath
    let mut entity_points: Vec<Point> = Vec::new();
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            entity_points.push(*pos)
        });

    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x .. camera.right_x {
            let point = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            if map.point_in_bounds(point) && !entity_points.contains(&point){
                let index = map_index(x, y);
                let glyph = match map.tiles[index] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                let new_point = point - offset;

                draw_batch.set(
                    new_point,
                    ColorPair::new(WHITE, BLACK),
                    glyph
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch Error - map");
}