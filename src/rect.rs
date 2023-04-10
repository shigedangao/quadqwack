use serde::Serialize;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Serialize)]
pub struct Rect {
    x: i64,
    y: i64,
    pub w: i64,
    pub h: i64,
}

impl Rect {
    pub fn new(x: i64, y: i64, w: i64, h: i64) -> Self {
        Rect { x, y, w, h }
    }

    pub fn get_sub_dimensions(&self) -> (i64, i64) {
        (self.w / 2, self.h / 2)
    }

    pub fn get_x_and_y(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    pub fn contains_rect(&self, n_rect: &Rect) -> i64 {
        let vertical_mid_point = self.x + (self.w / 2);
        let horizontal_mid_point = self.y + (self.h / 2);

        // Check whether the object completely fit within the left quadrants
        let end_is_west = n_rect.x < vertical_mid_point && n_rect.x + n_rect.w < vertical_mid_point;
        let end_is_east = n_rect.x > vertical_mid_point;
        let start_is_north =
            n_rect.y < horizontal_mid_point && n_rect.y + n_rect.h < horizontal_mid_point;
        let start_is_south = n_rect.y > horizontal_mid_point;

        if end_is_west && start_is_north {
            1
        } else if end_is_west && start_is_south {
            2
        } else if end_is_east && start_is_north {
            0
        } else if end_is_east && start_is_south {
            3
        } else {
            -1
        }
    }
}
