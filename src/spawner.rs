use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            }
        )
    );
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let render = match rng.range(0, 4) {
        0 => Render{glyph: to_cp437('E'), color: ColorPair::new(GRAY, BLACK)},
        1 => Render{glyph: to_cp437('O'), color: ColorPair::new(GREENYELLOW, BLACK)},
        2 => Render{glyph: to_cp437('o'), color: ColorPair::new(YELLOW, BLACK)},
        _ => Render{glyph: to_cp437('g'), color: ColorPair::new(GREEN, BLACK)},
    };

    ecs.push(
        (Enemy, pos, render)
    );
}