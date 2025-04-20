#[derive(Clone, Debug)]
pub struct Replacement<T: Clone + PartialEq + Eq> {
    pub(crate) delta_x: i32,
    pub(crate) delta_y: i32,
    pub(crate) delta_z: i32,
    pub(crate) replacement: Option<T>,
}

#[allow(unused)]
impl<T> Replacement<T>
where
    T: Clone + PartialEq + Eq,
{
    pub fn new(delta_x: i32, delta_y: i32, delta_z: i32, replacement: Option<T>) -> Self {
        Self {
            delta_x,
            delta_y,
            delta_z,
            replacement,
        }
    }

    pub fn some(delta_x: i32, delta_y: i32, delta_z: i32, replacement: T) -> Self {
        Self::new(delta_x, delta_y, delta_z, Some(replacement))
    }

    pub fn none(delta_x: i32, delta_y: i32, delta_z: i32) -> Self {
        Self::new(delta_x, delta_y, delta_z, None)
    }

    pub fn this(replacement: T) -> Self {
        Self::new(0, 0, 0, Some(replacement))
    }

    pub fn left(replacement: T) -> Self {
        Self::new(-1, 0, 0, Some(replacement))
    }

    pub fn right(replacement: T) -> Self {
        Self::new(1, 0, 0, Some(replacement))
    }

    pub fn up(replacement: T) -> Self {
        Self::new(0, -1, 0, Some(replacement))
    }

    pub fn down(replacement: T) -> Self {
        Self::new(0, 1, 0, Some(replacement))
    }

    pub fn top_left(replacement: T) -> Self {
        Self::new(-1, -1, 0, Some(replacement))
    }

    pub fn top_right(replacement: T) -> Self {
        Self::new(1, -1, 0, Some(replacement))
    }

    pub fn bottom_left(replacement: T) -> Self {
        Self::new(-1, 1, 0, Some(replacement))
    }

    pub fn bottom_right(replacement: T) -> Self {
        Self::new(1, 1, 0, Some(replacement))
    }

    pub fn above(replacement: T) -> Self {
        Self::new(0, 0, 1, Some(replacement))
    }

    pub fn below(replacement: T) -> Self {
        Self::new(0, 0, 1, Some(replacement))
    }
}
