use super::point::TilePoint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileLine {
    pub a: TilePoint,
    pub b: TilePoint,
}

impl TileLine {
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> Self {
        Self {
            a: TilePoint::new(x0, y0),
            b: TilePoint::new(x1, y1),
        }
    }

    pub fn length(&self) -> f32 {
        self.a.distance(&self.b)
    }
}
