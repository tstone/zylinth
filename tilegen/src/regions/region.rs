use std::fmt::Debug;
use std::usize;

use rand::prelude::*;

use super::point::TilePoint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileRegion<T: Clone + Debug + PartialEq + Eq> {
    pub region_type: T,
    pub min: TilePoint,
    pub max: TilePoint,
}

#[allow(unused)]
impl<T> TileRegion<T>
where
    T: Clone + Debug + PartialEq + Eq,
{
    pub fn new(region_type: T, x0: usize, y0: usize, x1: usize, y1: usize) -> TileRegion<T> {
        TileRegion {
            region_type,
            min: TilePoint::new(x0, y0),
            max: TilePoint::new(x1, y1),
        }
    }

    pub fn width(&self) -> usize {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> usize {
        self.max.y - self.min.y
    }

    pub fn top_left(&self) -> TilePoint {
        TilePoint::new(self.min.x, self.min.y)
    }

    pub fn top_right(&self) -> TilePoint {
        TilePoint::new(self.max.x, self.min.y)
    }

    pub fn bottom_left(&self) -> TilePoint {
        TilePoint::new(self.min.x, self.max.y)
    }

    pub fn bottom_right(&self) -> TilePoint {
        TilePoint::new(self.max.x, self.max.y)
    }

    pub fn center(&self) -> TilePoint {
        TilePoint::new(
            self.min.x + (self.width() / 2),
            self.min.y + (self.height() / 2),
        )
    }

    pub fn left_center(&self) -> TilePoint {
        TilePoint::new(self.min.x, self.min.y + (self.height() / 2))
    }

    pub fn right_center(&self) -> TilePoint {
        TilePoint::new(self.max.x, self.min.y + (self.height() / 2))
    }

    pub fn top_center(&self) -> TilePoint {
        TilePoint::new(self.min.x + (self.width() / 2), self.min.y)
    }

    pub fn bottom_center(&self) -> TilePoint {
        TilePoint::new(self.min.x + (self.width() / 2), self.max.y)
    }

    /// Get a new region that is just the left-most slice
    pub fn left_slice(&self, width: usize) -> TileRegion<T> {
        TileRegion::new(
            self.region_type.clone(),
            self.min.x,
            self.min.y,
            self.min.x + width,
            self.max.y,
        )
    }

    /// Get a new region that is just the right-most slice
    pub fn right_slice(&self, width: usize) -> TileRegion<T> {
        TileRegion::new(
            self.region_type.clone(),
            self.max.x - width,
            self.min.y,
            self.max.x,
            self.max.y,
        )
    }

    /// Get a new region that is just the top-most slice
    pub fn top_slice(&self, height: usize) -> TileRegion<T> {
        TileRegion::new(
            self.region_type.clone(),
            self.min.x,
            self.min.y,
            self.max.x,
            self.min.y + height,
        )
    }

    /// Get a new region that is just the bottom-most slice
    pub fn bottom_slice(&self, height: usize) -> TileRegion<T> {
        TileRegion::new(
            self.region_type.clone(),
            self.min.x,
            self.max.y - height,
            self.max.x,
            self.max.y,
        )
    }

    /// Get a new region that is centered within the current region minus the given padding
    pub fn inner_slice(&self, horz_padding: usize, vert_padding: usize) -> Option<TileRegion<T>> {
        if self.width() < (horz_padding * 2) || self.height() < (vert_padding * 2) {
            return None;
        }

        Some(TileRegion::new(
            self.region_type.clone(),
            self.min.x + horz_padding,
            self.min.y + vert_padding,
            self.max.x - horz_padding,
            self.max.y - vert_padding,
        ))
    }

    /// Get a new region that is the intersection between this and another region
    pub fn intersect(&self, other: &TileRegion<T>) -> Option<Self> {
        if self.min.x > other.max.x
            || self.max.x < other.min.x
            || self.min.y > other.max.y
            || self.max.y < other.min.y
        {
            None
        } else {
            let x0 = if self.min.x < other.min.x {
                other.min.x
            } else {
                self.min.x
            };
            let y0 = if self.min.y < other.min.y {
                other.min.y
            } else {
                self.min.y
            };
            let x1 = if other.max.x < self.max.x {
                other.max.x
            } else {
                self.max.x
            };
            let y1 = if other.max.y < self.max.y {
                other.max.y
            } else {
                self.max.y
            };
            Some(TileRegion::new(self.region_type.clone(), x0, y0, x1, y1))
        }
    }

    /// Return a random point inside of this
    pub fn rnd_point(&self, rng: &mut impl Rng) -> TilePoint {
        let x = (self.min.x..=self.max.x).choose(rng).unwrap();
        let y = (self.min.y..=self.max.y).choose(rng).unwrap();
        TilePoint::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width() {
        let region = TileRegion::new("a", 0, 0, 5, 10);
        assert_eq!(region.width(), 5);
    }

    #[test]
    fn height() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        assert_eq!(region.height(), 10);
    }

    #[test]
    fn top_left() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        assert_eq!(region.top_left(), TilePoint::new(0, 0));
    }

    #[test]
    fn top_right() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        assert_eq!(region.top_right(), TilePoint::new(5, 0));
    }

    #[test]
    fn bottom_left() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        assert_eq!(region.bottom_left(), TilePoint::new(0, 10));
    }

    #[test]
    fn bottom_right() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        assert_eq!(region.bottom_right(), TilePoint::new(5, 10));
    }

    #[test]
    fn center() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        assert_eq!(region.center(), TilePoint::new(2, 5));
    }

    #[test]
    fn left_center() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        assert_eq!(region.left_center(), TilePoint::new(0, 5));
    }

    #[test]
    fn right_center() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        assert_eq!(region.right_center(), TilePoint::new(5, 5));
    }

    #[test]
    fn top_center() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        assert_eq!(region.top_center(), TilePoint::new(2, 0));
    }

    #[test]
    fn bottom_center() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        assert_eq!(region.bottom_center(), TilePoint::new(2, 10));
    }

    #[test]
    fn left_slice() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        let slice = region.left_slice(2);
        assert_eq!(slice.min, TilePoint::new(0, 0));
        assert_eq!(slice.max, TilePoint::new(2, 10));
    }

    #[test]
    fn right_slice() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        let slice = region.right_slice(2);
        assert_eq!(slice.min, TilePoint::new(3, 0));
        assert_eq!(slice.max, TilePoint::new(5, 10));
    }

    #[test]
    fn top_slice() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        let slice = region.top_slice(2);
        assert_eq!(slice.min, TilePoint::new(0, 0));
        assert_eq!(slice.max, TilePoint::new(5, 2));
    }

    #[test]
    fn bottom_slice() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        let slice = region.bottom_slice(2);
        assert_eq!(slice.min, TilePoint::new(0, 8));
        assert_eq!(slice.max, TilePoint::new(5, 10));
    }

    #[test]
    fn inner_slice() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        let slice = region.inner_slice(2, 4).unwrap();
        assert_eq!(slice.min, TilePoint::new(2, 4));
        assert_eq!(slice.max, TilePoint::new(3, 6));
    }

    #[test]
    fn inner_slice_invalid() {
        let region = TileRegion::new("b", 0, 0, 5, 10);
        let slice = region.inner_slice(3, 4);
        // regions with zero area or inverted dimensions are nothing
        assert_eq!(slice, None);
    }

    #[test]
    fn intersect() {
        let a = TileRegion::new("a", 0, 0, 5, 10);
        let b = TileRegion::new("a", 2, 2, 4, 12);
        let slice = a.intersect(&b).unwrap();
        assert_eq!(slice.min, TilePoint::new(2, 2));
        assert_eq!(slice.max, TilePoint::new(4, 10));
    }

    #[test]
    fn intersect_invalid() {
        let a = TileRegion::new("a", 0, 0, 5, 10);
        let b = TileRegion::new("a", 20, 20, 24, 32);
        let slice = a.intersect(&b);
        // non-overlaping regions are none
        assert_eq!(slice, None);
    }
}
