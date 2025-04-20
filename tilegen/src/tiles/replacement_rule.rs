use super::context::TileContext;
use super::replacement::Replacement;

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct ReplacementRule<T: Clone + PartialEq + Eq> {
    pub condition: fn(&TileContext<T>, &TileContext<T>) -> bool,
    pub replacements: Vec<Replacement<T>>,
    pub chance: f64,
    pub apply_count: Option<u16>,
}

#[allow(unused)]
impl<T> ReplacementRule<T>
where
    T: Default + Clone + PartialEq + Eq,
{
    pub fn to(replacement: T, condition: fn(&TileContext<T>, &TileContext<T>) -> bool) -> Self {
        ReplacementRule {
            replacements: vec![Replacement::some(0, 0, 0, replacement)],
            condition,
            ..Default::default()
        }
    }

    pub fn to_none(target: T, condition: fn(&TileContext<T>, &TileContext<T>) -> bool) -> Self {
        ReplacementRule {
            replacements: vec![Replacement::new(0, 0, 0, None)],
            condition,
            ..Default::default()
        }
    }
}

impl<T> Default for ReplacementRule<T>
where
    T: Default + Clone + PartialEq + Eq,
{
    fn default() -> Self {
        Self {
            condition: |_, _| false,
            replacements: Default::default(),
            chance: 1.0,
            apply_count: None,
        }
    }
}
