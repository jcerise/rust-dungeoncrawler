use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        Self{
            left_x: player_position.x - DISPLAY_WIDTH / 2,
            right_x: player_position.x + DISPLAY_WIDTH / 2,
            top_y: player_position.y - DISPLAY_HEIGHT / 2,
            bottom_y: player_position.y + DISPLAY_HEIGHT /2
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        self.left_x = player_position.x - DISPLAY_WIDTH / 2;
        self.right_x = player_position.x + DISPLAY_WIDTH / 2;
        self.top_y = player_position.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = player_position.y + DISPLAY_HEIGHT / 2;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_new_camera() {
        let player_position: Point = Point::new(10, 10);
        let camera: Camera = Camera::new(player_position);

        assert_eq!(camera.left_x, -10);
        assert_eq!(camera.right_x, 30);
        assert_eq!(camera.top_y, -2);
        assert_eq!(camera.bottom_y, 22);
    }

    #[test]
    fn on_player_move() {
        let player_position: Point = Point::new(10, 10);
        let mut camera: Camera = Camera::new(player_position);

        assert_eq!(camera.left_x, -10);
        assert_eq!(camera.right_x, 30);
        assert_eq!(camera.top_y, -2);
        assert_eq!(camera.bottom_y, 22);

        let new_position: Point = Point::new(9, 10);
        camera.on_player_move(new_position);

        assert_eq!(camera.left_x, -11);
        assert_eq!(camera.right_x, 29);
        assert_eq!(camera.top_y, -2);
        assert_eq!(camera.bottom_y, 22);

        let another_position: Point = Point::new(9, 9);
        camera.on_player_move(another_position);

        assert_eq!(camera.left_x, -11);
        assert_eq!(camera.right_x, 29);
        assert_eq!(camera.top_y, -3);
        assert_eq!(camera.bottom_y, 21);
    }
}