use crate::prelude::*;

pub struct Player {
    pub position: Point
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self {
            position
        }
    }

    /// Renders the player char to the screen at its current position
    pub fn render(&self, context: &mut BTerm, camera: &Camera) {
        context.set_active_console(2);
        context.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@')
        );
    }

    /// Updates the position of the player based on the directional key pressed by the user
    ///
    /// # Examples
    /// ```
    /// let position: Point = Point(1, 1);
    /// let player: Player = Player::new(position);
    /// player.update(Some(VirtualKeyCode::Left), &map);
    ///
    /// assert_eq!(player.position.x, 0);
    /// ```
    pub fn update(&mut self, key: Option<VirtualKeyCode>, map: &Map, camera: &mut Camera) {
        if let Some(key) = key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero()
            };

            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_new_player() {
        let position: Point = Point::new(1, 1);
        let player: Player = Player::new(position);

        assert_eq!(player.position.x, 1);
        assert_eq!(player.position.y, 1);
    }

    #[test]
    fn update_player_position() {
        let position: Point = Point::new(1, 1);
        let mut player: Player = Player::new(position);
        let map: Map = Map::new();
        let mut camera: Camera = Camera::new(position);

        player.update(Some(VirtualKeyCode::Left), &map, &mut camera);

        assert_eq!(player.position.x, 0);
        assert_eq!(player.position.y, 1);

        player.update(Some(VirtualKeyCode::Right), &map, &mut camera);

        assert_eq!(player.position.x, 1);
        assert_eq!(player.position.y, 1);

        player.update(Some(VirtualKeyCode::Up), &map, &mut camera);

        assert_eq!(player.position.x, 1);
        assert_eq!(player.position.y, 0);

        player.update(Some(VirtualKeyCode::Down), &map, &mut camera);

        assert_eq!(player.position.x, 1);
        assert_eq!(player.position.y, 1);
    }
}