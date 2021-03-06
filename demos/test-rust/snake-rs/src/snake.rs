use crate::{direction::Direction, point::Point};

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
}

impl Snake {
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        // 为什么要反向??
        let opposite = direction.opposite();
        let body = (0..length)
            .into_iter()
            .map(|i| start.transform(opposite, i))
            .collect();

        Self {
            body,
            direction,
            digesting: false,
        }
    }
    pub fn get_head_point(&self) -> Point {
        self.body.first().unwrap().clone()
    }
    pub fn get_body_points(&self) -> Vec<Point> {
        self.body.clone()
    }
    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
    }
    pub fn contains_point(&self, point: &Point) -> bool {
        self.body.contains(point)
    }
    // 滑行
    pub fn slither(&mut self) {
        self.body
            .insert(0, self.get_head_point().transform(self.direction, 1));
        if !self.digesting {
            self.body.remove(self.body.len() - 1);
        } else {
            self.digesting = false;
        }
    }
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    pub fn grow(&mut self) {
        self.digesting = true;
    }
}
