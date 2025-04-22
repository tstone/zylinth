use super::point::TilePoint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileLine {
    pub from: TilePoint,
    pub to: TilePoint,
}

impl TileLine {
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> Self {
        Self {
            from: TilePoint::new(x0, y0),
            to: TilePoint::new(x1, y1),
        }
    }

    pub fn length(&self) -> f32 {
        self.from.distance(&self.to)
    }
}
